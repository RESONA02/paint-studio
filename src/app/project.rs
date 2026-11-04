// project.rs — .cmc multi-page project container (binary, self-contained)
use crate::state::store::{IVec2, Layer, LayerId};
use std::io::Cursor;

use super::comic::{Balloon, ComicFrame};
use super::pages::{Binding, PageDoc};
use super::App;

const MAGIC: &[u8; 4] = b"CMC1";

impl App {
    pub(crate) fn save_project(&mut self, path: &str) -> Result<(), String> {
        self.save_current_page();
        let bytes = self.encode_project()?;
        super::canvas::ensure_parent_dir(std::path::Path::new(path));
        std::fs::write(path, bytes).map_err(|e| e.to_string())?;
        self.saved = format!("บันทึกโปรเจกต์ .cmc: {path}");
        Ok(())
    }

    pub(crate) fn load_project(&mut self, path: &str) -> Result<(), String> {
        let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
        self.decode_project(&bytes)?;
        self.load_page_after_project()?;
        self.saved = format!("เปิดโปรเจกต์ .cmc: {path}");
        Ok(())
    }

    fn load_page_after_project(&mut self) -> Result<(), String> {
        if self.pages.is_empty() {
            return Err("โปรเจกต์ไม่มีหน้า".into());
        }
        self.page_index = 0;
        self.page_marks.clear();
        self.load_current_page();
        Ok(())
    }

    fn encode_project(&self) -> Result<Vec<u8>, String> {
        let mut out = Vec::new();
        out.extend_from_slice(MAGIC);
        push_u32(&mut out, self.pages.len() as u32);
        push_u32(
            &mut out,
            match self.binding {
                Binding::LeftToRight => 0,
                Binding::RightToLeft => 1,
            },
        );
        for page in &self.pages {
            encode_page(&mut out, page)?;
        }
        Ok(out)
    }

    fn decode_project(&mut self, bytes: &[u8]) -> Result<(), String> {
        let mut reader = Reader::new(bytes);
        if reader.raw(4)? != MAGIC {
            return Err("ไฟล์ .cmc ไม่ถูกต้อง".into());
        }
        let count = reader.u32()? as usize;
        self.binding = if reader.u32()? == 0 {
            Binding::LeftToRight
        } else {
            Binding::RightToLeft
        };
        let mut pages = Vec::with_capacity(count);
        for _ in 0..count {
            pages.push(decode_page(&mut reader)?);
        }
        if pages.is_empty() {
            return Err("ไฟล์ .cmc ไม่มีหน้า".into());
        }
        self.pages = pages;
        Ok(())
    }
}

impl App {
    // (project I/O lives in this module)
}

fn encode_page(out: &mut Vec<u8>, page: &PageDoc) -> Result<(), String> {
    push_str(out, &page.name);
    push_u32(out, page.w as u32);
    push_u32(out, page.h as u32);
    push_u32(out, page.folio);
    out.push(u8::from(page.spread));
    push_str(out, &page.script);
    encode_comic(out, page);
    push_u32(out, page.layers.len() as u32);
    for layer in &page.layers {
        encode_layer(out, layer)?;
    }
    Ok(())
}

fn encode_comic(out: &mut Vec<u8>, page: &PageDoc) {
    push_u32(out, page.frames.len() as u32);
    for frame in &page.frames {
        for value in [frame.min.x, frame.min.y, frame.max.x, frame.max.y] {
            push_u32(out, value as u32);
        }
    }
    push_u32(out, page.balloons.len() as u32);
    for balloon in &page.balloons {
        for value in [
            balloon.center.x,
            balloon.center.y,
            balloon.rx,
            balloon.ry,
            balloon.tail.x,
            balloon.tail.y,
        ] {
            push_u32(out, value as u32);
        }
        push_str(out, &balloon.text);
    }
}

fn encode_layer(out: &mut Vec<u8>, layer: &Layer) -> Result<(), String> {
    push_u32(out, layer.id.0 as u32);
    push_str(out, &layer.name);
    out.push(u8::from(layer.visible));
    out.push(u8::from(layer.locked));
    out.push(layer.opacity_percent);
    push_str(out, &layer.blend_mode);
    out.push(u8::from(layer.is_paper));
    let png = layer_png(layer)?;
    push_u32(out, png.len() as u32);
    out.extend_from_slice(&png);
    Ok(())
}

fn layer_png(layer: &Layer) -> Result<Vec<u8>, String> {
    let height = layer.pixels.len();
    let width = layer.pixels.first().map_or(0, Vec::len);
    let mut image = image::RgbaImage::new(width as u32, height as u32);
    for (y, row) in layer.pixels.iter().enumerate() {
        for (x, p) in row.iter().enumerate() {
            image.put_pixel(x as u32, y as u32, image::Rgba([p.0, p.1, p.2, p.3]));
        }
    }
    let mut png = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    Ok(png)
}

