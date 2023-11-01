#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod calc;

// enum Nav {
//     calc_normal,
//     calc_loan,
//     calc_other,
// }

// impl Default for Nav {
//     fn default() -> Self {
//         Self::calc_normal
//     }
// }

use eframe::{egui};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(550.0, 480.0)), // 设置窗口的宽度和高度
        ..Default::default() // 使用其他默认选项
    };
    eframe::run_native(
        "计算器",
        options,
        Box::new(|cc| Box::new(calc::CalcApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Убедитесь, что паника регистрируется с помощью `console.error`.
    console_error_panic_hook::set_once();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "calculator-wasm-rust-pwa",
            eframe::WebOptions::default(),
            Box::new(|cc| Box::new(CalcApp::new(cc))),
        )
            .await
            .expect("failed to start calculator-wasm-rust-pwa");
    });
}

