//! Home › Dictate (`Services::dictate`): speech recognition on this computer with whisper.cpp.
//!
//! On a worker thread: the speech model (OpenAI's Whisper, MIT, in whisper.cpp's quantized
//! format) is downloaded once into the settings folder's `models` and checked by SHA-256 (the
//! sums Hugging Face gives for those files); then the default microphone is opened (cpal),
//! the sound is brought to 16 kHz mono, cut into stretches of speech at pauses (an energy
//! threshold that follows the room's noise), and each stretch is transcribed and sent to the UI.
//! Thai, Chinese, Japanese, Korean, Vietnamese, Arabic and Hindi get the larger model, which
//! recognises them much better.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Sender};
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use doc4me_ui_egui::dictate::{Dictation, DictationEvent, DictationRequest};
use doc4me_ui_egui::net::FetchRequest;
use sha2::Digest;

/// The `Services::dictate` hook.
pub type Hook = Box<dyn Fn(DictationRequest) -> Result<Dictation, String>>;

/// A speech model: file name in whisper.cpp's Hugging Face repository, size and SHA-256.
struct Model {
    file: &'static str,
    size: u64,
    sha256: &'static str,
}

const BASE: Model =
    Model { file: "ggml-base-q5_1.bin", size: 59_707_625, sha256: "422f1ae452ade6f30a004d7e5c6a43195e4433bc370bf23fac9cc591f01a8898" };
const SMALL: Model =
    Model { file: "ggml-small-q5_1.bin", size: 190_085_487, sha256: "ae85e4a935d7a567bd102fe55afc16bb595bdb618e11b2fc7591bc08120411bb" };
const MODEL_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/";

/// Whisper's sample rate.
const RATE: u32 = 16_000;
/// Longest stretch transcribed at once, and the pause that ends one, in samples.
const MAX_CHUNK: usize = 25 * RATE as usize;
const PAUSE: usize = (RATE as usize) * 7 / 10;
/// Shortest speech worth transcribing.
const MIN_SPEECH: usize = (RATE as usize) * 4 / 10;

fn model_for(language: &str) -> &'static Model {
    if matches!(language, "th" | "zh" | "ja" | "ko" | "vi" | "ar" | "hi") { &SMALL } else { &BASE }
}

pub fn hook(ctx: &egui::Context) -> Hook {
    let ctx = ctx.clone();
    Box::new(move |req| {
        let models = doc4me_control_key::settings_dir().map(|d| d.join("models")).ok_or("there is no settings folder for the speech model")?;
        let (tx, rx) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let ctx = ctx.clone();
        std::thread::Builder::new()
            .name("dictate".into())
            .spawn(move || {
                let send = |e: DictationEvent| {
                    let _ = tx.send(e);
                    ctx.request_repaint();
                };
                let end = match run(&req.language, req.online, &models, &flag, &send) {
                    Ok(()) => DictationEvent::Stopped,
                    Err(e) => DictationEvent::Error(e),
                };
                send(end);
            })
            .map_err(|e| format!("couldn't start dictation: {e}"))?;
        Ok(Dictation { events: rx, stop })
    })
}

/// SHA-256 of a file, as lower-case hex.
fn file_sha256(path: &Path) -> Result<String, String> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut h = sha2::Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

