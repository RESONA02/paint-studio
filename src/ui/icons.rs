// icons.rs — ระบบไอคอนเวกเตอร์ "Warm Ink" วาดด้วยโค้ด 100%
// กริด 24x24 หน่วย, stroke 1.75 px, ปลายเส้นมนทุกเส้น, ไม่ใช้ไอคอนภายนอก

use super::theme::Theme;
use eframe::egui;

/// ชนิดของเครื่องมือหลัก 12 ชนิดตามข้อกำหนด
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WarmToolIcon {
    Brush,      // แปรง: ปลายพู่กันเฉียงที่มีรอยหมึกต่อท้าย
    InkPen,     // ปากกาหมึก: หัวปากกาแหลม มีรอยหยดหมึกเล็ก
    Eraser,     // ยางลบ: แคปซูลเอียงครึ่งหนึ่งเป็นกรอบเส้นประ
    FillBucket, // ถังสี: รูปหยดน้ำที่มีแอ่งสีอยู่ด้านล่าง
    RectSelect, // เลือกกรอบ: สี่เหลี่ยมมุมเปิดสองมุม
    FreeSelect, // เลือกอิสระ: เส้นลูกศรปิดวนเป็นรูปร่างไม่สม่ำเสมอ
    Move,       // ย้าย: จุดศูนย์กลางพร้อมลูกศรสี่ทิศแบบสั้น
    Eyedropper, // ดูดสี: หลอดหยดแบบเส้นเดียว
    Text,       // ข้อความ: ตัว T ที่มีเส้นฐานยาวเกินเล็กน้อย
    Zoom,       // ซูม: เลนส์กลมที่มีด้ามเอียง
    Hand,       // มือ: ฝ่ามือแบบเส้นโค้งสามนิ้ว
    Layers,     // เลเยอร์: แผ่นซ้อน 3 ชั้นที่เยื้องกัน
}

/// สถานะการแสดงผลของไอคอน
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IconState {
    Normal,
    Hovered,
    Active,
    Disabled,
}

impl WarmToolIcon {
    pub const ALL: [WarmToolIcon; 12] = [
        WarmToolIcon::Brush,
        WarmToolIcon::InkPen,
        WarmToolIcon::Eraser,
        WarmToolIcon::FillBucket,
        WarmToolIcon::RectSelect,
        WarmToolIcon::FreeSelect,
        WarmToolIcon::Move,
        WarmToolIcon::Eyedropper,
        WarmToolIcon::Text,
        WarmToolIcon::Zoom,
        WarmToolIcon::Hand,
        WarmToolIcon::Layers,
    ];

    pub fn name_th(&self) -> &'static str {
        match self {
            WarmToolIcon::Brush => "แปรง",
            WarmToolIcon::InkPen => "ปากกาหมึก",
            WarmToolIcon::Eraser => "ยางลบ",
            WarmToolIcon::FillBucket => "ถังเทสี",
            WarmToolIcon::RectSelect => "เลือกกรอบ",
            WarmToolIcon::FreeSelect => "เลือกอิสระ",
            WarmToolIcon::Move => "ย้ายตำแหน่ง",
            WarmToolIcon::Eyedropper => "ดูดสี",
            WarmToolIcon::Text => "ข้อความ",
            WarmToolIcon::Zoom => "ซูมมุมมอง",
            WarmToolIcon::Hand => "เลื่อนภาพ",
            WarmToolIcon::Layers => "เลเยอร์",
        }
    }

    pub fn shortcut(&self) -> &'static str {
        match self {
            WarmToolIcon::Brush => "B",
            WarmToolIcon::InkPen => "P",
            WarmToolIcon::Eraser => "E",
            WarmToolIcon::FillBucket => "G",
            WarmToolIcon::RectSelect => "M",
            WarmToolIcon::FreeSelect => "L",
            WarmToolIcon::Move => "V",
            WarmToolIcon::Eyedropper => "I",
            WarmToolIcon::Text => "T",
            WarmToolIcon::Zoom => "Z",
            WarmToolIcon::Hand => "H",
            WarmToolIcon::Layers => "F7",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            WarmToolIcon::Brush => "ระบายสีนุ่มนวลพร้อมน้ำหนักลายเส้นแบบพู่กัน",
            WarmToolIcon::InkPen => "ตัดเส้นหมึกคมชัดและปรับแรงกดสม่ำเสมอ",
            WarmToolIcon::Eraser => "ลบพิกเซลหรือพื้นที่ที่ระบายบนเลเยอร์ปัจจุบัน",
            WarmToolIcon::FillBucket => "เทสีลงในขอบเขตปิดหรือพื้นที่ที่เลือกไว้",
            WarmToolIcon::RectSelect => "สร้างพื้นที่เลือกสี่เหลี่ยมผืนผ้าแบบกำหนดขนาด",
            WarmToolIcon::FreeSelect => "ลากเส้นเลือกพื้นที่อิสระได้อย่างเป็นธรรมชาติ",
            WarmToolIcon::Move => "ย้ายเนื้อหาบนเลเยอร์หรือส่วนที่เลือกไว้อย่างอิสระ",
            WarmToolIcon::Eyedropper => "ดูดค่าสีจากแคนวาสมาเป็นสีวาดหลัก",
            WarmToolIcon::Text => "พิมพ์ข้อความและจัดวางฟอนต์บนผืนผ้าใบ",
            WarmToolIcon::Zoom => "ขยายหรือย่อมุมมองของแคนวาสตามตำแหน่งเคอร์เซอร์",
            WarmToolIcon::Hand => "เลื่อนมุมมองแคนวาสอย่างนุ่มนวลโดยไม่กระทบภาพ",
            WarmToolIcon::Layers => "เปิด/ปิดพาเนลจัดการลำดับและคุณสมบัติเลเยอร์",
        }
    }
}

