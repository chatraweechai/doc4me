//! The poster frame of an online video Doc4Me inserts (Insert › Online Video): a dark 16:9 card
//! with a play button, drawn here because Doc4Me doesn't fetch the video site's own thumbnail.

/// A PNG `w` × `h` pixels (clamped to 160–1920 wide, 90–1080 high): a dark card with a round
/// play button in the middle.
pub fn video_poster(w: u32, h: u32) -> Vec<u8> {
    let (w, h) = (w.clamp(160, 1920), h.clamp(90, 1080));
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let r = w.min(h) as f32 * 0.16;
    // The triangle, pointing right, a little right of centre so it looks centred.
    let (tl, tr) = (cx - r * 0.38, cx + r * 0.52);
    let th = r * 0.52;
    // How much of the pixel at (x, y) is covered, 4 × 4 samples.
    let cover = |x: u32, y: u32, inside: &dyn Fn(f32, f32) -> bool| {
        let mut n = 0;
        for sy in 0..4 {
            for sx in 0..4 {
                if inside(x as f32 + (sx as f32 + 0.5) / 4.0, y as f32 + (sy as f32 + 0.5) / 4.0) {
                    n += 1;
                }
            }
        }
        n as f32 / 16.0
    };
    let circle = |x: f32, y: f32| (x - cx).powi(2) + (y - cy).powi(2) <= r * r;
    let triangle = |x: f32, y: f32| x >= tl && x <= tr && (y - cy).abs() <= th * (tr - x) / (tr - tl);
    let mix = |a: [u8; 3], b: [u8; 3], k: f32| [0, 1, 2].map(|i| (a[i] as f32 + (b[i] as f32 - a[i] as f32) * k).round() as u8);
    let (bg, button, mark) = ([0x1f, 0x1f, 0x1f], [0xe8, 0xe8, 0xe8], [0x1f, 0x1f, 0x1f]);
    let mut img = image::RgbImage::new(w, h);
    for (x, y, px) in img.enumerate_pixels_mut() {
        // Only the button's square needs sampling.
        let near = (x as f32 - cx).abs() <= r + 1.0 && (y as f32 - cy).abs() <= r + 1.0;
        let c = if near { mix(mix(bg, button, cover(x, y, &circle)), mark, cover(x, y, &triangle)) } else { bg };
        *px = image::Rgb(c);
    }
    let mut png = Vec::new();
    match img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png) {
        Ok(()) => png,
        Err(e) => {
            log::warn!("video poster: {e}");
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_poster_is_a_png_of_the_player_size_with_a_button() {
        let png = super::video_poster(640, 360);
        assert_eq!(crate::image_size(&png), Some((640, 360)));
        let img = image::load_from_memory(&png).unwrap().to_rgb8();
        assert_eq!(img.get_pixel(5, 5).0, [0x1f; 3], "dark card");
        // Inside the button, left of the triangle, the button is light.
        assert!(img.get_pixel(320 - 50, 180).0[0] > 0xc0);
        // The triangle's middle is dark again.
        assert!(img.get_pixel(322, 180).0[0] < 0x40);
        assert_eq!(crate::image_size(&super::video_poster(1, 100_000)), Some((160, 1080)), "clamped");
    }
}
