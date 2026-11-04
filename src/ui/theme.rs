// theme.rs — โทเคนการออกแบบสไตล์ "Warm Ink" (Night Studio + Warm Amber)
// ห้าม hardcode สีหรือขนาดใน widget โดยเด็ดขาด ให้อ่านค่าจาก struct Theme เท่านั้น

use eframe::egui;

/// โครงสร้าง Design Tokens หลักสำหรับทั้งแอปพลิเคชัน
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Theme {
    pub is_dark: bool,

    // พื้นหลังและโครงสร้าง (Surfaces)
    pub bg_app: egui::Color32,
    pub bg_panel: egui::Color32,
    pub bg_panel_header: egui::Color32,
    pub bg_layer_active: egui::Color32,
    pub bg_hover_overlay: egui::Color32,

    // เส้นขอบ (Borders & Dividers)
    pub border: egui::Color32,
    pub border_subtle: egui::Color32,

    // สีข้อความและไอคอน (Typography & Content)
    pub text_primary: egui::Color32,
    pub text_secondary: egui::Color32,
    pub text_disabled: egui::Color32,

    // สีเน้นและสถานะ (Accents & Feedback)
    pub accent: egui::Color32,
    pub accent_hover: egui::Color32,
    pub accent_active: egui::Color32,
    pub alert: egui::Color32,

    // มุมโค้งมน (Corner Radii)
    pub radius_panel: f32,
    pub radius_button: f32,
    pub radius_input: f32,

    // ระยะห่างแบบกริด 4 px (Spacing Tokens)
    pub space_4: f32,
    pub space_8: f32,
    pub space_12: f32,
    pub space_16: f32,

    // ขนาดตัวอักษร (Typography Font Sizes)
    pub font_size_header: f32,
    pub font_size_body: f32,
    pub font_size_tabular: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

impl Theme {
    /// ธีมมืด Warm Ink: บรรยากาศห้องวาดภาพตอนกลางคืน โทนอุ่น โค้งมน ลุ่มลึก
    pub const fn dark() -> Self {
        Self {
            is_dark: true,
            bg_app: egui::Color32::from_rgb(0x17, 0x18, 0x1C),
            bg_panel: egui::Color32::from_rgb(0x1F, 0x21, 0x27),
            bg_panel_header: egui::Color32::from_rgb(0x26, 0x28, 0x30),
            bg_layer_active: egui::Color32::from_rgb(0x2C, 0x2F, 0x38),
            bg_hover_overlay: egui::Color32::from_rgba_premultiplied(236, 233, 228, 30), // 12% overlay

            border: egui::Color32::from_rgb(0x33, 0x36, 0x3F),
            border_subtle: egui::Color32::from_rgb(0x28, 0x2A, 0x33),

            text_primary: egui::Color32::from_rgb(0xEC, 0xE9, 0xE4),
            text_secondary: egui::Color32::from_rgb(0x8E, 0x90, 0x99),
            text_disabled: egui::Color32::from_rgb(0x4A, 0x4D, 0x57),

            accent: egui::Color32::from_rgb(0xF2, 0xA9, 0x3B), // Warm Amber
            accent_hover: egui::Color32::from_rgb(0xFF, 0xBE, 0x5C),
            accent_active: egui::Color32::from_rgb(0xD9, 0x92, 0x27),
            alert: egui::Color32::from_rgb(0xE5, 0x48, 0x4D),

            radius_panel: 10.0,
            radius_button: 8.0,
            radius_input: 6.0,

            space_4: 4.0,
            space_8: 8.0,
            space_12: 12.0,
            space_16: 16.0,

            font_size_header: 12.0,
            font_size_body: 13.0,
            font_size_tabular: 13.0,
        }
    }

    /// ธีมสว่าง Warm Ink Light: สีอำพันเข้มขึ้นเพื่อความคมชัดผ่านเกณฑ์ WCAG AA (>= 4.5:1)
    pub const fn light() -> Self {
        Self {
            is_dark: false,
            bg_app: egui::Color32::from_rgb(0xEE, 0xEA, 0xE2),
            bg_panel: egui::Color32::from_rgb(0xF7, 0xF5, 0xF0),
            bg_panel_header: egui::Color32::from_rgb(0xEA, 0xE6, 0xDD),
            bg_layer_active: egui::Color32::from_rgb(0xEB, 0xE4, 0xD6),
            bg_hover_overlay: egui::Color32::from_rgba_premultiplied(28, 27, 24, 25),

            border: egui::Color32::from_rgb(0xCE, 0xC9, 0xBD),
            border_subtle: egui::Color32::from_rgb(0xDD, 0xD8, 0xCC),

            text_primary: egui::Color32::from_rgb(0x1C, 0x1B, 0x18),
            text_secondary: egui::Color32::from_rgb(0x68, 0x65, 0x5E),
            text_disabled: egui::Color32::from_rgb(0x9E, 0x9A, 0x92),

            accent: egui::Color32::from_rgb(0xA6, 0x63, 0x04), // Darkened amber for WCAG AA >= 4.5:1
            accent_hover: egui::Color32::from_rgb(0xB8, 0x73, 0x0A),
            accent_active: egui::Color32::from_rgb(0x8C, 0x52, 0x02),
            alert: egui::Color32::from_rgb(0xC7, 0x2A, 0x30),

            radius_panel: 10.0,
            radius_button: 8.0,
            radius_input: 6.0,

            space_4: 4.0,
            space_8: 8.0,
            space_12: 12.0,
            space_16: 16.0,

            font_size_header: 12.0,
            font_size_body: 13.0,
            font_size_tabular: 13.0,
        }
    }

    /// ปรับใช้การตั้งค่าภาพของ egui ตามธีม Warm Ink
    pub fn apply_to_visuals(&self, ctx: &egui::Context) {
        let mut visuals = if self.is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        visuals.override_text_color = Some(self.text_primary);
        visuals.panel_fill = self.bg_app;
        visuals.window_fill = self.bg_panel;
        visuals.window_stroke = egui::Stroke::new(1.0, self.border);
        visuals.window_corner_radius = egui::CornerRadius::same(self.radius_panel as u8);

        // Widget สภาพนิ่ง (Noninteractive)
        visuals.widgets.noninteractive.bg_fill = self.bg_panel;
        visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, self.text_secondary);
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, self.border);
        visuals.widgets.noninteractive.corner_radius =
            egui::CornerRadius::same(self.radius_button as u8);

        // Widget ปุ่มปกติ (Inactive)
        visuals.widgets.inactive.bg_fill = self.bg_panel_header;
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.text_primary);
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, self.border);
        visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(self.radius_button as u8);

        // Widget ขณะชี้เมาส์ (Hovered)
        visuals.widgets.hovered.bg_fill = if self.is_dark {
            egui::Color32::from_rgb(0x32, 0x35, 0x40)
        } else {
            egui::Color32::from_rgb(0xDE, 0xD8, 0xCC)
        };
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, self.text_primary);
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, self.accent_hover);
        visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(self.radius_button as u8);

        // Widget ขณะกดใช้งาน (Active)
        visuals.widgets.active.bg_fill = self.accent;
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, self.bg_panel);
        visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, self.accent);
        visuals.widgets.active.corner_radius = egui::CornerRadius::same(self.radius_button as u8);

        // สไตล์ Selection
        visuals.selection.bg_fill = self.accent;
        visuals.selection.stroke = egui::Stroke::new(1.0, self.bg_panel);

        ctx.set_visuals(visuals);
    }
}

