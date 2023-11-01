//use std::os::windows::prelude::BorrowedHandle;

use eframe::egui;
use egui::Color32;


//use egui_extras::*;

use rust1::{calc_config, calc_state, keyboard, loan, math_exp};

pub(crate) struct CalcApp {
    pub(crate) ui_state: calc_state::UiState,
    pub(crate) math_exp: math_exp::MathExp,
    pub loan: loan::Loan,
}

pub const PRIMARY_COLOR: egui::Color32 = egui::Color32::from_rgb(25, 95, 200);
pub const PRIMARY_COLOR_HOVERED: egui::Color32 = egui::Color32::from_rgb(35, 115, 230);
pub const PRIMARY_COLOR_ACTIVE: egui::Color32 = egui::Color32::from_rgb(15, 75, 170);

impl CalcApp {
    fn draw_menu_line(&mut self, ctx: &egui::Context, ui: &mut egui::Ui){
        use egui::*;
    
        let line_width = 3.0;
        let line_height = 16.0;
        let line_root = ui.cursor().min + vec2(8.0, (32.0-line_height)/2.0);
        let target = match self.ui_state.page{
            calc_state::Nav::Standard => Rect::from_min_size(line_root, vec2(line_width, line_height)),
            calc_state::Nav::Loan => Rect::from_min_size(line_root + vec2(0.0, 42.0), vec2(line_width, line_height)),
            calc_state::Nav::Settings => Rect::from_min_size(line_root + vec2(0.0, 84.0), vec2(line_width, line_height)),
        };
        let mut current = self.ui_state.current_menu_line_rect.unwrap_or(target);
        if self.ui_state.at.menu_change > 0.0{
            let target_y_center = (target.min.y + target.max.y) / 2.0;
            if (current.min.y - target_y_center).abs() > (current.max.y - target_y_center).abs(){
                current.min.y = lerp((current.min.y)..=(target.min.y), 
                    ((1.0 - self.ui_state.at.menu_change) * 2.0 - 1.0).clamp(0.0, 1.0));
                current.max.y = lerp((current.max.y)..=(target.max.y), 
                    ((1.0 - self.ui_state.at.menu_change) * 2.0).clamp(0.0, 1.0));
            }else{
                current.min.y = lerp((current.min.y)..=(target.min.y), 
                    ((1.0 - self.ui_state.at.menu_change) * 2.0).clamp(0.0, 1.0));
                current.max.y = lerp((current.max.y)..=(target.max.y), 
                    ((1.0 - self.ui_state.at.menu_change) * 2.0 - 1.0).clamp(0.0, 1.0));
            }
            ui.painter().rect_filled(current, Rounding::same(line_width / 2.0), PRIMARY_COLOR);
            self.ui_state.current_menu_line_rect = Some(current);
            self.ui_state.at.menu_change = self.ui_state.at.menu_change - 0.1;
            ctx.request_repaint();
        }else{
            self.ui_state.at.menu_change = 0.0;
            ui.painter().rect_filled(target, Rounding::same(line_width / 2.0), PRIMARY_COLOR);
            self.ui_state.current_menu_line_rect = Some(target);
        }
    }
    
