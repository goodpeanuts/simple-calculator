use eframe::egui::*;


#[derive(Default)]
pub struct UiState{
    pub page: Nav,
    pub at: AnimationTime,
    pub current_menu_line_rect: Option<Rect>,
}


#[derive(Default,Debug,PartialEq)]
pub enum Nav{
    #[default]
    Standard,
    Loan,
    Settings,
}

impl Nav {
    pub fn to_str(&self) -> &str{
        match self{
            Nav::Standard => {
                "standard"
            }
            Nav::Loan => {
                "loan"
            },
            Nav::Settings => {
                "settings"
            },
        }
    }
}

#[derive(Default, Debug)]
pub struct AnimationTime{
    pub menu_change: f32,
}

impl AnimationTime {
    pub fn reset_menu_animate_time(&mut self){
        self.menu_change = 1.0;
    }
}