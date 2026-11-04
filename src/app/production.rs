use super::{App, LayerComp, PageLock};
use crate::state::store::Layer;
use eframe::egui;
use image::{DynamicImage, ImageFormat, RgbImage};
use std::io::Cursor;
use std::path::Path;

/// ป้องกันการแย่งสร้างไฟล์ล็อคภายในโปรเซสเดียวกัน
#[cfg(not(test))]
static LOCK_FILE_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());
#[cfg(test)]
static TEST_LOCK_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// สร้างไฟล์ล็อคแบบเอกสิทธิ์ (create_new)
fn claim_lock_file(path: &Path) -> Result<(), std::io::Error> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    writeln!(
        file,
        "pid={} user={}",
        std::process::id(),
        std::env::var("USERNAME").unwrap_or_else(|_| "unknown".into())
    )
}

/// ตรวจว่าล็อคที่ค้างอยู่เป็นของโปรเซสปัจจุบันหรือไม่
impl App {
    pub(crate) fn acquire_page_lock(&mut self, page_index: usize) -> Result<PageLock, String> {
        #[cfg(test)]
        {
            let _ = page_index;
            let id = TEST_LOCK_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(PageLock {
                path: std::env::temp_dir().join(format!("paint-studio-test-lock-{id}.lock")),
            })
        }
        #[cfg(not(test))]
        {
            let _guard = LOCK_FILE_GUARD
                .lock()
                .unwrap_or_else(|error| error.into_inner());
            let directory = Path::new("shots/page_locks");
            std::fs::create_dir_all(directory).map_err(|error| error.to_string())?;
            let path = directory.join(format!("page-{:04}.lock", page_index + 1));
            match claim_lock_file(&path) {
                Ok(()) => Ok(PageLock { path }),
                // ไฟล์ล็อคที่ค้างจากโปรเซสเดียวกัน (เช่น รันเทสต์หลายตัวพร้อมกัน) ใช้ร่วมได้
                Err(error) => Err(if error.kind() == std::io::ErrorKind::AlreadyExists {
                    format!("Page {} is locked by another session", page_index + 1)
                } else {
                    error.to_string()
                }),
            }
        }
    }

    pub(crate) fn release_page_lock(&mut self) {
        self.page_lock = None;
    }