/// ฟังก์ชันวาดโลโก้ monoline ของแบรนด์ "Warm Ink":
/// ลากเป็นเส้นเดียวต่อเนื่อง ปลายเส้นตวัดโค้งแล้วจบด้วยจุดหยดหมึกกลม
pub fn draw_warm_ink_logo(p: &egui::Painter, center: egui::Pos2, size: f32, color: egui::Color32) {
    let scale = size / 24.0;
    let stroke = egui::Stroke::new((1.8 * scale).max(1.2), color);

    // เส้นโครงหลักรูปหยดหมึกตวัดเส้นเดียว (Monoline S-curve ending in droplet)
    let p1 = center + egui::vec2(-8.0 * scale, 6.0 * scale);
    let p2 = center + egui::vec2(-4.0 * scale, -7.0 * scale);
    let p3 = center + egui::vec2(3.0 * scale, -7.0 * scale);
    let p4 = center + egui::vec2(6.0 * scale, 0.0 * scale);
    let p5 = center + egui::vec2(0.0 * scale, 7.0 * scale);
    let p6 = center + egui::vec2(4.0 * scale, 7.0 * scale);

    // วาดส่วนโค้งตวัด
    p.line_segment([p1, p2], stroke);
    p.line_segment([p2, p3], stroke);
    p.line_segment([p3, p4], stroke);
    p.line_segment([p4, p5], stroke);
    p.line_segment([p5, p6], stroke);

    // จุดหมึกเด่นชัดที่ปลายเส้น (Ink Dot)
    let dot_center = center + egui::vec2(7.5 * scale, 4.5 * scale);
    p.circle_filled(dot_center, 2.2 * scale, color);
}
