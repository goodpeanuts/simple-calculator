pub trait Weight {
    // 操作权重。确定彼此之间操作的优先级。
    // 权重最高的操作具有最高优先级.
    fn weight(&self) -> u8;
}

#[derive(Clone)]
pub enum Func {
    Sin,
    Cos,
    Tg,
    Ctg,
    Sqrt,
}

impl Weight for Func {
    fn weight(&self) -> u8 { 4 }
}

impl std::fmt::Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Func::Sin => { "sin" }
                Func::Cos => { "cos" }
                Func::Tg => { "tg" }
                Func::Ctg => { "ctg" }
                Func::Sqrt => { "√" }
            }
        )
    }
}

impl TryFrom<&str> for Func {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "sin" => Ok(Func::Sin),
            "cos" => Ok(Func::Cos),
            "tg" => Ok(Func::Tg),
            "ctg" => Ok(Func::Ctg),
            "√" => Ok(Func::Sqrt),
            _ => Err(())
        }
    }
}

// 代数运算.
#[derive(Clone)]
pub enum Op {
    // 加法 - 对应于 + 号.
    Add,
    // 减法 - 对应于 - 号.
    Sub,
    // 乘法 - 对应于 * 号.
    Multi,
    // 除号 - 对应于 / 号.
    Div,
    // 求幂 - 对应于 ^ 号.
    Exp,
    // 计算区域限制符号.
    ParenLeft,
    ParenRight,
}

impl Weight for Op {
    fn weight(&self) -> u8 {
        match self {
            Op::Add | Op::Sub => { 1 }
            Op::Multi | Op::Div => { 2 }
            Op::Exp => { 3 }
            Op::ParenRight | Op::ParenLeft => { 0 }
        }
    }
}


impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Add => { "+" }
                Op::Sub => { "-" }
                Op::Multi => { "*" }
                Op::Div => { "/" }
                Op::Exp => { "^" }
                Op::ParenLeft => { "(" }
                Op::ParenRight => { ")" }
            }
        )
    }
}


impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "^" => Ok(Op::Exp),
            "/" => Ok(Op::Div),
            "*" => Ok(Op::Multi),
            "-" => Ok(Op::Sub),
            "+" => Ok(Op::Add),
            ")" => Ok(Op::ParenRight),
            "(" => Ok(Op::ParenLeft),
            _ => Err(())
        }
    }
}

#[derive(Clone)]
pub enum Token {
    // 功能单一.
    Function(Func),
    // 数字运算.
    Operation(Op),
    // 数字（实数）.
    Operand(f64),
}

impl TryFrom<&str> for Token {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(o) = Op::try_from(s) {
            Ok(Token::Operation(o))
        } else if let Ok(f) = Func::try_from(s) {
            Ok(Token::Function(f))
        } else if let Ok(val) = s.parse::<f64>() {
            if val.is_infinite() {
                Err(())
            } else { Ok(Token::Operand(val)) }
        } else { Err(()) }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Function(func) => { func.to_string() }
                Token::Operation(op) => { op.to_string() }
                Token::Operand(o) => { o.to_string() }
            }
        )
    }
}