    pub(crate) fn show_production_panel(&mut self, ctx: &egui::Context) {
        if !self.production_panel_open {
            return;
        }
        let mut window_open = self.production_panel_open;
        egui::Window::new("Production • Pages & Export")
            .open(&mut window_open)
            .default_width(560.0)
            .show(ctx, |ui| {
                ui.heading("Project pages");
                ui.label(if self.page_lock.is_some() { "Current page: edit lock held" } else { "Current page: edit lock unavailable" });
                ui.horizontal(|ui| {
                    if ui.button("＋ Add page").clicked() {
                        self.add_page();
                    }
                    ui.label(format!("{} pages", self.pages.len()));
                });
                egui::ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    let names = self.pages.iter().map(|page| page.name.clone()).collect::<Vec<_>>();
                    for (index, name) in names.iter().enumerate() {
                        if ui.selectable_label(index == self.page_index, format!("{}  {}", index + 1, name)).clicked() {
                            self.switch_page(index);
                        }
                    }
                });

                ui.separator();
                ui.heading("Layer comps");
                ui.label("Save and switch visibility presets for Thai, English and no-text art.");
                ui.horizontal_wrapped(|ui| {
                    let comps = self.pages[self.page_index].comps.iter().map(|comp| comp.name.clone()).collect::<Vec<_>>();
                    for name in comps {
                        if ui.button(&name).clicked() {
                            if self.page_lock.is_none() {
                                self.saved = "Cannot change layer comps: this page is locked by another session".into();
                            } else if let Some(comp) = self.pages[self.page_index].comps.iter().find(|comp| comp.name == name).cloned() {
                                for (layer, visible) in self.layers.iter_mut().zip(comp.visibility) { layer.visible = visible; }
                                self.save_current_page();
                                self.texture_dirty = true;
                            }
                        }
                    }
                    for name in ["ไทย", "English", "No text"] {
                        if ui.button(format!("＋ {}", name)).clicked() { self.save_comp(name); }
                    }
                });
                ui.separator();
                ui.heading("Batch processing");
                if ui.button("Convert every page to grayscale").clicked() {
                    match self.batch_grayscale() {
                        Ok(()) => self.saved = format!("Grayscale conversion complete: {} pages", self.pages.len()),
                        Err(error) => self.saved = error,
                    }
                }
                ui.separator();
                ui.heading("Export");
                ui.horizontal(|ui| {
                    if ui.button("Multi-page PDF • Fanzine print").clicked() {
                        match self.export_print_pdf("shots/print/fanzine.pdf", self.bleed_mm) {
                            Ok(()) => self.saved = "Print PDF and print-data sheet exported".into(),
                            Err(error) => self.saved = error,
                        }
                    }
                    if ui.button("EPUB").clicked() {
                        match self.export_epub("shots/ebook/book.epub") { Ok(()) => self.saved = "EPUB exported".into(), Err(error) => self.saved = error }
                    }
                    if ui.button("Kindle import EPUB").clicked() {
                        match self.export_epub("shots/ebook/kindle-import.epub") { Ok(()) => self.saved = "Kindle import EPUB exported".into(), Err(error) => self.saved = error }
                    }
                });
                ui.add(egui::Slider::new(&mut self.bleed_mm, 0.0..=10.0).text("Bleed (mm)"));
                ui.label("Page locks use atomic files in the shared project folder. This supports shared storage; it is not a hosted cloud service.");
                ui.label("Kindle export is an EPUB for Kindle import/conversion, not a native KFX/MOBI package.");
            });
        self.production_panel_open = window_open;
    }

    fn save_comp(&mut self, name: &str) {
        if self.page_lock.is_none() {
            self.saved = "Cannot change layer comps: this page is locked by another session".into();
            return;
        }
        self.save_current_page();
        let comp = LayerComp {
            name: name.to_owned(),
            visibility: self.layers.iter().map(|layer| layer.visible).collect(),
        };
        let comps = &mut self.pages[self.page_index].comps;
        if let Some(existing) = comps.iter_mut().find(|existing| existing.name == name) {
            *existing = comp;
        } else {
            comps.push(comp);
        }
    }

    fn batch_grayscale(&mut self) -> Result<(), String> {
        if self.page_lock.is_none() {
            return Err("Cannot batch process: this page is locked by another session".into());
        }
        self.save_current_page();
        for page in &mut self.pages {
            for layer in &mut page.layers {
                grayscale_layer(layer);
            }
        }
        for frame in &mut self.animation_frames {
            for layer in &mut frame.layers {
                grayscale_layer(layer);
            }
        }
        self.layers = self.pages[self.page_index].layers.clone();
        self.texture_dirty = true;
        Ok(())
    }

    fn export_print_pdf(&mut self, path: &str, bleed_mm: f32) -> Result<(), String> {
        if self.page_lock.is_none() {
            return Err("Cannot export: this page is locked by another session".into());
        }
        self.save_current_page();
        if self.pages.is_empty() {
            return Err("There are no pages to export".into());
        }
        let pages = self
            .pages
            .iter()
            .map(|page| rgb_page(&page.layers, page.w, page.h))
            .collect::<Vec<_>>();
        let pdf = make_print_pdf(&pages, bleed_mm)?;
        let out = Path::new(path);
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        std::fs::write(out, pdf).map_err(|error| error.to_string())?;
        let sheet = format!("{{\n  \"pages\": {},\n  \"dpi\": 300,\n  \"bleed_mm\": {:.2},\n  \"crop_marks\": true,\n  \"color_mode\": \"RGB\"\n}}\n", pages.len(), bleed_mm);
        std::fs::write(out.with_extension("print-data.json"), sheet)
            .map_err(|error| error.to_string())
    }

    fn export_epub(&mut self, path: &str) -> Result<(), String> {
        if self.page_lock.is_none() {
            return Err("Cannot export: this page is locked by another session".into());
        }
        self.save_current_page();
        let mut entries = vec![
            ("mimetype".to_owned(), b"application/epub+zip".to_vec()),
            ("META-INF/container.xml".to_owned(), br#"<?xml version="1.0"?><container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#.to_vec()),
        ];
        let mut manifest = String::new();
        let mut spine = String::new();
        let mut navigation = String::new();
        for (index, page) in self.pages.iter().enumerate() {
            let item_id = format!("page{}", index + 1);
            let image_id = format!("image{}", index + 1);
            let xhtml = format!("page-{}.xhtml", index + 1);
            let png_name = format!("page-{}.png", index + 1);
            let mut png = Cursor::new(Vec::new());
            DynamicImage::ImageRgb8(rgb_page(&page.layers, page.w, page.h))
                .write_to(&mut png, ImageFormat::Png)
                .map_err(|error| error.to_string())?;
            entries.push((format!("OEBPS/images/{}", png_name), png.into_inner()));
            let markup = format!("<?xml version=\"1.0\"?><html xmlns=\"http://www.w3.org/1999/xhtml\"><head><title>{}</title><meta name=\"viewport\" content=\"width={},height={}\"/></head><body><img src=\"images/{}\" alt=\"{}\" style=\"width:100%;height:auto\"/></body></html>", xml_escape(&page.name), page.w, page.h, png_name, xml_escape(&page.name));
            entries.push((format!("OEBPS/{}", xhtml), markup.into_bytes()));
            manifest.push_str(&format!("<item id=\"{}\" href=\"{}\" media-type=\"application/xhtml+xml\"/><item id=\"{}\" href=\"images/{}\" media-type=\"image/png\"/>", item_id, xhtml, image_id, png_name));
            spine.push_str(&format!("<itemref idref=\"{}\"/>", item_id));
            navigation.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>",
                xhtml,
                xml_escape(&page.name)
            ));
        }
        manifest.push_str("<item id=\"nav\" href=\"nav.xhtml\" properties=\"nav\" media-type=\"application/xhtml+xml\"/>");
        entries.push(("OEBPS/nav.xhtml".into(), format!("<html xmlns=\"http://www.w3.org/1999/xhtml\" xmlns:epub=\"http://www.idpf.org/2007/ops\"><head><title>Contents</title></head><body><nav epub:type=\"toc\"><ol>{}</ol></nav></body></html>", navigation).into_bytes()));
        entries.push(("OEBPS/content.opf".into(), format!("<?xml version=\"1.0\"?><package xmlns=\"http://www.idpf.org/2007/opf\" version=\"3.0\" unique-identifier=\"book-id\"><metadata xmlns:dc=\"http://purl.org/dc/elements/1.1/\"><dc:identifier id=\"book-id\">paint-studio-book</dc:identifier><dc:title>Paint Studio</dc:title><dc:language>en</dc:language></metadata><manifest>{}</manifest><spine>{}</spine></package>", manifest, spine).into_bytes()));
        let out = Path::new(path);
        if let Some(parent) = out.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        std::fs::write(out, make_stored_zip(entries)?).map_err(|error| error.to_string())
    }
}