/// วาดไอคอนพร้อมพื้นหลังและเอฟเฟกต์ตามสถานะ (Normal, Hovered, Active, Disabled)
pub fn draw_icon_widget(
    p: &egui::Painter,
    rect: egui::Rect,
    icon: WarmToolIcon,
    state: IconState,
    theme: &Theme,
) {
    let center = rect.center();

    // 1. พื้นหลังตามสถานะ
    match state {
        IconState::Active => {
            // พื้นหลัง Accent อำพันเต็มกล่อง
            p.rect_filled(
                rect,
                egui::CornerRadius::same(theme.radius_button as u8),
                theme.accent,
            );
        }
        IconState::Hovered => {
            // วงกลมพื้นหลังโปร่งใส 12%
            let bg_radius = (rect.width().min(rect.height()) / 2.0) - 2.0;
            p.circle_filled(center, bg_radius, theme.bg_hover_overlay);
        }
        _ => {}
    }

    // 2. สีเส้นตามสถานะ
    let stroke_color = match state {
        IconState::Normal => theme.text_secondary,
        IconState::Hovered => theme.text_primary,
        IconState::Active => theme.bg_panel, // ตัดกับสีอำพันชัดเจน
        IconState::Disabled => theme.text_disabled,
    };

    let stroke = egui::Stroke::new(1.75, stroke_color);

    // 3. วาดเส้นและรูปทรงของไอคอนในกล่อง 24x24 px
    draw_icon_geometry(p, center, icon, stroke, stroke_color);
}

