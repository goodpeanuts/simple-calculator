
#[derive(Default,PartialEq)]
pub enum LoanType{
    #[default]
    EqualInterest,
    EqualPrincipal,
}

#[derive(Default,PartialEq)]
pub struct Loan{
    pub loan_type: LoanType,
    pub loan_year: f32,
    pub loan_money: f32,
    pub loan_rate: f32,
    pub money_per_month: String,
    pub total_interest: String,
    pub total_money: String,
}

impl Loan{
    pub fn new() -> Self{
        Self{
            loan_type: LoanType::EqualInterest,
            loan_year: 0.0,
            loan_money: 0.0,
            loan_rate: 0.0,
            money_per_month: String::new(),
            total_interest: String::new(),
            total_money: String::new(),
        }
    }
    
    pub fn calc(&mut self){
        match self.loan_type{
            LoanType::EqualInterest => {
                self.calc_equal_interest();
            },
            LoanType::EqualPrincipal => {
                self.calc_equal_principal();
            },
        }
    }

    pub fn reset(&mut self){
        self.loan_type = LoanType::EqualInterest;
        self.loan_year = 0.0;
        self.loan_money = 0.0;
        self.loan_rate = 0.0;
        self.money_per_month = String::new();
        self.total_interest = String::new();
        self.total_money = String::new();
    }

    fn calc_equal_interest(&mut self){
        let month = self.loan_year * 12.0;
        let rate = self.loan_rate / 100.0 / 12.0;
        let money = self.loan_money;
        let mut total_interest = 0.0;
        let mut money_per_month = 0.0;
        let mut total_money = 0.0;
        if month > 0.0 && rate > 0.0 && money > 0.0{
            money_per_month = money * rate * (1.0 + rate).powf(month) / ((1.0 + rate).powf(month) - 1.0);
            total_interest = money_per_month * month - money;
            total_money = money_per_month * month;
        }
        self.money_per_month = money_per_month.to_string();
        self.total_interest = total_interest.to_string();
        self.total_money = total_money.to_string();
    }

    fn calc_equal_principal(&mut self){
        let month = self.loan_year * 12.0;
        let rate = self.loan_rate / 100.0 / 12.0;
        let money = self.loan_money;
        let mut total_interest = 0.0;
        let mut money_per_month = 0.0;
        let mut total_money = 0.0;
        if month > 0.0 && rate > 0.0 && money > 0.0{
            money_per_month = money / month + money * rate;
            total_interest = money * rate * (month + 1.0) / 2.0;
            total_money = money + total_interest;
        }
        self.money_per_month = money_per_month.to_string();
        self.total_interest = total_interest.to_string();
        self.total_money = total_money.to_string();
    }

}