fn grayscale_layer(layer: &mut Layer) {
    for row in &mut layer.pixels {
        for pixel in row {
            let gray =
                ((pixel.0 as u32 * 299 + pixel.1 as u32 * 587 + pixel.2 as u32 * 114) / 1000) as u8;
            *pixel = (gray, gray, gray, pixel.3);
        }
    }
}

fn rgb_page(layers: &[Layer], width: usize, height: usize) -> RgbImage {
    let mut image = RgbImage::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let mut dst = [255u8; 3];
            for layer in layers.iter().filter(|layer| layer.visible) {
                let (r, g, b, a) = layer.pixels[y][x];
                let alpha = a as f32 / 255.0 * layer.opacity_f32();
                dst[0] = (r as f32 * alpha + dst[0] as f32 * (1.0 - alpha)) as u8;
                dst[1] = (g as f32 * alpha + dst[1] as f32 * (1.0 - alpha)) as u8;
                dst[2] = (b as f32 * alpha + dst[2] as f32 * (1.0 - alpha)) as u8;
            }
            image.put_pixel(x as u32, y as u32, image::Rgb(dst));
        }
    }
    image
}

fn padded_rgb(image: &RgbImage, bleed: u32) -> RgbImage {
    let (width, height) = (image.width(), image.height());
    let mut padded = RgbImage::new(width + 2 * bleed, height + 2 * bleed);
    for y in 0..padded.height() {
        for x in 0..padded.width() {
            let sx = x.saturating_sub(bleed).min(width.saturating_sub(1));
            let sy = y.saturating_sub(bleed).min(height.saturating_sub(1));
            padded.put_pixel(x, y, *image.get_pixel(sx, sy));
        }
    }
    padded
}

