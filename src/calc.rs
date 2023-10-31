use eframe::egui;

use rust1::{keyboard, math_exp};

pub(crate) struct CalcApp {
    //state: State,
    pub(crate) math_exp: math_exp::MathExp,
}

impl CalcApp {
    
    fn draw_keyboard(&mut self, ctx: &egui::Context, ui: &mut egui::Ui){
        keyboard::CalcKeyboard::from_buffer(&mut self.math_exp).show(ui)
    }

    pub(crate) fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CalcApp {
            //state: State::default(),
            math_exp: math_exp::MathExp::default(),
        }
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("screen_panel").show(ctx, |ui| {
            let size_font = |l: f32| -> f32 {
                if l <= 22.0 {
                    25.0
                } else {
                    let a = 330.0 / (l / 0.6);
                    if a > 12.0 { a } else { 12.0 }
                }
            };

            let result = self.math_exp.get_output();
            let result_length = result.chars().count() as f32;
            let expression = self.math_exp.to_string();
            let expression_length = expression.chars().count() as f32;
            ui.add_sized(
                [330.0, 70.0],
                egui::Label::new(
                    egui::RichText::new(expression)
                        .font(egui::FontId::monospace(size_font(expression_length))),
            ).wrap(true),
            );
            ui.add_sized(
                [330.0, 45.0],
                egui::Label::new(
                    egui::RichText::new(result)
                        .font(egui::FontId::monospace(size_font(result_length)))
                        .color(egui::Color32::LIGHT_GREEN)
                ).wrap(true),
            );
        });

        egui::CentralPanel::default().show(ctx, |ui| {
           self.draw_keyboard(ctx, ui)
        });
        

    }
}