
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
    pub money_per_month: f32,
    pub total_interest: f32,
    pub total_money: f32,
}