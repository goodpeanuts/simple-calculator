use eframe::egui;

//use egui_extras::*;

use rust1::{calc_config, calc_state, keyboard, math_exp,loan};



pub(crate) struct CalcApp {
    pub(crate) ui_state: calc_state::UiState,
    pub(crate) math_exp: math_exp::MathExp,
    pub loan: loan::Loan,
}

pub const PRIMARY_COLOR: egui::Color32 = egui::Color32::from_rgb(25, 95, 200);
pub const PRIMARY_COLOR_HOVERED: egui::Color32 = egui::Color32::from_rgb(35, 115, 230);
pub const PRIMARY_COLOR_ACTIVE: egui::Color32 = egui::Color32::from_rgb(15, 75, 170);



impl CalcApp {

    fn standard_ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui){
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Min),
            |ui|{
                let size_font = |l: f32| -> f32 {
                    if l <= 22.0 {
                        25.0
                    } else {
                        let a = 330.0 / (l / 0.6);
                        if a > 12.0 {
                            a
                        } else {
                            12.0
                        }
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
                    )
                    .wrap(true),
                );
                ui.add_sized(
                    [330.0, 45.0],
                    egui::Label::new(
                        egui::RichText::new(result)
                            .font(egui::FontId::monospace(size_font(result_length)))
                            .color(egui::Color32::LIGHT_GREEN),
                    )
                    .wrap(true),
                );
        });
        ui.allocate_ui_with_layout(ui.available_size(),
            egui::Layout::top_down(egui::Align::Center)
            , |ui|{
                    keyboard::CalcKeyboard::from_buffer(&mut self.math_exp).show(ui)
        });
        
    }

    fn loan_ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui){
        let space = 20.0;

        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Center),
        |ui|{


            // 上半部分
            ui.allocate_ui_with_layout(
                ui.available_size(),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui|{
                    ui.add_space(space);
                    ui.vertical(|ui|{
                        // 竖直方向上标签间距
                        let temp_space = egui::vec2(0.0, 22.0);
                        ui.spacing_mut().item_spacing = temp_space;
                        ui.label("还款方式：");
                        ui.label("贷款年限(年):");
                        ui.label("贷款金额(万元):");
                        ui.label("贷款利率(%):");

                    });

                    let mut my_string = String::new();

                    ui.add_space(space);
                    ui.vertical(|ui|{
                        let temp_space = egui::vec2(10.0, 22.0);
                        ui.spacing_mut().item_spacing = temp_space;
                        ui.horizontal(|ui|{
                            ui.selectable_value(&mut self.loan.loan_type, loan::LoanType::EqualInterest, "等额本息");
                            ui.selectable_value(&mut self.loan.loan_type, loan::LoanType::EqualPrincipal, "等额本金");
                        });
                        // 贷款年限
                        ui.add(egui::DragValue::new(&mut self.loan.loan_year));
                        // 贷款金额
                        ui.add(egui::DragValue::new(&mut self.loan.loan_money));
                        // 贷款利率
                        ui.add(egui::DragValue::new(&mut self.loan.loan_rate));
                    });
            });

            // 中间部分
            ui.horizontal(|ui|{
                if ui.add(egui::Button::new("计算")).clicked() {
                    
                }

                if ui.add(egui::Button::new("重置")).clicked() {
                
                }
            });

            // 下半部分
            ui.allocate_ui_with_layout(
                ui.available_size(),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui|{
                    ui.add_space(space);
                    ui.vertical(|ui|{
                        // 竖直方向上标签间距
                        let temp_space = egui::vec2(0.0, 22.0);
                        ui.spacing_mut().item_spacing = temp_space;
                        ui.label("月均还款(元):");
                        ui.label("贷款年限(元):");
                        ui.label("贷款金额(元):");

                    });

                    let mut my_string = String::new();

                    ui.add_space(space);
                    ui.vertical(|ui|{
                        let temp_space = egui::vec2(10.0, 22.0);
                        ui.spacing_mut().item_spacing = temp_space;
                        // 月均还款
                        ui.add(egui::DragValue::new(&mut self.loan.money_per_month));
                        // 利息总额
                        ui.add(egui::DragValue::new(&mut self.loan.total_interest));
                        // 还款总额
                        ui.add(egui::DragValue::new(&mut self.loan.total_money));
                    });
            });
            

        });

        
    }

    fn settings_ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui){
        egui::ScrollArea::vertical()
            .max_height(ui.available_height())
            // .auto_shrink([false;2])
            .show(ui, |ui|{
                ui.label(ui.available_height().to_string());
                ctx.settings_ui(ui);
            });
    }

    pub(crate) fn new(cc: &eframe::CreationContext<'_>) -> Self {
        calc_config::custom_font(cc);
        CalcApp {
            ui_state: calc_state::UiState::default(),
            math_exp: math_exp::MathExp::default(),
            loan: loan::Loan::default(),
        }
    }
}

impl eframe::App for CalcApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       

        egui::CentralPanel::default().show(ctx, |ui| {
            let menu_size = egui::vec2(150.0, ui.available_height());
            let button_fill = ui.visuals().widgets.inactive.weak_bg_fill;

            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    menu_size,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        for (name, menu, nav) in [
                            ("standard", "标准", calc_state::Nav::Standard),
                            ("loan", "利率", calc_state::Nav::Loan),
                            ("settings", "设置", calc_state::Nav::Settings),
                        ] {
                        
                            let menu_size = egui::vec2(120.0, 32.0);
                            // 设置间距和按钮的外观
                            ui.spacing_mut().item_spacing = egui::Vec2::splat(10.0);
                            ui.visuals_mut().widgets.hovered.bg_stroke.width = 0.0;
                            ui.visuals_mut().widgets.active.bg_stroke.width = 0.0;
                            // 根据当前页面状态设置按钮的填充颜色
                            if self.ui_state.page.to_str() != name {
                                ui.visuals_mut().widgets.inactive.weak_bg_fill =
                                    ui.visuals().window_fill;
                            } else {
                                ui.visuals_mut().widgets.inactive.weak_bg_fill = button_fill;
                            }

                            let button = ui.add_sized(
                                menu_size,
                                egui::Button::new(
                                   egui::RichText::new(menu).size(14.0),
                                ),
                            );
                            if button.clicked() {
                                if self.ui_state.page != nav {
                                    self.ui_state.page = nav;
                                    self.ui_state.at.reset_menu_animate_time();
                                }
                            }
                        }
                        ui.allocate_space(ui.available_size());
                    },
                );
                let offset_y = self.ui_state.at.menu_change * 25.0;
                egui::Frame::group(ui.style())
                    .multiply_with_opacity(0.1)
                    .show(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| match self.ui_state.page {
                                calc_state::Nav::Standard => self.standard_ui(ctx, ui),
                                calc_state::Nav::Loan => self.loan_ui(ctx, ui),
                                calc_state::Nav::Settings => self.settings_ui(ctx, ui),
                            },
                        );
                    });
            });
        });
    }
}
