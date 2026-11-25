use crate::core::codificacion::Codificacion;
use crate::ui::screens::principal;

pub struct App {
    pub entrada: String,
    pub seleccion: Codificacion,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::ui::theme::apply_theme(&cc.egui_ctx);
        Self {
            entrada: "0010001110".to_owned(),
            seleccion: Codificacion::Manchester,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        principal::show(ctx, self);
    }
}
