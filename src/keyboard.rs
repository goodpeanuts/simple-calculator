use eframe::egui;
use eframe::egui::Widget;
use crate::math_exp;


static KEYS: [&str; 25] = [
    "√", "C", "(", ")", "del",
    "sin", "7", "8", "9", "*",
    "cos", "4", "5", "6", "/",
    "tg", "1", "2", "3", "-",
    "ctg", ".", "0", "=", "+"
];


pub struct CalcKeyboard<'a> {
    buffer: &'a mut math_exp::MathExp,
    pub width: f32,
    pub height: f32,
}


impl<'a> CalcKeyboard<'a> {
    pub fn from_buffer(buffer: &'a mut math_exp::MathExp) -> Self {
        Self {
            buffer,
            width: 340.0,
            height: 320.0,
        }
    }

    pub fn show(self, ui: &mut egui::Ui) {
        egui::Grid::new("keyboard")
            .num_columns(5)
            .max_col_width(self.width)
            .show(ui, |ui| {
                for (ind, title) in KEYS.iter().enumerate() {
                    if ind % 5 == 0 && ind != 0 {
                        ui.end_row();
                    }
                    if CustomKey::from(*title).ui(ui).clicked() {
                        match *title {
                            "C" => { self.buffer.clear(); }
                            "del" => { self.buffer.pop(); }
                            "=" => { self.buffer.calculate(); }
                            _ => { self.buffer.add(title); }
                        }
                    };
                }
            });
    }
}

pub struct CustomKey {
    pub text: String,
    pub width: f32,
    pub height: f32,
}

impl CustomKey {
    // 创建一个新按钮并设置给定的文本.
    pub fn from(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Default::default()
        }
    }
    // 设置按钮大小.
    fn _set_size(mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }
}

impl Default for CustomKey {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            width: 58.0,
            height: 48.0,
        }
    }
}


impl Widget for CustomKey {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_sized(
            [self.width, self.height],
            egui::Button::new(self.text).small(),
        )
    }
}

