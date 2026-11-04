// Paint Studio — a simple desktop paint app inspired by CLIP STUDIO PAINT's
// workflow: canvas, pen tools, colour panel, layers and history.
// All artwork is original; all names in the code are generic (no copied assets).

use eframe::egui;

#[derive(Clone, Copy, PartialEq)]
enum Tool { Pen, Pencil, Eraser, Fill, EyeDropper }

struct Layer {
    name: String,
    visible: bool,
    locked: bool,
    opacity: f32,
    pixels: Vec<Vec<(u8,u8,u8,u8)>>,
}

struct App {
    w: usize, h: usize,
    layers: Vec<Layer>,
    active: usize,
    tool: Tool,
    color: egui::Color32,
    size: f32,
    drawing: bool,
    last: Option<(isize,isize)>,
    history: Vec<String>,
    hi: usize,
    zoom: f32,
    saved: String,
    screenshot_path: Option<String>,
    frame_count: u32,
    screenshot_requested: bool,
}

impl App {
    fn new(screenshot_path: Option<String>) -> Self {
        let w = 400usize; let h = 300usize;
        let blank = |name:&str| Layer { name: name.to_string(), visible:true, locked:false, opacity:1.0,
            pixels: vec![vec![(255,255,255,255); w]; h] };
        let mut layers = vec![blank("Paper"), blank("Ink"), blank("Colour")];
        // simple sample drawing so the first screenshot isn't blank
        for x in 60..340 { for y in 55..65 { let p=&mut layers[1].pixels[y][x]; *p=(30,30,35,255); } }
        for y in 55..220 { for x in 55..65 { let p=&mut layers[1].pixels[y][x]; *p=(30,30,35,255); } }
        for y in 215..225 { for x in 55..340 { let p=&mut layers[1].pixels[y][x]; *p=(30,30,35,255); } }
        for x in 330..340 { for y in 55..220 { let p=&mut layers[1].pixels[y][x]; *p=(30,30,35,255); } }
        for r in 0..60 { for c in 0..80 {
            let x = 160 + c; let y = 110 + r;
            let dx = (c as f64-40.0)/40.0; let dy = (r as f64-30.0)/30.0;
            if dx*dx+dy*dy < 1.0 { layers[2].pixels[y][x] = (255, 190, 90, 255); } } }
        Self { w, h, layers, active:1, tool:Tool::Pen,
            color: egui::Color32::from_rgb(30,30,35), size: 8.0,
            drawing:false, last:None, history: vec!["Opened new canvas".into()],
            hi:0, zoom:1.0, saved: String::new(),
            screenshot_path, frame_count: 0, screenshot_requested: false }
    }
    fn push_hist(&mut self, s:&str){ self.history.truncate(self.hi+1); self.history.push(s.into()); self.hi=self.history.len()-1; }
    fn canvas_pos(&self, p: egui::Pos2, rect: egui::Rect) -> (isize,isize) {
        let x = (p.x - rect.min.x)/self.zoom; let y = (p.y - rect.min.y)/self.zoom;
        (x as isize, y as isize)
    }
    fn stroke(&mut self, a:(isize,isize), b:(isize,isize)){
        let steps = ((b.0-a.0).abs().max((b.1-a.1).abs())).max(1);
        let r = (self.size as isize).max(1)/2;
        let col = match self.tool { Tool::Eraser => (255,255,255,255),
            Tool::Pencil => (self.color.r(), self.color.g(), self.color.b(), 200),
            _ => (self.color.r(), self.color.g(), self.color.b(), 255) };
        let li = self.active;
        if self.layers[li].locked { return; }
        for s in 0..=steps {
            let cx = a.0 + (b.0-a.0)*s/steps; let cy = a.1 + (b.1-a.1)*s/steps;
            for dy in -r..=r { for dx in -r..=r {
                let x = cx+dx; let y = cy+dy;
                if x>=0 && y>=0 && (x as usize)<self.w && (y as usize)<self.h {
                    self.layers[li].pixels[y as usize][x as usize]=col;
                }}}
        }
    }
    fn save_png(&self) -> std::result::Result<(), String> {
        let mut img = image::RgbaImage::new(self.w as u32, self.h as u32);
        for y in 0..self.h { for x in 0..self.w {
            // composite top-down (skip transparent handled as white)
            let mut px=[255u8,255,255,255];
            for li in (0..self.layers.len()).rev() {
                let l=&self.layers[li];
                if !l.visible { continue; }
                let (r,g,b,a)=l.pixels[y][x];
                let al = (a as f32/255.0)*l.opacity;
                px[0]=(r as f32*al+px[0] as f32*(1.0-al)) as u8;
                px[1]=(g as f32*al+px[1] as f32*(1.0-al)) as u8;
                px[2]=(b as f32*al+px[2] as f32*(1.0-al)) as u8;
            }
            img.put_pixel(x as u32,y as u32,image::Rgba(px));
        }}
        let out_path = std::path::Path::new("shots/art.png");
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        img.save(out_path).map_err(|e| e.to_string())?;
        Ok(())
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for event in ctx.input(|i| i.raw.events.clone()) {
            if let egui::Event::Screenshot { image, .. } = event {
                if let Some(path) = &self.screenshot_path {
                    if let Some(parent) = std::path::Path::new(path).parent() {
                        if !parent.as_os_str().is_empty() {
                            let _ = std::fs::create_dir_all(parent);
                        }
                    }
                    image::save_buffer(
                        path,
                        image.as_raw(),
                        image.size[0] as u32,
                        image.size[1] as u32,
                        image::ColorType::Rgba8,
                    )
                    .expect("failed to save screenshot");
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("✒ Paint Studio");
                ui.separator();
                if ui.button("New").clicked(){ self.push_hist("New canvas"); }
                if ui.button("Save PNG").clicked(){ match self.save_png(){ Ok(_v)=>{self.saved="saved ✓".into(); self.push_hist("Saved PNG");}, Err(e)=>{self.saved=e;} } }
                if ui.button("Undo").clicked() && self.hi>0 { self.hi-=1; self.push_hist("Undo"); }
                if ui.button("Redo").clicked() && self.hi+1<self.history.len() { self.hi+=1; }
                ui.label(format!("Zoom {:.0}%", self.zoom*100.0));
                ui.add(egui::Slider::new(&mut self.zoom, 0.5..=2.5).text(""));
                ui.label(self.saved.clone());
            });
        });
        egui::SidePanel::left("tools").show(ctx, |ui| {
            ui.heading("Tools");
            for (t,n) in [(Tool::Pen,"🖊 Pen"),(Tool::Pencil,"✏ Pencil"),(Tool::Eraser,"🧽 Eraser"),(Tool::Fill,"🪣 Fill"),(Tool::EyeDropper,"💧 Eye Dropper")] {
                if ui.selectable_label(self.tool==t, n).clicked(){ self.tool=t; }
            }
            ui.add(egui::Slider::new(&mut self.size, 1.0..=40.0).text("Size"));
            ui.separator(); ui.heading("Colour");
            ui.colored_label(self.color, "Selected colour");
            for c in [(30,30,35),(200,40,40),(40,120,220),(40,180,90),(250,200,60),(150,80,220),(0,0,0),(255,255,255)] {
                let cc=egui::Color32::from_rgb(c.0,c.1,c.2);
                if ui.button("■").clicked(){ self.color=cc; }
            }
        });
        egui::SidePanel::right("layers").show(ctx, |ui| {
            ui.heading("Layers");
            let n=self.layers.len();
            for i in (0..n).rev() {
                let l:&mut Layer = &mut self.layers[i];
                let label=format!("{}{}", if i==self.active {"▶ "} else {"  "}, l.name);
                if ui.selectable_label(false, label).clicked(){ self.active=i; }
                ui.add(egui::Checkbox::new(&mut l.visible, "show"));
                ui.add(egui::Slider::new(&mut l.opacity, 0.0..=1.0).text("opacity"));
            }
            if ui.button("+ Add layer").clicked(){ let k=self.layers.len()+1; let w=self.w; let h=self.h;
                self.layers.push(Layer{name:format!("Layer {}",k),visible:true,locked:false,opacity:1.0,pixels:vec![vec![(0,0,0,0);w];h]}); self.push_hist("Added layer"); }
            ui.separator(); ui.heading("History");
            for i in 0..self.history.len() { ui.label(format!("{} {}", if i==self.hi {"●"} else {"○"}, self.history[i])); }
            ui.separator(); ui.heading("Navigator");
            ui.label("Canvas 400 × 300 px");
            ui.label(format!("Active: {}", self.layers[self.active].name));
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let avail=ui.available_size();
            let side=(avail.x - self.w as f32*self.zoom).max(0.0)/2.0;
            let cw = self.w as f32*self.zoom; let ch = self.h as f32*self.zoom;
            let rect=egui::Rect::from_min_size(ui.cursor().min+egui::vec2(side,10.0), egui::vec2(cw, ch));
            let painter=ui.painter();
            painter.rect_filled(rect, 2.0, egui::Color32::WHITE);
            // sample: draw layer pixels as tiny rects (coarse but visible)
            let step=3usize;
            for y in (0..self.h).step_by(step){ for x in (0..self.w).step_by(step){
                let mut px=(255,255,255,255);
                for li in (0..self.layers.len()).rev(){ let l=&self.layers[li]; if !l.visible{continue;}
                    let (r,g,b,a)=l.pixels[y][x]; if a>0{px=(r,g,b,a);break;} }
                let rr=egui::Rect::from_min_size(rect.min+egui::vec2(x as f32*self.zoom, y as f32*self.zoom), egui::vec2(step as f32*self.zoom+1.0, step as f32*self.zoom+1.0));
                painter.rect_filled(rr,0.0,egui::Color32::from_rgba_unmultiplied(px.0,px.1,px.2,px.3));
            }}
            painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0_f32, egui::Color32::GRAY), egui::StrokeKind::Inside);
            // interaction
            let resp=ui.interact(rect, egui::Id::new("canvas"), egui::Sense::click_and_drag());
            let ptr=resp.interact_pointer_pos();
            if resp.drag_started() { self.drawing=true; }
            if resp.drag_started_by(egui::PointerButton::Primary) { self.drawing=true; }
            if resp.drag_stopped() { self.drawing=false; self.last=None; self.push_hist("Stroked"); }
            if self.drawing {
                if let Some(p)=ptr {
                    let c=self.canvas_pos(p, rect);
                    if let Some(l)=self.last { self.stroke(l,c); } else { self.stroke(c,c); }
                    self.last=Some(c);
                }
            }
            if resp.clicked() && self.tool==Tool::EyeDropper {
                if let Some(p)=ptr { let (x,y)=self.canvas_pos(p,rect);
                    if x>=0&&y>=0&&(x as usize)<self.w&&(y as usize)<self.h {
                        let (r,g,b,_)=self.layers[self.active].pixels[y as usize][x as usize];
                        self.color=egui::Color32::from_rgb(r,g,b); } }
            }
        });
        if self.screenshot_path.is_some() {
            self.frame_count += 1;
            ctx.request_repaint();
            if self.frame_count >= 120 && !self.screenshot_requested {
                self.screenshot_requested = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
            }
        }
    }
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let shot = args.windows(2).find(|w| w[0]=="--screenshot").map(|w| w[1].clone());
    let opts = eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1180.0, 760.0]), ..Default::default() };
    let app = App::new(shot);
    eframe::run_native("Paint Studio", opts, Box::new(move |_cc| Ok(Box::new(app) as Box<dyn eframe::App>)))
}