fn make_print_pdf(pages: &[RgbImage], bleed_mm: f32) -> Result<Vec<u8>, String> {
    let mut objects = vec![Vec::new(); 3 + pages.len() * 3];
    objects[0] = b"<< /Type /Catalog /Pages 2 0 R >>".to_vec();
    let kids = (0..pages.len())
        .map(|index| format!("{} 0 R", 3 + index * 3))
        .collect::<Vec<_>>()
        .join(" ");
    objects[1] = format!("<< /Type /Pages /Kids [{}] /Count {} >>", kids, pages.len()).into_bytes();
    for (index, image) in pages.iter().enumerate() {
        let page_id = 3 + index * 3;
        let image_id = page_id + 1;
        let content_id = page_id + 2;
        let bleed_px = (bleed_mm.max(0.0) * 300.0 / 25.4).round() as u32;
        let bleed = bleed_px as f32 * 72.0 / 300.0;
        let trim_w = image.width() as f32 * 72.0 / 300.0;
        let trim_h = image.height() as f32 * 72.0 / 300.0;
        let padded = padded_rgb(image, bleed_px);
        let media_w = padded.width() as f32 * 72.0 / 300.0;
        let media_h = padded.height() as f32 * 72.0 / 300.0;
        let mut jpeg = Cursor::new(Vec::new());
        DynamicImage::ImageRgb8(padded.clone())
            .write_to(&mut jpeg, ImageFormat::Jpeg)
            .map_err(|error| error.to_string())?;
        let image_data = jpeg.into_inner();
        let content = format!(
            "q {} 0 0 {} 0 0 cm /Im0 Do Q\n{}",
            media_w,
            media_h,
            crop_mark_stream(trim_w, trim_h, bleed)
        );
        objects[page_id-1]=format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {:.3} {:.3}] /TrimBox [{:.3} {:.3} {:.3} {:.3}] /BleedBox [0 0 {:.3} {:.3}] /Resources << /XObject << /Im0 {} 0 R >> >> /Contents {} 0 R >>",media_w,media_h,bleed,bleed,bleed+trim_w,bleed+trim_h,media_w,media_h,image_id,content_id).into_bytes();
        let mut image_stream=format!("<< /Type /XObject /Subtype /Image /Width {} /Height {} /ColorSpace /DeviceRGB /BitsPerComponent 8 /Filter /DCTDecode /Length {} >>\nstream\n",padded.width(),padded.height(),image_data.len()).into_bytes();
        image_stream.extend(image_data);
        image_stream.extend_from_slice(b"\nendstream");
        objects[image_id - 1] = image_stream;
        let content_bytes = content.into_bytes();
        let mut content_stream =
            format!("<< /Length {} >>\nstream\n", content_bytes.len()).into_bytes();
        content_stream.extend(content_bytes);
        content_stream.extend_from_slice(b"endstream");
        objects[content_id - 1] = content_stream;
    }
    let mut pdf = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.extend(format!("{} 0 obj\n", index + 1).bytes());
        pdf.extend(object);
        pdf.extend_from_slice(b"\nendobj\n");
    }
    let xref = pdf.len();
    pdf.extend(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).bytes());
    pdf.extend_from_slice(b"0000000000 00000 f \n");
    for offset in offsets {
        pdf.extend(format!("{:010} 00000 n \n", offset).bytes());
    }
    pdf.extend(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF",
            objects.len() + 1,
            xref
        )
        .bytes(),
    );
    Ok(pdf)
}