/// ฟังก์ชันวาดเส้นรูปทรงเรขาคณิตของแต่ละไอคอน (กริด 24x24 px, stroke 1.75 px)
fn draw_icon_geometry(
    p: &egui::Painter,
    c: egui::Pos2,
    icon: WarmToolIcon,
    s: egui::Stroke,
    col: egui::Color32,
) {
    match icon {
        WarmToolIcon::Brush => {
            // แปรง: ปลายพู่กันเฉียงที่มีรอยหมึกต่อท้าย
            // ตัวปลอกแปรงเอียง 45 องศา
            let tip = c + egui::vec2(-4.0, 3.0);
            let p_top = c + egui::vec2(5.0, -6.0);
            p.line_segment([tip, p_top], s);
            p.line_segment([c + egui::vec2(-1.5, 5.5), c + egui::vec2(7.5, -3.5)], s);
            p.line_segment([p_top, c + egui::vec2(7.5, -3.5)], s);
            // ปลายขนพู่กัน
            p.line_segment([tip, c + egui::vec2(-7.0, 7.0)], s);
            p.line_segment([c + egui::vec2(-1.5, 5.5), c + egui::vec2(-7.0, 7.0)], s);
            // รอยหมึกต่อท้าย (Trailing ink curved dot)
            p.circle_filled(c + egui::vec2(-6.5, -5.0), 1.5, col);
            p.circle_filled(c + egui::vec2(-3.5, -7.0), 1.0, col);
        }

        WarmToolIcon::InkPen => {
            // ปากกาหมึก: หัวปากกาแหลม มีรอยหยดหมึกเล็ก
            let tip = c + egui::vec2(0.0, 8.0);
            let left_sh = c + egui::vec2(-5.0, -1.0);
            let right_sh = c + egui::vec2(5.0, -1.0);
            let top_left = c + egui::vec2(-3.5, -7.0);
            let top_right = c + egui::vec2(3.5, -7.0);

            // โครงหัวปากกา (Nib outline)
            p.line_segment([tip, left_sh], s);
            p.line_segment([left_sh, top_left], s);
            p.line_segment([top_left, top_right], s);
            p.line_segment([top_right, right_sh], s);
            p.line_segment([right_sh, tip], s);

            // ร่องผ่ากลางหัวปากกา (Nib slit)
            p.line_segment([tip, c + egui::vec2(0.0, 1.5)], s);
            p.circle_filled(c + egui::vec2(0.0, 1.0), 1.2, col);

            // หยดหมึกเล็กด้านข้าง
            p.circle_filled(c + egui::vec2(6.5, 4.5), 1.4, col);
        }

        WarmToolIcon::Eraser => {
            // ยางลบ: แคปซูลเอียงครึ่งหนึ่งเป็นกรอบเส้นประ
            // แกนเอียง -35 องศา
            let p1 = c + egui::vec2(-6.0, 4.0);
            let p2 = c + egui::vec2(3.0, -5.0);
            let p3 = c + egui::vec2(6.5, -1.5);
            let p4 = c + egui::vec2(-2.5, 7.5);

            // ครึ่งล่าง: เส้นทึบ (Solid half)
            p.line_segment([p1, p4], s);
            p.line_segment([p4, c + egui::vec2(2.0, 3.0)], s);
            p.line_segment([p1, c + egui::vec2(-1.5, -0.5)], s);

            // ครึ่งบน: เส้นประ (Dashed half)
            p.line_segment(
                [c + egui::vec2(-0.5, -1.5), p2],
                egui::Stroke::new(1.4, col),
            );
            p.line_segment([p2, p3], egui::Stroke::new(1.4, col));
            p.line_segment([p3, c + egui::vec2(3.0, 2.0)], egui::Stroke::new(1.4, col));

            // เส้นแบ่งครึ่ง
            p.line_segment(
                [c + egui::vec2(-1.5, -0.5), c + egui::vec2(2.0, 3.0)],
                egui::Stroke::new(1.0, col),
            );
        }

        WarmToolIcon::FillBucket => {
            // ถังสี: รูปหยดน้ำที่มีแอ่งสีอยู่ด้านล่าง
            // หยดน้ำคว่ำกำลังตกลงมา (Falling droplet)
            let top_tip = c + egui::vec2(0.0, -7.5);
            let drop_left = c + egui::vec2(-4.0, -1.5);
            let drop_right = c + egui::vec2(4.0, -1.5);
            let drop_bottom = c + egui::vec2(0.0, 1.5);

            p.line_segment([top_tip, drop_left], s);
            p.line_segment([top_tip, drop_right], s);
            p.line_segment([drop_left, drop_bottom], s);
            p.line_segment([drop_right, drop_bottom], s);

            // แอ่งสีโค้งอยู่ด้านล่าง (Puddle of paint)
            let p_left = c + egui::vec2(-7.5, 6.0);
            let p_mid = c + egui::vec2(0.0, 7.5);
            let p_right = c + egui::vec2(7.5, 6.0);
            p.line_segment([p_left, p_mid], s);
            p.line_segment([p_mid, p_right], s);
            p.circle_filled(c + egui::vec2(0.0, -2.5), 1.3, col);
        }

        WarmToolIcon::RectSelect => {
            // เลือกกรอบ: สี่เหลี่ยมมุมเปิดสองมุม (Open corner rectangle)
            let min = c + egui::vec2(-7.0, -7.0);
            let max = c + egui::vec2(7.0, 7.0);
            let len = 5.0;

            // มุมบนซ้าย (Top-Left corner bracket)
            p.line_segment([min, min + egui::vec2(len, 0.0)], s);
            p.line_segment([min, min + egui::vec2(0.0, len)], s);

            // มุมล่างขวา (Bottom-Right corner bracket)
            p.line_segment([max, max - egui::vec2(len, 0.0)], s);
            p.line_segment([max, max - egui::vec2(0.0, len)], s);

            // ขีดจุดบอกแนวขอบ (Subtle boundary ticks)
            p.circle_filled(c + egui::vec2(-7.0, 7.0), 1.2, col);
            p.circle_filled(c + egui::vec2(7.0, -7.0), 1.2, col);
        }

        WarmToolIcon::FreeSelect => {
            // เลือกอิสระ: เส้นลูกศรปิดวนเป็นรูปร่างไม่สม่ำเสมอ (Organic looping path with arrow)
            let p1 = c + egui::vec2(-6.0, -3.0);
            let p2 = c + egui::vec2(-2.0, -7.0);
            let p3 = c + egui::vec2(5.0, -4.0);
            let p4 = c + egui::vec2(6.0, 4.0);
            let p5 = c + egui::vec2(1.0, 7.0);
            let p6 = c + egui::vec2(-5.0, 5.0);

            p.line_segment([p1, p2], s);
            p.line_segment([p2, p3], s);
            p.line_segment([p3, p4], s);
            p.line_segment([p4, p5], s);
            p.line_segment([p5, p6], s);
            p.line_segment([p6, p1], s);

            // หัวลูกศรตรงจุดวนบรรจบ (Looping arrowhead)
            p.line_segment([p1, p1 + egui::vec2(3.0, -1.0)], s);
            p.line_segment([p1, p1 + egui::vec2(0.0, 3.0)], s);
        }

        WarmToolIcon::Move => {
            // ย้าย: จุดศูนย์กลางพร้อมลูกศรสี่ทิศแบบสั้น
            p.circle_filled(c, 2.2, col);

            // 4 ทิศ (บน ล่าง ซ้าย ขวา)
            let len = 6.5;
            let arr = 2.5;

            // บน
            let top = c - egui::vec2(0.0, len);
            p.line_segment([c, top], s);
            p.line_segment([top, top + egui::vec2(-arr, arr)], s);
            p.line_segment([top, top + egui::vec2(arr, arr)], s);

            // ล่าง
            let bot = c + egui::vec2(0.0, len);
            p.line_segment([c, bot], s);
            p.line_segment([bot, bot + egui::vec2(-arr, -arr)], s);
            p.line_segment([bot, bot + egui::vec2(arr, -arr)], s);

            // ซ้าย
            let lft = c - egui::vec2(len, 0.0);
            p.line_segment([c, lft], s);
            p.line_segment([lft, lft + egui::vec2(arr, -arr)], s);
            p.line_segment([lft, lft + egui::vec2(arr, arr)], s);

            // ขวา
            let rgt = c + egui::vec2(len, 0.0);
            p.line_segment([c, rgt], s);
            p.line_segment([rgt, rgt + egui::vec2(-arr, -arr)], s);
            p.line_segment([rgt, rgt + egui::vec2(-arr, arr)], s);
        }

        WarmToolIcon::Eyedropper => {
            // ดูดสี: หลอดหยดแบบเส้นเดียว (Monoline pipette)
            // เอียง 45 องศา
            let bulb_top = c + egui::vec2(5.5, -5.5);
            let bulb_left = c + egui::vec2(3.5, -7.5);
            let bulb_right = c + egui::vec2(7.5, -3.5);
            let tube_start = c + egui::vec2(2.5, -2.5);
            let tip = c + egui::vec2(-6.0, 6.0);

            // จุกยาง (Bulb)
            p.line_segment([bulb_left, bulb_top], s);
            p.line_segment([bulb_top, bulb_right], s);
            p.line_segment([bulb_left, tube_start], s);
            p.line_segment([bulb_right, tube_start], s);

            // หลอดแก้ว (Stem)
            p.line_segment([tube_start, tip], s);

            // หยดสีที่ปลาย (Droplet at tip)
            p.circle_filled(tip + egui::vec2(-2.0, 2.0), 1.5, col);
        }

        WarmToolIcon::Text => {
            // ข้อความ: ตัว T ที่มีเส้นฐานยาวเกินเล็กน้อย (T with extended baseline shelf)
            let top_l = c + egui::vec2(-6.5, -6.5);
            let top_r = c + egui::vec2(6.5, -6.5);
            let stem_top = c + egui::vec2(0.0, -6.5);
            let stem_bot = c + egui::vec2(0.0, 6.5);

            // คานบน T
            p.line_segment([top_l, top_r], s);
            // เสากลาง
            p.line_segment([stem_top, stem_bot], s);
            // เส้นฐานยาวเกิน (Extended baseline shelf)
            let base_l = c + egui::vec2(-4.5, 6.5);
            let base_r = c + egui::vec2(4.5, 6.5);
            p.line_segment([base_l, base_r], s);
        }

        WarmToolIcon::Zoom => {
            // ซูม: เลนส์กลมที่มีด้ามเอียง
            let lens_center = c + egui::vec2(-2.5, -2.5);
            let radius = 5.5;
            p.circle_stroke(lens_center, radius, s);

            // ด้ามจับเอียง 45 องศา
            let handle_start = lens_center + egui::vec2(3.8, 3.8);
            let handle_end = c + egui::vec2(7.5, 7.5);
            p.line_segment([handle_start, handle_end], egui::Stroke::new(2.2, col));
        }

        WarmToolIcon::Hand => {
            // มือ: ฝ่ามือแบบเส้นโค้งสามนิ้ว (Three-fingered curved palm)
            let wrist_l = c + egui::vec2(-4.0, 7.0);
            let wrist_r = c + egui::vec2(4.0, 7.0);

            // นิ้วโป้ง
            let thumb = c + egui::vec2(-7.0, 1.0);
            // นิ้วชี้
            let finger1 = c + egui::vec2(-2.5, -7.0);
            // นิ้วกลาง
            let finger2 = c + egui::vec2(1.5, -7.5);
            // นิ้วนาง
            let finger3 = c + egui::vec2(5.5, -5.0);

            p.line_segment([wrist_l, thumb], s);
            p.line_segment([thumb, finger1], s);
            p.line_segment([finger1, finger2], s);
            p.line_segment([finger2, finger3], s);
            p.line_segment([finger3, wrist_r], s);
            p.line_segment([wrist_l, wrist_r], s);
        }

        WarmToolIcon::Layers => {
            // เลเยอร์: แผ่นซ้อน 3 ชั้นที่เยื้องกัน (3 staggered rectangular sheets)
            for i in 0..3 {
                let offset_y = -5.0 + (i as f32 * 5.0);
                let offset_x = (i as f32 - 1.0) * 2.0;
                let min = c + egui::vec2(-6.0 + offset_x, offset_y - 2.0);
                let max = c + egui::vec2(6.0 + offset_x, offset_y + 2.0);
                p.rect_stroke(
                    egui::Rect::from_min_max(min, max),
                    egui::CornerRadius::same(1),
                    egui::Stroke::new(1.3, col),
                    egui::StrokeKind::Inside,
                );
            }
        }
    }
}