    fn standard_ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
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
                    [330.0, 75.0],
                    egui::Label::new(
                        egui::RichText::new(result)
                            .font(egui::FontId::monospace(size_font(result_length)))
                            .color(egui::Color32::LIGHT_GREEN),
                    )
                    .wrap(true),
                );
            },
        );
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Center),
            |ui| keyboard::CalcKeyboard::from_buffer(&mut self.math_exp).show(ui),
        );
    }

    fn loan_ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let space = 2.0;
        let calc_size = egui::vec2(500.0, 600.0);

        ui.allocate_ui_with_layout(
            calc_size,
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                let temp_space = egui::vec2(20.0, 20.0);
                ui.spacing_mut().item_spacing = temp_space;

                // 上半部分
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::left_to_right(egui::Align::Min),
                    |ui| {
                        ui.vertical(|ui| {
                            // 竖直方向上标签间距
                            ui.label(egui::RichText::new(
                                "🔥还款方式：")
                            .size(16.0)
                            );

                            ui.label(egui::RichText::new("贷款年限(年):").size(16.0));
                            ui.label(egui::RichText::new("贷款金额(万元):").size(16.0));
                            ui.label(egui::RichText::new("贷款利率(%):").size(16.0));
                        });

                        ui.add_space(space);
                        ui.vertical(|ui| {
                            
                            
                            ui.horizontal(|ui| {
                                
                                ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(51,0,105);
                                ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(102,0,205);
                                ui.visuals_mut().widgets.active.weak_bg_fill = egui::Color32::from_rgb(51,0,105);
                                if ui.add_sized([80.0, 19.0], egui::SelectableLabel::new(
                                    self.loan.loan_type == loan::LoanType::EqualInterest,
                                    egui::RichText::new("等额本息").size(15.0),
                                )).clicked() {
                                    self.loan.loan_type = loan::LoanType::EqualInterest;
                                }
                                if ui.add_sized([80.0, 19.0], egui::SelectableLabel::new(
                                    self.loan.loan_type == loan::LoanType::EqualPrincipal,
                                    egui::RichText::new("等额本金").size(15.0),
                                )).clicked() {
                                    self.loan.loan_type = loan::LoanType::EqualPrincipal;
                                }
                            });

                            let temp_space = egui::vec2(10.0, 18.0);
                            ui.spacing_mut().item_spacing = temp_space;
                            // 贷款年限
                            ui.add_sized(egui::vec2(165.0, 21.5),egui::DragValue::new(&mut self.loan.loan_year));
                            // 贷款金额
                            ui.add_sized(egui::vec2(165.0, 21.5),egui::DragValue::new(&mut self.loan.loan_money));
                            // 贷款利率
                            ui.add_sized(egui::vec2(165.0, 21.5),egui::DragValue::new(&mut self.loan.loan_rate));
                        });
                    },
                );


                // 中间部分
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::left_to_right(egui::Align::Min),
                    |ui| {
                        ui.add_space(20.0);
                        ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(0,128,255);
                        ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(0,130,255);
                        ui.visuals_mut().widgets.active.weak_bg_fill = PRIMARY_COLOR_ACTIVE;
                        if ui.add_sized([80.0, 27.0], egui::Button::new(egui::RichText::new("计算").size(14.0))).clicked() {
                            self.loan.calc();
                        }
                        ui.add_space(18.0);
                        ui.visuals_mut().widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(204,0,0);
                        ui.visuals_mut().widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(255,0,0);
                        ui.visuals_mut().widgets.active.weak_bg_fill = PRIMARY_COLOR_ACTIVE;
                        if ui.add_sized([80.0, 27.0], egui::Button::new(egui::RichText::new("重置").size(14.0))).clicked() {
                            self.loan.reset();
                        }
                    },
                );

                // 下半部分
                ui.allocate_ui_with_layout(
                    ui.available_size(),
                    egui::Layout::left_to_right(egui::Align::Min),
                    |ui| {
                        ui.vertical(|ui| {
                            // 竖直方向上标签间距

                            //ui.spacing_mut().item_spacing = temp_space;
                            ui.label(egui::RichText::new("月均还款(万元):").size(16.0));
                            ui.label(egui::RichText::new("利息金额(万元):").size(16.0));
                            ui.label(egui::RichText::new("贷款金额(万元):").size(16.0));
                        });

                        ui.vertical(|ui| {
                            let temp_space = egui::vec2(10.0, 20.0);
                            ui.spacing_mut().item_spacing = temp_space;


                            let money_per_month = &self.loan.money_per_month;
                            let total_interest = &self.loan.total_interest;
                            let total_money = &self.loan.total_money;
                            ui.visuals_mut().widgets.active.bg_fill = egui::Color32::from_rgb(60, 60, 60); // 设置背景颜色
                            ui.visuals_mut().widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 60, 60); // 设置背景颜色
                            ui.visuals_mut().widgets.noninteractive.bg_fill = egui::Color32::from_rgb(60, 60, 60); // 设置背景颜色
                            ui.visuals_mut().widgets.open.bg_fill = egui::Color32::from_rgb(60, 60, 60); // 设置背景颜色
                            ui.add_sized(
                                [165.0, 21.5],
                                egui::Label::new(
                                    egui::RichText::new(money_per_month)
                                        .color(egui::Color32::LIGHT_GREEN)
                                        .size(14.0)
                                        
                                )
                                .wrap(true),
                            );
                            ui.add_sized(
                                [165.0, 21.5],
                                egui::Label::new(
                                    egui::RichText::new(total_interest)
                                        .color(egui::Color32::LIGHT_GREEN)
                                        .size(14.0),
                                )
                                .wrap(true),
                            );
                            ui.add_sized(
                                [165.0, 21.5],
                                egui::Label::new(
                                    egui::RichText::new(total_money)
                                        .color(egui::Color32::LIGHT_GREEN)
                                        .size(14.0),
                                )
                                .wrap(true),
                            );
                        });
                    },
                );
            },
        );
    }

    fn settings_ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .max_height(ui.available_height())
            // .auto_shrink([false;2])
            .show(ui, |ui| {
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
            let menu_size = egui::vec2(165.0, ui.available_height());
            let button_fill = ui.visuals().widgets.inactive.weak_bg_fill;

            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    menu_size,
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        self.draw_menu_line(ctx, ui);
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
                                egui::Button::new(egui::RichText::new(menu).size(14.0)),
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

                egui::Frame::group(ui.style())
                    .multiply_with_opacity(0.1)
                    .show(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            egui::Layout::top_down(egui::Align::Min),
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