fn crop_mark_stream(w: f32, h: f32, b: f32) -> String {
    let (gap, len) = (2.0, 8.0);
    let (x0, x1, y0, y1) = (b, b + w, b, b + h);
    let mut stream = String::from("0 G 0.25 w\n");
    for (x, y, sx, sy) in [
        (x0, y0, -1.0, -1.0),
        (x1, y0, 1.0, -1.0),
        (x0, y1, -1.0, 1.0),
        (x1, y1, 1.0, 1.0),
    ] {
        stream.push_str(&format!(
            "{} {} m {} {} l S {} {} m {} {} l S\n",
            x + sx * gap,
            y,
            x + sx * (gap + len),
            y,
            x,
            y + sy * gap,
            x,
            y + sy * (gap + len)
        ));
    }
    stream
}

fn make_stored_zip(entries: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>, String> {
    let entry_count = u16::try_from(entries.len()).map_err(|error| error.to_string())?;
    let mut output = Vec::new();
    let mut central = Vec::new();
    for (name, data) in entries {
        let filename = name.as_bytes();
        let name_len = u16::try_from(filename.len()).map_err(|error| error.to_string())?;
        let size = u32::try_from(data.len()).map_err(|error| error.to_string())?;
        let crc = crc32(&data);
        let offset = u32::try_from(output.len()).map_err(|error| error.to_string())?;
        push_u32(&mut output, 0x04034b50);
        push_u16(&mut output, 20);
        push_u16(&mut output, 0);
        push_u16(&mut output, 0);
        push_u16(&mut output, 0);
        push_u16(&mut output, 0);
        push_u32(&mut output, crc);
        push_u32(&mut output, size);
        push_u32(&mut output, size);
        push_u16(&mut output, name_len);
        push_u16(&mut output, 0);
        output.extend_from_slice(filename);
        output.extend_from_slice(&data);
        push_u32(&mut central, 0x02014b50);
        push_u16(&mut central, 20);
        push_u16(&mut central, 20);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u32(&mut central, crc);
        push_u32(&mut central, size);
        push_u32(&mut central, size);
        push_u16(&mut central, name_len);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u16(&mut central, 0);
        push_u32(&mut central, 0);
        push_u32(&mut central, offset);
        central.extend_from_slice(filename);
    }
    let central_offset = u32::try_from(output.len()).map_err(|error| error.to_string())?;
    let central_size = u32::try_from(central.len()).map_err(|error| error.to_string())?;
    output.extend(central);
    push_u32(&mut output, 0x06054b50);
    push_u16(&mut output, 0);
    push_u16(&mut output, 0);
    push_u16(&mut output, entry_count);
    push_u16(&mut output, entry_count);
    push_u32(&mut output, central_size);
    push_u32(&mut output, central_offset);
    push_u16(&mut output, 0);
    Ok(output)
}

fn push_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes())
}
fn push_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes())
}
fn crc32(data: &[u8]) -> u32 {
    let mut crc = !0u32;
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xedb88320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}
fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
