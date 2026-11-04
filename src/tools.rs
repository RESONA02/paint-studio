// tools.rs — Tool and SubTool definitions
// รายการเครื่องมือหลักและเครื่องมือย่อย พร้อมค่าตั้งต้นของแปรงแต่ละชนิด

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum Tool {
    Zoom,
    Hand,
    LayerSelect,
    Move,
    Lasso,
    MagicWand,
    EyeDropper,
    #[default]
    Pen,
    Pencil,
    Brush,
    Airbrush,
    Decoration,
    Eraser,
    Blend,
    Fill,
    Gradient,
    Figure,
    Text,
    Balloon,
    ComicFrame,
    CorrectLine,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum SubTool {
    // Pen
    #[default]
    GPen,
    MappingPen,
    TurnipPen,
    Calligraphy,
    Marker,
    MilliPen,
    // Pencil
    RealPencil,
    DesignPencil,
    DarkerPencil,
    ColoredPencil,
    // Brush
    Watercolor,
    ThickPaint,
    IndiaInk,
    // Airbrush
    SoftAirbrush,
    HardAirbrush,
    HighlightAirbrush,
    DropletAirbrush,
    // Eraser
    HardEraser,
    SoftEraser,
    RoughEraser,
    KneadedEraser,
    // Fill
    ReferEditLayer,
    ReferOtherLayers,
    CloseAndFill,
    // Blend
    ColorBlend,
    Blur,
    Fingertip,
    // Gradient
    FgToTransparent,
    FgToBg,
    Stripe,
    // Figure
    RectangleFigure,
    EllipseFigure,
    StraightLine,
    PolygonFigure,
    // Navigation / Utilities
    ZoomIn,
    ZoomOut,
    HandPan,
    RotateCanvas,
    LayerSelectPick,
    LayerMoveNormal,
    LassoFree,
    LassoPoly,
    MagicWandSelect,
    EyeDropperPick,
    TextHorizontal,
    BalloonSpeech,
    ComicFrameRect,
    CorrectLineSmooth,
}

#[derive(Clone, Copy)]
pub struct SubToolDef {
    pub subtool: SubTool,
    pub name_th: &'static str,
    pub name_en: &'static str,
    pub default_size: f32,
    pub default_opacity: f32,
    pub hardness: f32,
}

impl Tool {
    pub fn subtools(&self) -> Vec<SubToolDef> {
        match self {
            Tool::Pen => pen_subtools(),
            Tool::Pencil => pencil_subtools(),
            Tool::Brush => brush_subtools(),
            Tool::Airbrush => airbrush_subtools(),
            Tool::Eraser => eraser_subtools(),
            Tool::Fill => fill_subtools(),
            Tool::Blend => blend_subtools(),
            Tool::Gradient => gradient_subtools(),
            Tool::Figure => figure_subtools(),
            Tool::Zoom => zoom_subtools(),
            Tool::Hand => hand_subtools(),
            Tool::Move => one(SubTool::LayerMoveNormal, "ย้ายเลเยอร์", "Move Layer", 1.0),
            Tool::LayerSelect => one(
                SubTool::LayerSelectPick,
                "คลิกเลือกเลเยอร์",
                "Select Layer",
                1.0,
            ),
            Tool::Lasso => lasso_subtools(),
            Tool::MagicWand => one(SubTool::MagicWandSelect, "เลือกอัตโนมัติ", "Magic Wand", 1.0),
            Tool::EyeDropper => one(SubTool::EyeDropperPick, "ดูดสีจากหน้าจอ", "Eyedropper", 1.0),
            Tool::Text => one(
                SubTool::TextHorizontal,
                "ข้อความแนวนอน",
                "Horizontal Text",
                24.0,
            ),
            Tool::Balloon => one(SubTool::BalloonSpeech, "บอลลูนคำพูด", "Speech Balloon", 2.0),
            Tool::ComicFrame => one(
                SubTool::ComicFrameRect,
                "ตัดกรอบช่องการ์ตูน",
                "Comic Frame Border",
                3.0,
            ),
            Tool::CorrectLine => one(
                SubTool::CorrectLineSmooth,
                "ทำให้เส้นเรียบ",
                "Smooth Line",
                10.0,
            ),
            Tool::Decoration => vec![def(SubTool::Marker, "ประกายดาว", "Sparkle", 20.0, 1.0, 0.8)],
        }
    }
}

fn def(
    subtool: SubTool,
    name_th: &'static str,
    name_en: &'static str,
    size: f32,
    opacity: f32,
    hardness: f32,
) -> SubToolDef {
    SubToolDef {
        subtool,
        name_th,
        name_en,
        default_size: size,
        default_opacity: opacity,
        hardness,
    }
}

fn one(
    subtool: SubTool,
    name_th: &'static str,
    name_en: &'static str,
    size: f32,
) -> Vec<SubToolDef> {
    vec![def(subtool, name_th, name_en, size, 1.0, 1.0)]
}

fn pen_subtools() -> Vec<SubToolDef> {
    vec![
        def(SubTool::GPen, "ปากกา G-pen", "G-pen", 10.0, 1.0, 1.0),
        def(
            SubTool::MappingPen,
            "ปากกาเขียนแผนที่",
            "Mapping pen",
            5.0,
            1.0,
            1.0,
        ),
        def(
            SubTool::TurnipPen,
            "ปากกาหัวช้อน",
            "Turnip pen",
            12.0,
            1.0,
            0.8,
        ),
        def(
            SubTool::Calligraphy,
            "ปากกาคัดลายมือ",
            "Calligraphy",
            15.0,
            1.0,
            0.9,
        ),
        def(SubTool::Marker, "มาร์กเกอร์", "Marker", 20.0, 0.8, 0.6),
        def(SubTool::MilliPen, "มิลลิเพน", "Milli pen", 3.0, 1.0, 1.0),
    ]
}

fn pencil_subtools() -> Vec<SubToolDef> {
    vec![
        def(
            SubTool::RealPencil,
            "ดินสอจริง",
            "Real pencil",
            8.0,
            0.75,
            0.6,
        ),
        def(
            SubTool::DesignPencil,
            "ดินสอร่างแบบ",
            "Design pencil",
            12.0,
            0.6,
            0.4,
        ),
        def(
            SubTool::DarkerPencil,
            "ดินสอเข้ม",
            "Darker pencil",
            10.0,
            0.9,
            0.8,
        ),
        def(
            SubTool::ColoredPencil,
            "ดินสอสี",
            "Colored pencil",
            14.0,
            0.7,
            0.5,
        ),
    ]
}

fn brush_subtools() -> Vec<SubToolDef> {
    vec![
        def(
            SubTool::Watercolor,
            "สีน้ำโปร่ง",
            "Watercolor",
            18.0,
            0.65,
            0.4,
        ),
        def(
            SubTool::ThickPaint,
            "สีข้น / สีน้ำมัน",
            "Thick paint",
            22.0,
            1.0,
            0.8,
        ),
        def(SubTool::IndiaInk, "หมึกจีน", "India ink", 16.0, 0.95, 0.9),
    ]
}

fn airbrush_subtools() -> Vec<SubToolDef> {
    vec![
        def(SubTool::SoftAirbrush, "นุ่มนวล", "Soft", 40.0, 0.4, 0.2),
        def(SubTool::HardAirbrush, "คมชัด", "Hard", 30.0, 0.6, 0.6),
        def(
            SubTool::HighlightAirbrush,
            "ไฮไลท์",
            "Highlight",
            35.0,
            0.5,
            0.3,
        ),
        def(
            SubTool::DropletAirbrush,
            "ละอองสเปรย์",
            "Droplet",
            25.0,
            0.7,
            0.5,
        ),
    ]
}

fn eraser_subtools() -> Vec<SubToolDef> {
    vec![
        def(
            SubTool::HardEraser,
            "ยางลบแข็ง",
            "Hard eraser",
            16.0,
            1.0,
            1.0,
        ),
        def(
            SubTool::SoftEraser,
            "ยางลบอ่อน",
            "Soft eraser",
            26.0,
            0.6,
            0.3,
        ),
        def(
            SubTool::RoughEraser,
            "ยางลบหยาบ",
            "Rough eraser",
            20.0,
            0.9,
            0.7,
        ),
        def(
            SubTool::KneadedEraser,
            "ยางลบซับสี",
            "Kneaded eraser",
            30.0,
            0.4,
            0.2,
        ),
    ]
}

fn fill_subtools() -> Vec<SubToolDef> {
    vec![
        def(
            SubTool::ReferEditLayer,
            "อ้างอิงเฉพาะเลเยอร์",
            "Refer edit layer",
            1.0,
            1.0,
            1.0,
        ),
        def(
            SubTool::ReferOtherLayers,
            "อ้างอิงทุกเลเยอร์",
            "Refer other layers",
            1.0,
            1.0,
            1.0,
        ),
        def(
            SubTool::CloseAndFill,
            "ล้อมรอบแล้วเติม",
            "Close and fill",
            1.0,
            1.0,
            1.0,
        ),
    ]
}

fn blend_subtools() -> Vec<SubToolDef> {
    vec![
        def(SubTool::ColorBlend, "ผสานสี", "Blend", 20.0, 0.7, 0.5),
        def(SubTool::Blur, "เบลอ", "Blur", 25.0, 0.6, 0.3),
        def(
            SubTool::Fingertip,
            "เกลี่ยปลายนิ้ว",
            "Fingertip",
            18.0,
            0.8,
            0.6,
        ),
    ]
}

fn gradient_subtools() -> Vec<SubToolDef> {
    vec![
        def(
            SubTool::FgToTransparent,
            "สีหลักไปโปร่งใส",
            "Fg to Transparent",
            1.0,
            1.0,
            1.0,
        ),
        def(SubTool::FgToBg, "สีหลักไปสีรอง", "Fg to Bg", 1.0, 1.0, 1.0),
        def(SubTool::Stripe, "แถบสีลายทาง", "Stripe", 1.0, 1.0, 1.0),
    ]
}

fn figure_subtools() -> Vec<SubToolDef> {
    vec![
        def(
            SubTool::StraightLine,
            "เส้นตรง",
            "Straight line",
            4.0,
            1.0,
            1.0,
        ),
        def(
            SubTool::RectangleFigure,
            "สี่เหลี่ยม",
            "Rectangle",
            4.0,
            1.0,
            1.0,
        ),
        def(
            SubTool::EllipseFigure,
            "วงรี / วงกลม",
            "Ellipse",
            4.0,
            1.0,
            1.0,
        ),
        def(
            SubTool::PolygonFigure,
            "หลายเหลี่ยม",
            "Polygon",
            4.0,
            1.0,
            1.0,
        ),
    ]
}

fn zoom_subtools() -> Vec<SubToolDef> {
    vec![
        def(SubTool::ZoomIn, "ขยาย (Zoom In)", "Zoom In", 1.0, 1.0, 1.0),
        def(SubTool::ZoomOut, "ย่อ (Zoom Out)", "Zoom Out", 1.0, 1.0, 1.0),
    ]
}

fn hand_subtools() -> Vec<SubToolDef> {
    vec![
        def(SubTool::HandPan, "เลื่อนภาพ (Pan)", "Pan", 1.0, 1.0, 1.0),
        def(
            SubTool::RotateCanvas,
            "หมุนภาพ (Rotate)",
            "Rotate",
            1.0,
            1.0,
            1.0,
        ),
    ]
}

fn lasso_subtools() -> Vec<SubToolDef> {
    vec![
        def(SubTool::LassoFree, "บ่วงบาศอิสระ", "Lasso", 1.0, 1.0, 1.0),
        def(
            SubTool::LassoPoly,
            "เลือกกรอบสี่เหลี่ยม",
            "Rect Select",
            1.0,
            1.0,
            1.0,
        ),
    ]
}
