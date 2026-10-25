//! Insert › 3D Models: read a 3D model file and draw it as a picture.
//!
//! Word keeps an inserted model in the document (`am3d:model3d`, a glTF part) and shows a picture
//! of it to programs that don't know 3D models. Doc4Me reads three common formats — glTF binary
//! (`.glb`: the meshes of the default scene with their node transforms and base colours), STL
//! (binary and text) and Wavefront OBJ — and inserts the picture: a shaded view from the front,
//! a little from the right and above, on a transparent background. The file comes from anywhere,
//! so every count and offset is checked before it is used, and models are limited in size.

use serde_json::Value;

/// Largest model file read.
pub const MAX_BYTES: usize = 200 << 20;
/// Most triangles drawn.
pub const MAX_TRIANGLES: usize = 4_000_000;
/// The file extensions Insert › 3D Models offers.
pub const EXTENSIONS: &[&str] = &["glb", "stl", "obj"];

type V3 = [f32; 3];

/// A model's triangles, each with its colour.
#[derive(Debug, Default)]
pub struct Mesh {
    pub triangles: Vec<[V3; 3]>,
    pub colors: Vec<[u8; 3]>,
}

/// The colour of models without one (STL, OBJ): a mid grey-blue.
const DEFAULT_COLOR: [u8; 3] = [0x8a, 0x9b, 0xb0];

impl Mesh {
    fn push(&mut self, t: [V3; 3], color: [u8; 3]) -> Result<(), String> {
        if self.triangles.len() >= MAX_TRIANGLES {
            return Err(format!("the model has more than {MAX_TRIANGLES} triangles"));
        }
        if t.iter().flatten().all(|v| v.is_finite()) {
            self.triangles.push(t);
            self.colors.push(color);
        }
        Ok(())
    }
}

/// Read a model; `ext` (lower case, no dot) says which format when the bytes don't.
pub fn load(bytes: &[u8], ext: Option<&str>) -> Result<Mesh, String> {
    if bytes.len() > MAX_BYTES {
        return Err("the model is larger than 200 MB".into());
    }
    let mesh = if bytes.starts_with(b"glTF") {
        glb(bytes)?
    } else if ext == Some("obj") {
        obj(bytes)?
    } else if ext == Some("stl") || binary_stl_len(bytes) || bytes.starts_with(b"solid") {
        stl(bytes)?
    } else if ext == Some("glb") || ext == Some("gltf") {
        return Err("only binary glTF (.glb) models can be inserted".into());
    } else {
        return Err("not a 3D model Doc4Me can read (glTF binary .glb, STL or OBJ)".into());
    };
    if mesh.triangles.is_empty() {
        return Err("the model has no triangles to show".into());
    }
    Ok(mesh)
}

fn binary_stl_len(b: &[u8]) -> bool {
    b.len() >= 84 && b.get(80..84).and_then(|n| n.try_into().ok()).map(u32::from_le_bytes).is_some_and(|n| 84 + 50 * n as u64 == b.len() as u64)
}

fn f32_at(b: &[u8], at: usize) -> Option<f32> {
    b.get(at..at.checked_add(4)?).and_then(|s| s.try_into().ok()).map(f32::from_le_bytes)
}

fn stl(b: &[u8]) -> Result<Mesh, String> {
    let mut mesh = Mesh::default();
    if binary_stl_len(b) {
        let n = (b.len() - 84) / 50;
        for i in 0..n {
            let at = 84 + i * 50 + 12;
            let v = |k: usize| -> V3 { [0, 1, 2].map(|c| f32_at(b, at + k * 12 + c * 4).unwrap_or(f32::NAN)) };
            // STL is z-up; the views here are y-up.
            let up = |p: V3| [p[0], p[2], -p[1]];
            mesh.push([up(v(0)), up(v(1)), up(v(2))], DEFAULT_COLOR)?;
        }
        return Ok(mesh);
    }
    let text = String::from_utf8_lossy(b);
    let mut corners = Vec::with_capacity(3);
    for line in text.lines() {
        let mut words = line.split_whitespace();
        if words.next() != Some("vertex") {
            continue;
        }
        let p: Vec<f32> = words.take(3).map(|w| w.parse().unwrap_or(f32::NAN)).collect();
        if let [x, y, z] = p[..] {
            corners.push([x, z, -y]);
        }
        if corners.len() == 3 {
            mesh.push([corners[0], corners[1], corners[2]], DEFAULT_COLOR)?;
            corners.clear();
        }
    }
    Ok(mesh)
}

