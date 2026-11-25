use egui::{Button, Color32, FontId, Margin, RichText, Stroke, Vec2};

use crate::app::App;
use crate::core::codificacion::{Codificacion, codificar, leer_bits};
use crate::ui::theme::{JADE, OBSIDIAN_0, OBSIDIAN_1, OBSIDIAN_2, TEXT};
use crate::ui::widgets::grafica::grafica;

const ERROR: Color32 = Color32::from_rgb(255, 110, 110);

pub fn show(ctx: &egui::Context, app: &mut App) {
    egui::SidePanel::left("sidebar")
        .exact_width(210.0)
        .resizable(false)
        .show_separator_line(false)
        .frame(egui::Frame::new().fill(OBSIDIAN_1).inner_margin(Margin::same(12)))
        .show(ctx, |ui| {
            ui.label(RichText::new("Transmisión de Datos").color(TEXT).size(15.0).strong());
            ui.label(
                RichText::new("Codificación de señales digitales")
                    .size(10.0)
                    .color(Color32::from_gray(120)),
            );
            ui.add_space(14.0);
            ui.separator();
            ui.add_space(8.0);
            ui.label(
                RichText::new("CODIFICACIÓN")
                    .size(9.0)
                    .color(Color32::from_gray(140))
                    .strong(),
            );
            ui.add_space(4.0);
            for cod in Codificacion::TODAS {
                if boton_codificacion(ui, cod.nombre(), app.seleccion == cod).clicked() {
                    app.seleccion = cod;
                }
                ui.add_space(4.0);
            }
        });

    egui::TopBottomPanel::top("entrada")
        .show_separator_line(false)
        .frame(egui::Frame::new().fill(OBSIDIAN_0).inner_margin(Margin::same(12)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Ingresa código:").color(TEXT).size(14.0).strong());
                ui.add(
                    egui::TextEdit::singleline(&mut app.entrada)
                        .hint_text("cadena de bits, p. ej. 0010001110")
                        .font(FontId::monospace(16.0))
                        .desired_width(f32::INFINITY),
                );
            });
        });

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(OBSIDIAN_0).inner_margin(Margin::same(12)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(format!("CÓDIGO {}", app.seleccion.nombre().to_uppercase()))
                        .color(JADE)
                        .size(22.0)
                        .strong(),
                );
                ui.label(
                    RichText::new(app.seleccion.descripcion())
                        .size(12.0)
                        .color(Color32::from_gray(160)),
                );
            });
            ui.add_space(8.0);

            match leer_bits(&app.entrada) {
                Err(c) => {
                    ui.label(
                        RichText::new(format!(
                            "Carácter no válido: «{c}». La cadena solo puede contener 0 y 1."
                        ))
                        .color(ERROR),
                    );
                }
                Ok(bits) if bits.is_empty() => {
                    ui.label("Ingresa una cadena de bits para ver su codificación.");
                }
                Ok(bits) => grafica(ui, &bits, &codificar(app.seleccion, &bits)),
            }
        });
}

fn boton_codificacion(ui: &mut egui::Ui, nombre: &str, activo: bool) -> egui::Response {
    let (relleno, borde, texto) = if activo {
        (Color32::from_rgb(0, 45, 60), JADE, JADE)
    } else {
        (OBSIDIAN_2, Color32::from_gray(60), TEXT)
    };
    ui.add(
        Button::new(RichText::new(nombre).color(texto).size(13.0))
            .fill(relleno)
            .stroke(Stroke::new(1.0, borde))
            .min_size(Vec2::new(186.0, 34.0)),
    )
}
