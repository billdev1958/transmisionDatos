use egui::epaint::TextShape;
use egui::{Align2, Color32, CornerRadius, FontId, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, Ui, Vec2};

use crate::core::codificacion::Senal;
use crate::ui::theme::{OBSIDIAN_1, TEXT};

const COLORES: [Color32; 7] = [
    Color32::from_rgb(0, 229, 255),
    Color32::from_rgb(255, 140, 66),
    Color32::from_rgb(255, 214, 74),
    Color32::from_rgb(190, 132, 255),
    Color32::from_rgb(140, 220, 90),
    Color32::from_rgb(255, 110, 150),
    Color32::from_rgb(110, 160, 255),
];

const MARGEN_IZQ: f32 = 56.0;
const MARGEN_DER: f32 = 12.0;
const MARGEN_SUP: f32 = 48.0;
const MARGEN_INF: f32 = 44.0;
const ANCHO_MIN_BIT: f32 = 28.0;

pub fn grafica(ui: &mut Ui, bits: &[bool], senal: &Senal) {
    let alto = ui.available_height().max(220.0);
    egui::ScrollArea::horizontal().auto_shrink([false, false]).show(ui, |ui| {
        let ancho = ui
            .available_width()
            .max(MARGEN_IZQ + MARGEN_DER + ANCHO_MIN_BIT * bits.len() as f32);
        let (lienzo, _) = ui.allocate_exact_size(Vec2::new(ancho, alto - 16.0), Sense::hover());
        dibujar(ui, lienzo, bits, senal);
    });
}

fn dibujar(ui: &Ui, lienzo: Rect, bits: &[bool], senal: &Senal) {
    let painter = ui.painter_at(lienzo);
    let gris = Color32::from_gray(140);
    let rejilla = Stroke::new(1.0, Color32::from_gray(55));

    let area = Rect::from_min_max(
        Pos2::new(lienzo.left() + MARGEN_IZQ, lienzo.top() + MARGEN_SUP),
        Pos2::new(lienzo.right() - MARGEN_DER, lienzo.bottom() - MARGEN_INF),
    );
    painter.rect_filled(area, CornerRadius::same(4u8), OBSIDIAN_1);
    painter.rect_stroke(area, CornerRadius::same(4u8), rejilla, StrokeKind::Middle);

    let ancho_bit = area.width() / bits.len() as f32;
    let x = |t: f32| area.left() + t * ancho_bit;
    // x: el bit i ocupa de i a i+1, escalado al ancho del área.
    // y: el nivel 0 va al centro y ±1 al 75 % de la media altura (el eje y de pantalla crece hacia abajo).
    let y = |nivel: i8| area.center().y - nivel as f32 * area.height() * 0.375;

    for (nivel, etiqueta) in [(1, "+V"), (0, "0"), (-1, "-V")] {
        painter.line_segment([Pos2::new(area.left(), y(nivel)), Pos2::new(area.right(), y(nivel))], rejilla);
        painter.text(
            Pos2::new(area.left() - 8.0, y(nivel)),
            Align2::RIGHT_CENTER,
            etiqueta,
            FontId::proportional(13.0),
            gris,
        );
    }
    let galera = painter.layout_no_wrap("Amplitud".to_owned(), FontId::proportional(14.0), TEXT);
    let pos = Pos2::new(lienzo.left() + 4.0, area.center().y + galera.size().x / 2.0);
    painter.add(TextShape::new(pos, galera, TEXT).with_angle(-std::f32::consts::FRAC_PI_2));

    for i in 0..=bits.len() {
        let xi = x(i as f32);
        painter.extend(Shape::dashed_line(
            &[Pos2::new(xi, area.top()), Pos2::new(xi, area.bottom())],
            rejilla,
            4.0,
            4.0,
        ));
        if ancho_bit >= 22.0 || i % 5 == 0 {
            painter.text(
                Pos2::new(xi, area.bottom() + 6.0),
                Align2::CENTER_TOP,
                i.to_string(),
                FontId::proportional(11.0),
                gris,
            );
        }
    }
    painter.text(
        Pos2::new(area.center().x, lienzo.bottom() - 2.0),
        Align2::CENTER_BOTTOM,
        "Tiempo (bits)",
        FontId::proportional(14.0),
        TEXT,
    );

    let mut previo: Option<i8> = None;
    for (i, [a, b]) in senal.niveles.iter().copied().enumerate() {
        let t = i as f32;
        let color = COLORES[i % COLORES.len()];
        let mut puntos = Vec::with_capacity(5);
        if let Some(p) = previo {
            puntos.push(Pos2::new(x(t), y(p)));
        }
        puntos.extend([
            Pos2::new(x(t), y(a)),
            Pos2::new(x(t + 0.5), y(a)),
            Pos2::new(x(t + 0.5), y(b)),
            Pos2::new(x(t + 1.0), y(b)),
        ]);
        painter.add(Shape::line(puntos, Stroke::new(3.0, color)));
        previo = Some(b);

        painter.text(
            Pos2::new(x(t + 0.5), area.top() - 8.0),
            Align2::CENTER_BOTTOM,
            if bits[i] { "1" } else { "0" },
            FontId::monospace(18.0),
            color,
        );
        if let Some(marca) = senal.marcas[i] {
            painter.text(
                Pos2::new(x(t + 0.5), area.top() - 30.0),
                Align2::CENTER_BOTTOM,
                marca,
                FontId::proportional(12.0),
                Color32::from_rgb(255, 110, 110),
            );
        }
    }
}