fn obj(b: &[u8]) -> Result<Mesh, String> {
    let text = String::from_utf8_lossy(b);
    let mut mesh = Mesh::default();
    let mut vertices: Vec<V3> = Vec::new();
    for line in text.lines() {
        let mut words = line.split_whitespace();
        match words.next() {
            Some("v") => {
                let p: Vec<f32> = words.take(3).map(|w| w.parse().unwrap_or(f32::NAN)).collect();
                if vertices.len() >= MAX_TRIANGLES * 3 {
                    return Err("the model has too many vertices".into());
                }
                vertices.push(match p[..] {
                    [x, y, z] => [x, y, z],
                    _ => [f32::NAN; 3],
                });
            }
            Some("f") => {
                // `v`, `v/vt`, `v//vn` or `v/vt/vn`; negative indices count back from the last vertex.
                let ids: Vec<Option<usize>> = words
                    .map(|w| {
                        let i: i64 = w.split('/').next()?.parse().ok()?;
                        let n = vertices.len() as i64;
                        let i = if i < 0 { n + i } else { i - 1 };
                        (0..n).contains(&i).then_some(i as usize)
                    })
                    .collect();
                let Some(ids) = ids.into_iter().collect::<Option<Vec<usize>>>() else { continue };
                // A polygon is drawn as a fan of triangles.
                for k in 1..ids.len().saturating_sub(1) {
                    mesh.push([vertices[ids[0]], vertices[ids[k]], vertices[ids[k + 1]]], DEFAULT_COLOR)?;
                }
            }
            _ => {}
        }
    }
    Ok(mesh)
}

/// The JSON and binary chunks of a GLB file (glTF 2.0 §4.4).
fn glb(b: &[u8]) -> Result<Mesh, String> {
    let u32_at = |at: usize| b.get(at..at + 4).and_then(|s| s.try_into().ok()).map(u32::from_le_bytes);
    if u32_at(4) != Some(2) {
        return Err("only glTF 2.0 models can be inserted".into());
    }
    let mut json = None;
    let mut bin: &[u8] = &[];
    let mut at = 12usize;
    while let (Some(len), Some(kind)) = (u32_at(at), u32_at(at + 4)) {
        let start = at + 8;
        let Some(chunk) = start.checked_add(len as usize).and_then(|end| b.get(start..end)) else { break };
        match kind {
            0x4E4F534A if json.is_none() => json = Some(chunk),
            0x004E4942 if bin.is_empty() => bin = chunk,
            _ => {}
        }
        at = start + len as usize;
        // Chunks are 4-byte aligned.
        at = at.div_ceil(4) * 4;
    }
    let json: Value = serde_json::from_slice(json.ok_or("the glTF file has no JSON chunk")?).map_err(|e| format!("the glTF file is damaged: {e}"))?;
    let g = Gltf { json: &json, bin, work: std::cell::Cell::new(0) };
    let mut mesh = Mesh::default();
    let scene = json.get("scene").and_then(Value::as_u64).unwrap_or(0) as usize;
    let roots: Vec<usize> = match json.pointer(&format!("/scenes/{scene}/nodes")).and_then(Value::as_array) {
        Some(nodes) => nodes.iter().filter_map(|n| n.as_u64()).map(|n| n as usize).collect(),
        None => {
            // No scene: every mesh as it is.
            let n = json.get("meshes").and_then(Value::as_array).map(Vec::len).unwrap_or(0);
            for m in 0..n {
                g.mesh(m, &IDENTITY, &mut mesh)?;
            }
            return Ok(mesh);
        }
    };
    let mut visits = 0usize;
    for root in roots {
        g.node(root, &IDENTITY, &mut Vec::new(), &mut visits, &mut mesh)?;
    }
    Ok(mesh)
}

/// A column-major 4 × 4 matrix (as glTF writes them).
type M4 = [f32; 16];
const IDENTITY: M4 = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];