fn decode_page(reader: &mut Reader<'_>) -> Result<PageDoc, String> {
    let name = reader.string()?;
    let w = reader.u32()? as usize;
    let h = reader.u32()? as usize;
    let folio = reader.u32()?;
    let spread = reader.u8()? != 0;
    let script = reader.string()?;
    let (frames, balloons) = decode_comic(reader)?;
    let count = reader.u32()? as usize;
    let mut layers = Vec::with_capacity(count);
    for _ in 0..count {
        layers.push(decode_layer(reader)?);
    }
    let mut page = PageDoc::new_page(&name, w, h, layers);
    page.folio = folio;
    page.spread = spread;
    page.script = script;
    page.frames = frames;
    page.balloons = balloons;
    Ok(page)
}

fn decode_comic(reader: &mut Reader<'_>) -> Result<(Vec<ComicFrame>, Vec<Balloon>), String> {
    let frame_count = reader.u32()? as usize;
    let mut frames = Vec::with_capacity(frame_count);
    for _ in 0..frame_count {
        let min = IVec2::new(reader.i32()?, reader.i32()?);
        let max = IVec2::new(reader.i32()?, reader.i32()?);
        frames.push(ComicFrame { min, max });
    }
    let balloon_count = reader.u32()? as usize;
    let mut balloons = Vec::with_capacity(balloon_count);
    for _ in 0..balloon_count {
        let center = IVec2::new(reader.i32()?, reader.i32()?);
        let rx = reader.i32()?;
        let ry = reader.i32()?;
        let tail = IVec2::new(reader.i32()?, reader.i32()?);
        let text = reader.string()?;
        balloons.push(Balloon {
            center,
            rx,
            ry,
            tail,
            text,
        });
    }
    Ok((frames, balloons))
}

fn decode_layer(reader: &mut Reader<'_>) -> Result<Layer, String> {
    let id = reader.u32()? as usize;
    let name = reader.string()?;
    let visible = reader.u8()? != 0;
    let locked = reader.u8()? != 0;
    let opacity = reader.u8()?;
    let blend = reader.string()?;
    let is_paper = reader.u8()? != 0;
    let blob = reader.blob()?;
    let decoded = image::load_from_memory(blob)
        .map_err(|e| e.to_string())?
        .to_rgba8();
    let (w, h) = decoded.dimensions();
    let mut layer = Layer::new(LayerId(id), name, w as usize, h as usize, is_paper);
    layer.visible = visible;
    layer.locked = locked;
    layer.opacity_percent = opacity;
    layer.blend_mode = blend;
    for (x, y, p) in decoded.enumerate_pixels() {
        layer.pixels[y as usize][x as usize] = (p[0], p[1], p[2], p[3]);
    }
    Ok(layer)
}

fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn push_str(out: &mut Vec<u8>, text: &str) {
    push_u32(out, text.len() as u32);
    out.extend_from_slice(text.as_bytes());
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn raw(&mut self, len: usize) -> Result<&'a [u8], String> {
        let end = self.pos.checked_add(len).ok_or("ไฟล์เสียหาย")?;
        if end > self.data.len() {
            return Err("ไฟล์ .cmc ถูกตัดขาด".into());
        }
        let slice = &self.data[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.raw(1)?[0])
    }

    fn u32(&mut self) -> Result<u32, String> {
        let bytes = self.raw(4)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn i32(&mut self) -> Result<isize, String> {
        Ok(self.u32()? as i32 as isize)
    }

    fn string(&mut self) -> Result<String, String> {
        let len = self.u32()? as usize;
        let bytes = self.raw(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| "ข้อความในไฟล์ไม่ใช่ UTF-8".into())
    }

    fn blob(&mut self) -> Result<&'a [u8], String> {
        let len = self.u32()? as usize;
        self.raw(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::pages::build_page_layers;

    fn tiny_app() -> App {
        let mut app = App::new(None);
        app.w = 16;
        app.h = 12;
        app.layers = build_page_layers(16, 12);
        app.layers[1].pixels[2][3] = (10, 20, 30, 255);
        app.pages = vec![PageDoc::new_page("หน้า 1", 16, 12, app.layers.clone())];
        app.pages[0].script = "บททดสอบ".into();
        app
    }

    #[test]
    fn project_roundtrip_keeps_pages_layers_and_script() {
        let mut app = tiny_app();
        app.add_page();
        app.pages[1].script = "หน้าที่สอง".into();
        let bytes = app.encode_project().expect("encode");
        let mut loaded = App::new(None);
        loaded.decode_project(&bytes).expect("decode");
        assert_eq!(loaded.pages.len(), 2);
        assert_eq!(loaded.pages[0].script, "บททดสอบ");
        assert_eq!(loaded.pages[1].script, "หน้าที่สอง");
        assert_eq!(loaded.pages[1].w, 16);
        let px = loaded.pages[0].layers[1].pixels[2][3];
        assert_eq!(px, (10, 20, 30, 255));
    }

    #[test]
    fn corrupt_project_is_rejected() {
        let mut app = App::new(None);
        let error = app.decode_project(b"nope");
        assert!(error.is_err());
    }
}
