use eframe::egui::FontFamily::Proportional;
use eframe::egui::FontId;
use eframe::egui::TextStyle::{Body, Button, Heading, Monospace, Name, Small};
use eframe::{egui, CreationContext};
use font_kit::source::SystemSource;

pub fn set_style(cc: &CreationContext) {
    load_fonts(&cc.egui_ctx);
    let mut style = (*cc.egui_ctx.style()).clone();
    style.text_styles = [
        (Heading, FontId::new(20.0, Proportional)),
        (Name("Heading2".into()), FontId::new(25.0, Proportional)),
        (Name("Context".into()), FontId::new(23.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(14.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    cc.egui_ctx.set_style(style);
}

fn load_fonts(ctx: &egui::Context) {
    let sys = SystemSource::new();
    let font_name = "Microsoft YaHei UI".to_string();
    let font = sys.select_family_by_name(&font_name).unwrap().fonts()[2]
        .load()
        .unwrap()
        .copy_font_data()
        .unwrap()
        .to_vec();
    let mut font_defs = egui::FontDefinitions::default();
    font_defs
        .font_data
        .insert(font_name.to_string(), egui::FontData::from_owned(font));
    font_defs
        .families
        .get_mut(&Proportional)
        .unwrap()
        .insert(0, font_name);
    ctx.set_fonts(font_defs);
}