fn mul(a: &M4, b: &M4) -> M4 {
    let mut m = [0.0; 16];
    for c in 0..4 {
        for r in 0..4 {
            m[c * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[c * 4 + k]).sum();
        }
    }
    m
}

fn apply(m: &M4, p: V3) -> V3 {
    [0, 1, 2].map(|r| m[r] * p[0] + m[4 + r] * p[1] + m[8 + r] * p[2] + m[12 + r])
}

struct Gltf<'a> {
    json: &'a Value,
    bin: &'a [u8],
    /// Vertices and indices read so far: a node graph that draws one big mesh many times (or
    /// a mesh of points and lines, which adds no triangles) can't make reading take forever.
    work: std::cell::Cell<usize>,
}

/// Most vertices and indices read from a glTF file, counting every time a mesh is drawn.
const MAX_WORK: usize = MAX_TRIANGLES * 6;

impl Gltf<'_> {
    /// Draw node `i` and its children; `path` holds its ancestors.
    fn node(&self, i: usize, parent: &M4, path: &mut Vec<usize>, visits: &mut usize, out: &mut Mesh) -> Result<(), String> {
        *visits += 1;
        // Node graphs are trees: a node that is its own ancestor is skipped, and a very deep or
        // very large graph is cut off.
        if path.len() > 64 || *visits > 100_000 || path.contains(&i) {
            return Ok(());
        }
        let Some(node) = self.json.pointer(&format!("/nodes/{i}")) else { return Ok(()) };
        let nums = |k: &str, n: usize| -> Option<Vec<f32>> {
            let a = node.get(k)?.as_array()?;
            (a.len() == n).then(|| a.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect())
        };
        let local: M4 = if let Some(m) = nums("matrix", 16) {
            m.try_into().unwrap_or(IDENTITY)
        } else {
            let t = nums("translation", 3).unwrap_or(vec![0.0; 3]);
            let q = nums("rotation", 4).unwrap_or(vec![0.0, 0.0, 0.0, 1.0]);
            let s = nums("scale", 3).unwrap_or(vec![1.0; 3]);
            let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
            let r = [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y + z * w),
                2.0 * (x * z - y * w),
                2.0 * (x * y - z * w),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z + x * w),
                2.0 * (x * z + y * w),
                2.0 * (y * z - x * w),
                1.0 - 2.0 * (x * x + y * y),
            ];
            [
                r[0] * s[0],
                r[1] * s[0],
                r[2] * s[0],
                0.0,
                r[3] * s[1],
                r[4] * s[1],
                r[5] * s[1],
                0.0,
                r[6] * s[2],
                r[7] * s[2],
                r[8] * s[2],
                0.0,
                t[0],
                t[1],
                t[2],
                1.0,
            ]
        };
        let world = mul(parent, &local);
        if let Some(m) = node.get("mesh").and_then(Value::as_u64) {
            self.mesh(m as usize, &world, out)?;
        }
        path.push(i);
        for child in node.get("children").and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_u64) {
            self.node(child as usize, &world, path, visits, out)?;
        }
        path.pop();
        Ok(())
    }

    fn mesh(&self, i: usize, m: &M4, out: &mut Mesh) -> Result<(), String> {
        let Some(prims) = self.json.pointer(&format!("/meshes/{i}/primitives")).and_then(Value::as_array) else { return Ok(()) };
        for prim in prims {
            if prim.pointer("/extensions/KHR_draco_mesh_compression").is_some() {
                return Err("compressed (Draco) glTF models can't be read yet".into());
            }
            let mode = prim.get("mode").and_then(Value::as_u64).unwrap_or(4);
            let Some(pos) = prim.pointer("/attributes/POSITION").and_then(Value::as_u64) else { continue };
            let Some(points) = self.floats3(pos as usize) else { continue };
            let points: Vec<V3> = points.into_iter().map(|p| apply(m, p)).collect();
            let order: Vec<usize> = match prim.get("indices").and_then(Value::as_u64) {
                Some(a) => match self.indices(a as usize) {
                    Some(ix) => ix,
                    None => continue,
                },
                None => (0..points.len()).collect(),
            };
            let work = self.work.get().saturating_add(points.len()).saturating_add(order.len());
            if work > MAX_WORK {
                return Err("the model is too large to draw".into());
            }
            self.work.set(work);
            let color = prim
                .get("material")
                .and_then(Value::as_u64)
                .and_then(|k| self.json.pointer(&format!("/materials/{k}/pbrMetallicRoughness/baseColorFactor")))
                .and_then(Value::as_array)
                .filter(|c| c.len() >= 3)
                .map(|c| [0, 1, 2].map(|k| (c[k].as_f64().unwrap_or(0.8).clamp(0.0, 1.0).powf(1.0 / 2.2) * 255.0).round() as u8))
                .unwrap_or(DEFAULT_COLOR);
            let at = |k: usize| order.get(k).and_then(|&i| points.get(i)).copied();
            let mut tri = |a: usize, b: usize, c: usize| -> Result<(), String> {
                if let (Some(a), Some(b), Some(c)) = (at(a), at(b), at(c)) {
                    out.push([a, b, c], color)?;
                }
                Ok(())
            };
            match mode {
                // Triangles, triangle strip, triangle fan (glTF 2.0 §3.7.2.1); points and lines show nothing.
                4 => {
                    for k in (0..order.len() / 3).map(|k| k * 3) {
                        tri(k, k + 1, k + 2)?;
                    }
                }
                5 => {
                    for k in 0..order.len().saturating_sub(2) {
                        if k % 2 == 0 { tri(k, k + 1, k + 2)? } else { tri(k + 1, k, k + 2)? }
                    }
                }
                6 => {
                    for k in 1..order.len().saturating_sub(1) {
                        tri(0, k, k + 1)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// An accessor's bytes in the binary chunk: (data, element count, stride, component type).
    fn view(&self, a: usize, kind: &str, elem: usize) -> Option<(&[u8], usize, usize, u64)> {
        let acc = self.json.pointer(&format!("/accessors/{a}"))?;
        if acc.get("type")?.as_str()? != kind || acc.get("sparse").is_some() {
            return None;
        }
        let component = acc.get("componentType")?.as_u64()?;
        let size = match component {
            5121 => 1,
            5123 => 2,
            5125 | 5126 => 4,
            _ => return None,
        } * elem;
        let count = acc.get("count")?.as_u64()? as usize;
        let bv = self.json.pointer(&format!("/bufferViews/{}", acc.get("bufferView")?.as_u64()?))?;
        // Only the GLB's own binary chunk (buffer 0) is read.
        if bv.get("buffer").and_then(Value::as_u64).unwrap_or(0) != 0 {
            return None;
        }
        let start = (bv.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize)
            .checked_add(acc.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize)?;
        let view_end = (bv.get("byteOffset").and_then(Value::as_u64).unwrap_or(0) as usize).checked_add(bv.get("byteLength")?.as_u64()? as usize)?;
        let stride = bv.get("byteStride").and_then(Value::as_u64).map(|s| s as usize).filter(|&s| s >= size).unwrap_or(size);
        let end = match count {
            0 => start,
            n => start.checked_add((n - 1).checked_mul(stride)?)?.checked_add(size)?,
        };
        if end > view_end || count > MAX_TRIANGLES * 3 {
            return None;
        }
        Some((self.bin.get(start..end)?, count, stride, component))
    }

    fn floats3(&self, a: usize) -> Option<Vec<V3>> {
        let (data, count, stride, component) = self.view(a, "VEC3", 3)?;
        if component != 5126 {
            return None;
        }
        Some((0..count).map(|i| [0, 1, 2].map(|c| f32_at(data, i * stride + c * 4).unwrap_or(f32::NAN))).collect())
    }

    fn indices(&self, a: usize) -> Option<Vec<usize>> {
        let (data, count, stride, component) = self.view(a, "SCALAR", 1)?;
        (0..count)
            .map(|i| {
                let at = i * stride;
                Some(match component {
                    5121 => *data.get(at)? as usize,
                    5123 => u16::from_le_bytes(data.get(at..at + 2)?.try_into().ok()?) as usize,
                    5125 => u32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?) as usize,
                    _ => return None,
                })
            })
            .collect()
    }
}

/// Draw the model `px` pixels square: turned `yaw` degrees about the vertical axis and tilted
/// `pitch` degrees towards the viewer, lit from the upper left, on a transparent background.
/// PNG bytes.
pub fn render(mesh: &Mesh, px: u32, yaw: f32, pitch: f32) -> Vec<u8> {
    let px = px.clamp(64, 2048);
    // Drawn twice as large, then scaled down: smooth edges.
    let n = px as usize * 2;
    let (sy, cy) = yaw.to_radians().sin_cos();
    let (sp, cp) = pitch.to_radians().sin_cos();
    let turn = |p: V3| -> V3 {
        let (x, z) = (p[0] * cy + p[2] * sy, -p[0] * sy + p[2] * cy);
        let (y, z) = (p[1] * cp - z * sp, p[1] * sp + z * cp);
        [x, y, z]
    };
    // Fit the bounding sphere, so every turn of the model fits the same square.
    let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
    for p in mesh.triangles.iter().flatten() {
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let centre: V3 = [0, 1, 2].map(|k| (lo[k] + hi[k]) / 2.0);
    let radius = mesh.triangles.iter().flatten().map(|p| (0..3).map(|k| (p[k] - centre[k]).powi(2)).sum::<f32>()).fold(0.0f32, f32::max).sqrt();
    let scale = if radius > 0.0 && radius.is_finite() { n as f32 * 0.46 / radius } else { 1.0 };
    let half = n as f32 / 2.0;
    let light = {
        let l = [-0.45f32, 0.6, 0.66];
        let len = l.iter().map(|v| v * v).sum::<f32>().sqrt();
        l.map(|v| v / len)
    };
    let mut depth = vec![f32::NEG_INFINITY; n * n];
    let mut color = vec![[0u8; 4]; n * n];
    for (t, base) in mesh.triangles.iter().zip(&mesh.colors) {
        let v = t.map(|p| {
            let q = turn([p[0] - centre[0], p[1] - centre[1], p[2] - centre[2]]);
            [half + q[0] * scale, half - q[1] * scale, q[2] * scale]
        });
        // Flat shading from the face's normal (either side faces the light).
        let (e1, e2) = ([0, 1, 2].map(|k| v[1][k] - v[0][k]), [0, 1, 2].map(|k| v[2][k] - v[0][k]));
        let nrm = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]];
        let len = nrm.iter().map(|x| x * x).sum::<f32>().sqrt();
        if len == 0.0 || !len.is_finite() {
            continue;
        }
        // Screen y points down: flip it back for the light.
        let lit = ((nrm[0] * light[0] - nrm[1] * light[1] + nrm[2] * light[2]) / len).abs();
        let k = 0.3 + 0.7 * lit;
        let shade = [base[0], base[1], base[2]].map(|c| (c as f32 * k).round().min(255.0) as u8);
        let area = e1[0] * e2[1] - e1[1] * e2[0];
        if area.abs() < 1e-9 {
            continue;
        }
        let min_x = v.iter().map(|p| p[0]).fold(f32::MAX, f32::min).floor().max(0.0) as usize;
        let max_x = (v.iter().map(|p| p[0]).fold(f32::MIN, f32::max).ceil() as isize).clamp(0, n as isize - 1) as usize;
        let min_y = v.iter().map(|p| p[1]).fold(f32::MAX, f32::min).floor().max(0.0) as usize;
        let max_y = (v.iter().map(|p| p[1]).fold(f32::MIN, f32::max).ceil() as isize).clamp(0, n as isize - 1) as usize;
        for y in min_y..=max_y.max(min_y) {
            for x in min_x..=max_x.max(min_x) {
                if x >= n || y >= n {
                    continue;
                }
                let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
                let w0 = ((v[1][0] - fx) * (v[2][1] - fy) - (v[1][1] - fy) * (v[2][0] - fx)) / area;
                let w1 = ((v[2][0] - fx) * (v[0][1] - fy) - (v[2][1] - fy) * (v[0][0] - fx)) / area;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let z = w0 * v[0][2] + w1 * v[1][2] + w2 * v[2][2];
                let i = y * n + x;
                if z > depth[i] {
                    depth[i] = z;
                    color[i] = [shade[0], shade[1], shade[2], 255];
                }
            }
        }
    }
    // Scale down 2 × 2 → 1, with straight (not premultiplied) alpha.
    let mut img = image::RgbaImage::new(px, px);
    for (x, y, out) in img.enumerate_pixels_mut() {
        let (mut sum, mut alpha) = ([0u32; 3], 0u32);
        for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            let c = color[(y as usize * 2 + dy) * n + x as usize * 2 + dx];
            let a = c[3] as u32;
            alpha += a;
            for k in 0..3 {
                sum[k] += c[k] as u32 * a;
            }
        }
        *out = match [0, 1, 2].map(|k| sum[k].checked_div(alpha)) {
            [Some(r), Some(g), Some(b)] => image::Rgba([r as u8, g as u8, b as u8, (alpha / 4) as u8]),
            _ => image::Rgba([0; 4]),
        };
    }
    let mut png = Vec::new();
    match img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png) {
        Ok(()) => png,
        Err(e) => {
            log::warn!("3D model picture: {e}");
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A unit cube's 12 triangles as binary STL.
    fn cube_stl() -> Vec<u8> {
        let c = |i: u8| [(i & 1) as f32, ((i >> 1) & 1) as f32, ((i >> 2) & 1) as f32];
        let faces: [[u8; 4]; 6] = [[0, 1, 3, 2], [4, 6, 7, 5], [0, 4, 5, 1], [2, 3, 7, 6], [0, 2, 6, 4], [1, 5, 7, 3]];
        let mut b = vec![0u8; 80];
        b.extend_from_slice(&12u32.to_le_bytes());
        for f in faces {
            for t in [[f[0], f[1], f[2]], [f[0], f[2], f[3]]] {
                b.extend_from_slice(&[0u8; 12]);
                for v in t {
                    for k in c(v) {
                        b.extend_from_slice(&k.to_le_bytes());
                    }
                }
                b.extend_from_slice(&[0, 0]);
            }
        }
        b
    }

    /// A one-triangle GLB with a red material and a translated node.
    fn triangle_glb() -> Vec<u8> {
        let mut bin = Vec::new();
        for v in [0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0] {
            bin.extend_from_slice(&v.to_le_bytes());
        }
        for i in [0u16, 1, 2, 0] {
            bin.extend_from_slice(&i.to_le_bytes());
        }
        let json = serde_json::json!({
            "asset": {"version": "2.0"},
            "scene": 0,
            "scenes": [{"nodes": [0]}],
            "nodes": [{"mesh": 0, "translation": [5.0, 0.0, 0.0], "children": [0]}],
            "meshes": [{"primitives": [{"attributes": {"POSITION": 0}, "indices": 1, "material": 0}]}],
            "materials": [{"pbrMetallicRoughness": {"baseColorFactor": [1.0, 0.0, 0.0, 1.0]}}],
            "buffers": [{"byteLength": bin.len()}],
            "bufferViews": [{"buffer": 0, "byteOffset": 0, "byteLength": 36}, {"buffer": 0, "byteOffset": 36, "byteLength": 6}],
            "accessors": [
                {"bufferView": 0, "componentType": 5126, "count": 3, "type": "VEC3"},
                {"bufferView": 1, "componentType": 5123, "count": 3, "type": "SCALAR"}
            ]
        });
        let mut js = serde_json::to_vec(&json).unwrap();
        while !js.len().is_multiple_of(4) {
            js.push(b' ');
        }
        let total = 12 + 8 + js.len() + 8 + bin.len();
        let mut b = Vec::new();
        b.extend_from_slice(b"glTF");
        b.extend_from_slice(&2u32.to_le_bytes());
        b.extend_from_slice(&(total as u32).to_le_bytes());
        b.extend_from_slice(&(js.len() as u32).to_le_bytes());
        b.extend_from_slice(&0x4E4F534Au32.to_le_bytes());
        b.extend_from_slice(&js);
        b.extend_from_slice(&(bin.len() as u32).to_le_bytes());
        b.extend_from_slice(&0x004E4942u32.to_le_bytes());
        b.extend_from_slice(&bin);
        b
    }

    #[test]
    fn stl_obj_and_glb_models_are_read() {
        let cube = load(&cube_stl(), None).unwrap();
        assert_eq!(cube.triangles.len(), 12);
        let text = "solid t\nfacet normal 0 0 1\nouter loop\nvertex 0 0 0\nvertex 1 0 0\nvertex 0 1 0\nendloop\nendfacet\nendsolid t\n";
        assert_eq!(load(text.as_bytes(), Some("stl")).unwrap().triangles.len(), 1);
        // A quad (two triangles) with slashes and a negative index; a bad face is skipped.
        let obj = "v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1/1/1 2//1 3 -1\nf 1 2 99\n";
        assert_eq!(load(obj.as_bytes(), Some("obj")).unwrap().triangles.len(), 2);
        // The node's translation applies, its material colours the triangle, and the node that
        // lists itself as its child doesn't loop forever.
        let glb = load(&triangle_glb(), Some("glb")).unwrap();
        assert_eq!(glb.triangles.len(), 1, "drawn once");
        assert_eq!(glb.triangles[0][0], [5.0, 0.0, 0.0]);
        assert_eq!(glb.colors[0], [255, 0, 0]);
    }

    #[test]
    fn damaged_and_unknown_files_are_refused() {
        assert!(load(b"hello", None).is_err());
        assert!(load(b"glTF\x01\0\0\0", None).is_err(), "glTF 1");
        assert!(load(b"glTF\x02\0\0\0\x14\0\0\0\xff\xff\xff\xffJSON", None).is_err(), "a chunk longer than the file");
        // Positions said to run past their buffer view: nothing is read from beyond it.
        let glb = String::from_utf8_lossy(&triangle_glb()).replacen("\"count\":3", "\"count\":9", 1).into_bytes();
        assert!(load(&glb, None).is_err(), "no triangles from out-of-range data");
        assert!(load(b"", Some("obj")).is_err(), "nothing to show");
        // A node that is its own child, listed many times, keeps drawing a mesh of lines (no
        // triangles): reading stops at the work limit instead of running on.
        let lines = String::from_utf8_lossy(&triangle_glb())
            .replacen("\"indices\":1,", "\"indices\":1,\"mode\":1,", 1)
            .replacen("\"children\":[0]", &format!("\"children\":[{}]", vec!["0"; 4000].join(",")), 1)
            .into_bytes();
        let fixed = |mut b: Vec<u8>| {
            // The JSON chunk grew: fix its length, the padding and the file length.
            let json_len = u32::from_le_bytes(b[12..16].try_into().unwrap()) as usize;
            // The binary chunk: 8 bytes of header, 36 of positions and 8 of indices.
            let grow = b.len() - (12 + 8 + json_len + 8 + 44);
            let mut js = b[20..20 + json_len + grow].to_vec();
            while !js.len().is_multiple_of(4) {
                js.push(b' ');
            }
            let bin = b.split_off(20 + json_len + grow);
            let mut out = b[..12].to_vec();
            out.extend_from_slice(&(js.len() as u32).to_le_bytes());
            out.extend_from_slice(&0x4E4F534Au32.to_le_bytes());
            out.extend_from_slice(&js);
            out.extend_from_slice(&bin);
            let total = out.len() as u32;
            out[8..12].copy_from_slice(&total.to_le_bytes());
            out
        };
        assert_eq!(load(&fixed(triangle_glb()), None).unwrap().triangles.len(), 1, "the helper keeps a file whole");
        let started = std::time::Instant::now();
        assert!(load(&fixed(lines), None).is_err(), "lines only: nothing to show");
        assert!(started.elapsed() < std::time::Duration::from_secs(20), "stopped in time");
        let huge = vec![0u8; MAX_BYTES + 1];
        assert!(load(&huge, Some("stl")).is_err(), "too large");
    }

    #[test]
    fn the_picture_shows_the_model_on_transparent_ground() {
        let cube = load(&cube_stl(), None).unwrap();
        let png = render(&cube, 200, -30.0, 20.0);
        let img = image::load_from_memory(&png).unwrap().to_rgba8();
        assert_eq!(img.dimensions(), (200, 200));
        assert_eq!(img.get_pixel(1, 1).0[3], 0, "transparent corner");
        let middle = img.get_pixel(100, 100).0;
        assert_eq!(middle[3], 255, "the cube covers the middle");
        assert!(middle[0] > 20, "shaded, not black: {middle:?}");
    }
}