/// The model's file, downloaded and checked first if it isn't there yet.
fn ensure_model(model: &Model, dir: &Path, online: bool, stop: &AtomicBool, send: &dyn Fn(DictationEvent)) -> Result<PathBuf, String> {
    let path = dir.join(model.file);
    if std::fs::metadata(&path).is_ok_and(|m| m.len() == model.size) {
        return Ok(path);
    }
    if !online {
        return Err("the speech model isn't on this computer and online services are turned off (File › Options › Trust Center)".into());
    }
    std::fs::create_dir_all(dir).map_err(|e| format!("couldn't make {}: {e}", dir.display()))?;
    let part = dir.join(format!("{}.part", model.file));
    let progress = Arc::new(AtomicU64::new(0));
    let mut req = FetchRequest::get(format!("{MODEL_URL}{}", model.file), model.size);
    req.to_file = Some(part.clone());
    req.progress = Some(progress.clone());
    let (tx, rx) = mpsc::channel();
    std::thread::Builder::new()
        .name("dictate-download".into())
        .spawn(move || {
            let _ = tx.send(crate::fetch::fetch(&req));
        })
        .map_err(|e| e.to_string())?;
    let answer = loop {
        match rx.recv_timeout(Duration::from_millis(300)) {
            Ok(a) => break a,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                send(DictationEvent::Downloading { done: progress.load(Ordering::Relaxed), total: model.size });
                if stop.load(Ordering::Relaxed) {
                    // The download finishes in the background; its partial file is replaced next time.
                    return Err("stopped while the speech model was downloading".into());
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break Err("the download stopped".into()),
        }
    };
    let checked = answer.and_then(|_| file_sha256(&part)).and_then(|sum| {
        if sum == model.sha256 { Ok(()) } else { Err("the downloaded speech model is damaged (SHA-256 doesn't match); try again".into()) }
    });
    if let Err(e) = checked {
        let _ = std::fs::remove_file(&part);
        return Err(format!("couldn't get the speech model: {e}"));
    }
    std::fs::rename(&part, &path).map_err(|e| format!("couldn't keep the speech model: {e}"))?;
    Ok(path)
}

/// Mono samples at `from` Hz brought to 16 kHz (linear interpolation; speech has little above
/// 8 kHz, so no further filtering).
pub fn resample(input: &[f32], from: u32) -> Vec<f32> {
    if from == RATE || input.is_empty() || from == 0 {
        return input.to_vec();
    }
    let ratio = from as f64 / RATE as f64;
    let n = (input.len() as f64 / ratio).floor() as usize;
    (0..n)
        .map(|i| {
            let x = i as f64 * ratio;
            let k = x.floor() as usize;
            let t = (x - k as f64) as f32;
            let a = input.get(k).copied().unwrap_or(0.0);
            let b = input.get(k + 1).copied().unwrap_or(a);
            a + (b - a) * t
        })
        .collect()
}

/// Cuts the sound into stretches of speech: speech starts when a 30 ms frame is well above the
/// noise floor, and ends after a pause (or at the longest stretch).
#[derive(Default)]
pub struct Cutter {
    buf: Vec<f32>,
    pending: Vec<f32>,
    noise: Option<f32>,
    speech: usize,
    quiet: usize,
}

const FRAME: usize = (RATE as usize) * 3 / 100;

impl Cutter {
    /// Add 16 kHz samples; the stretches of speech that ended, ready to transcribe.
    pub fn push(&mut self, samples: &[f32]) -> Vec<Vec<f32>> {
        self.pending.extend_from_slice(samples);
        let mut out = Vec::new();
        while self.pending.len() >= FRAME {
            let frame: Vec<f32> = self.pending.drain(..FRAME).collect();
            let rms = (frame.iter().map(|s| s * s).sum::<f32>() / FRAME as f32).sqrt();
            // The noise floor follows the quietest frames, slowly.
            let noise = self.noise.get_or_insert(rms.max(1e-4));
            if rms < *noise {
                *noise = rms.max(1e-4);
            } else {
                *noise = (*noise * 0.999 + rms * 0.001).max(1e-4);
            }
            let loud = rms > (*noise * 4.0).max(0.008);
            if loud {
                self.speech += FRAME;
                self.quiet = 0;
            } else if !self.buf.is_empty() {
                self.quiet += FRAME;
            }
            if loud || !self.buf.is_empty() {
                self.buf.extend_from_slice(&frame);
            }
            let ended = self.quiet >= PAUSE || self.buf.len() >= MAX_CHUNK;
            if ended && let Some(chunk) = self.take() {
                out.push(chunk);
            }
        }
        out
    }

    /// What is left, when dictation stops.
    pub fn take(&mut self) -> Option<Vec<f32>> {
        let enough = self.speech >= MIN_SPEECH;
        let chunk = std::mem::take(&mut self.buf);
        self.speech = 0;
        self.quiet = 0;
        enough.then_some(chunk)
    }
}

/// The text of a transcription without what Whisper writes for non-speech (`[BLANK_AUDIO]`,
/// `(music)`, `*cough*`).
pub fn clean(text: &str) -> String {
    let mut out = String::new();
    let (mut depth, mut starred) = (0u32, false);
    for c in text.chars() {
        match c {
            '[' | '(' => depth += 1,
            ']' | ')' if depth > 0 => depth -= 1,
            '*' => starred = !starred,
            _ if depth > 0 || starred => {}
            _ => out.push(c),
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn run(language: &str, online: bool, models: &Path, stop: &AtomicBool, send: &dyn Fn(DictationEvent)) -> Result<(), String> {
    let model = model_for(language);
    let path = ensure_model(model, models, online, stop, send)?;
    send(DictationEvent::Loading);
    whisper_rs::install_logging_hooks();
    let ctx = whisper_rs::WhisperContext::new_with_params(&path, whisper_rs::WhisperContextParameters::default())
        .map_err(|e| format!("couldn't load the speech model: {e}"))?;
    let mut state = ctx.create_state().map_err(|e| format!("couldn't start speech recognition: {e}"))?;
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2).clamp(1, 8) as i32;
    let transcribe = |state: &mut whisper_rs::WhisperState, audio: &[f32]| -> Result<String, String> {
        let mut params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(language));
        params.set_n_threads(threads);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_print_timestamps(false);
        params.set_no_context(true);
        params.set_suppress_blank(true);
        state.full(params, audio).map_err(|e| format!("speech recognition failed: {e}"))?;
        let mut text = String::new();
        for i in 0..state.full_n_segments() {
            if let Some(seg) = state.get_segment(i)
                && let Ok(t) = seg.to_str_lossy()
            {
                text.push_str(&t);
            }
        }
        Ok(clean(&text))
    };
    // The microphone; its stream stays on this thread (it isn't Send everywhere).
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or("no microphone found")?;
    let supported = device.default_input_config().map_err(|e| format!("the microphone can't be used: {e}"))?;
    let channels = usize::from(supported.channels()).max(1);
    let rate = supported.sample_rate();
    let (audio_tx, audio_rx) = mpsc::channel::<Vec<f32>>();
    let stream = open_stream(&device, &supported, channels, audio_tx)?;
    stream.play().map_err(|e| format!("the microphone can't be used: {e}"))?;
    send(DictationEvent::Listening);
    let mut cutter = Cutter::default();
    while !stop.load(Ordering::Relaxed) {
        match audio_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(samples) => {
                for chunk in cutter.push(&resample(&samples, rate)) {
                    let text = transcribe(&mut state, &chunk)?;
                    if !text.is_empty() {
                        send(DictationEvent::Text(text));
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => return Err("the microphone stopped".into()),
        }
    }
    drop(stream);
    if let Some(chunk) = cutter.take() {
        let text = transcribe(&mut state, &chunk)?;
        if !text.is_empty() {
            send(DictationEvent::Text(text));
        }
    }
    Ok(())
}

/// An input stream that sends mono `f32` samples, whatever the device's format.
fn open_stream(
    device: &cpal::Device,
    supported: &cpal::SupportedStreamConfig,
    channels: usize,
    tx: Sender<Vec<f32>>,
) -> Result<cpal::Stream, String> {
    use cpal::{FromSample, Sample, SampleFormat, SizedSample};
    fn build<T>(device: &cpal::Device, config: cpal::StreamConfig, channels: usize, tx: Sender<Vec<f32>>) -> Result<cpal::Stream, cpal::Error>
    where
        T: SizedSample + Send + 'static,
        f32: FromSample<T>,
    {
        device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                let mono: Vec<f32> =
                    data.chunks(channels).map(|frame| frame.iter().map(|s| f32::from_sample(*s)).sum::<f32>() / channels as f32).collect();
                let _ = tx.send(mono);
            },
            |e| log::warn!("microphone: {e}"),
            None,
        )
    }
    let config = supported.config();
    let r = match supported.sample_format() {
        SampleFormat::F32 => build::<f32>(device, config, channels, tx),
        SampleFormat::I16 => build::<i16>(device, config, channels, tx),
        SampleFormat::U16 => build::<u16>(device, config, channels, tx),
        SampleFormat::I32 => build::<i32>(device, config, channels, tx),
        SampleFormat::U8 => build::<u8>(device, config, channels, tx),
        other => return Err(format!("the microphone's sample format ({other}) isn't supported")),
    };
    r.map_err(|e| format!("the microphone can't be used: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sound_is_resampled_and_cut_at_pauses() {
        assert_eq!(resample(&[0.0, 1.0, 0.0, -1.0], 16_000), vec![0.0, 1.0, 0.0, -1.0]);
        let r = resample(&vec![0.5; 48_000], 48_000);
        assert_eq!(r.len(), 16_000);
        // One second of quiet, a second of "speech", a second of quiet, a click.
        let tone = |n: usize, amp: f32| (0..n).map(|i| amp * ((i as f32) * 0.3).sin()).collect::<Vec<f32>>();
        let mut cut = Cutter::default();
        let mut chunks = cut.push(&tone(16_000, 0.001));
        chunks.extend(cut.push(&tone(16_000, 0.3)));
        chunks.extend(cut.push(&tone(16_000, 0.001)));
        assert_eq!(chunks.len(), 1, "one stretch of speech");
        assert!(chunks[0].len() >= 16_000 && chunks[0].len() < 16_000 + 16_000, "{}", chunks[0].len());
        chunks = cut.push(&tone(800, 0.3));
        chunks.extend(cut.push(&tone(16_000, 0.001)));
        assert!(chunks.is_empty(), "too short to be speech");
        assert!(cut.take().is_none());
    }

    /// The real model and recognition, only with `DOC4ME_WHISPER_TEST=<a 16-bit mono WAV of
    /// "Hello world…">` (downloads the 60 MB model into a temporary folder; CI doesn't).
    #[test]
    fn real_model_downloads_checks_and_recognises_speech() {
        let Ok(wav) = std::env::var("DOC4ME_WHISPER_TEST") else { return };
        let bytes = std::fs::read(&wav).unwrap();
        // A plain PCM WAV: 44-byte header, rate at 24, samples after "data".
        let rate = u32::from_le_bytes(bytes[24..28].try_into().unwrap());
        let data = bytes.windows(4).position(|w| w == b"data").unwrap() + 8;
        let samples: Vec<f32> = bytes[data..].chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0).collect();
        let dir = std::env::temp_dir().join("doc4me-whisper-test");
        let stop = AtomicBool::new(false);
        let path = ensure_model(&BASE, &dir, true, &stop, &|_| {}).unwrap();
        assert_eq!(file_sha256(&path).unwrap(), BASE.sha256);
        let ctx = whisper_rs::WhisperContext::new_with_params(&path, whisper_rs::WhisperContextParameters::default()).unwrap();
        let mut state = ctx.create_state().unwrap();
        let audio = resample(&samples, rate);
        let mut params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        state.full(params, &audio).unwrap();
        let text: String =
            (0..state.full_n_segments()).filter_map(|i| state.get_segment(i)).filter_map(|s| s.to_str_lossy().ok().map(|t| t.into_owned())).collect();
        let text = clean(&text).to_lowercase();
        assert!(text.contains("hello") && text.contains("test"), "{text}");
        // The cutter finds the speech in it, padded with quiet.
        let mut cut = Cutter::default();
        let mut padded = vec![0.0; 16_000];
        padded.extend_from_slice(&audio);
        padded.extend(vec![0.0; 16_000]);
        assert_eq!(cut.push(&padded).len(), 1);
    }

    #[test]
    fn non_speech_markers_are_dropped() {
        assert_eq!(clean(" [BLANK_AUDIO] Hello (music) world *cough* "), "Hello world");
        assert_eq!(clean("สวัสดีครับ"), "สวัสดีครับ");
        assert_eq!(model_for("th").file, "ggml-small-q5_1.bin");
        assert_eq!(model_for("en").file, "ggml-base-q5_1.bin");
    }

    /// File › Options › Trust Center off: no download, not even a folder for one.
    #[test]
    fn no_download_with_online_services_off() {
        let dir = std::env::temp_dir().join(format!("doc4me-no-download-{}", std::process::id()));
        let e = ensure_model(&BASE, &dir, false, &AtomicBool::new(false), &|_| panic!("nothing is downloaded")).unwrap_err();
        assert!(e.contains("Trust Center"), "{e}");
        assert!(!dir.exists());
    }
}