/// ฟังก์ชันวาดเส้นประ Marching Ants และ Corner Handles สำหรับ Floating Selection
pub(crate) fn draw_marching_ants(p: &egui::Painter, rect: egui::Rect, show_handles: bool) {
    let stroke_black = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(20, 20, 20));
    let stroke_white = egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(240, 240, 240));

    p.rect_stroke(
        rect,
        egui::CornerRadius::ZERO,
        stroke_black,
        egui::StrokeKind::Outside,
    );
    p.rect_stroke(
        rect.shrink(1.0_f32),
        egui::CornerRadius::ZERO,
        stroke_white,
        egui::StrokeKind::Inside,
    );

    if show_handles {
        draw_handles(p, rect, stroke_black);
    }
}

fn draw_handles(p: &egui::Painter, rect: egui::Rect, stroke_black: egui::Stroke) {
    let handle_size = egui::vec2(6.0_f32, 6.0_f32);
    for pt in [
        rect.left_top(),
        rect.right_top(),
        rect.left_bottom(),
        rect.right_bottom(),
        rect.left_center(),
        rect.right_center(),
        rect.center_top(),
        rect.center_bottom(),
    ] {
        let hr = egui::Rect::from_center_size(pt, handle_size);
        p.rect_filled(hr, egui::CornerRadius::same(1), egui::Color32::WHITE);
        p.rect_stroke(
            hr,
            egui::CornerRadius::same(1),
            stroke_black,
            egui::StrokeKind::Inside,
        );
    }

    let stalk_start = rect.center_top();
    let stalk_end = stalk_start - egui::vec2(0.0_f32, 18.0_f32);
    p.line_segment([stalk_start, stalk_end], stroke_black);
    p.circle_filled(stalk_end, 3.5_f32, egui::Color32::WHITE);
    p.circle_stroke(stalk_end, 3.5_f32, stroke_black);
}
