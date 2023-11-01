use crate::token;
use crate::token::Weight;

pub struct MathExp {
    tokens: Vec<token::Token>,
    buffer: String,
    output: String,
}

impl Default for MathExp {
    fn default() -> Self {
        Self::new()
    }
}

impl MathExp {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            buffer: String::new(),
            output: String::new(),
        }
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    fn update_output(&mut self, s: &str) {
        self.output = s.to_string();
    }


    // 插入运算符需遵循以下规则：
    
    // 操作数 后必须紧跟操作符号或闭括号
    // 操作符 后只能是开括号或操作数（数字）

    fn push_to_token(&mut self, t: token::Token) {
        fn push(tokens: &mut Vec<token::Token>, t: token::Token) {
            if let token::Token::Function(_) = t {
                tokens.push(t);
                tokens.push(token::Token::Operation(token::Op::ParenLeft));
            } else { tokens.push(t); }
        }

        // 如果插入后闭括号的数量将超过开括号的数量，则禁止插入闭括号。
        if let token::Token::Operation(token::Op::ParenRight) = t {
            let mut count_paren: i32 = -1; // // 设置为-1，因为在将来我们想要插入一个括号.
            for token in &self.tokens {
                if let token::Token::Operation(p) = token {
                    match p {
                        token::Op::ParenLeft => { count_paren += 1 }
                        token::Op::ParenRight => { count_paren -= 1 }
                        _ => {}
                    }
                }
            }
            if count_paren < 0 { return; }
        }


        let last_token = self.tokens.last();
        if last_token.is_none() {
            // 当token列表为空时，
            // 我们只允许插入新令牌
            // 如果它们不是操作的标记（左括号除外）。
            if !matches!(t,token::Token::Operation(_)) || matches!(t,token::Token::Operation(token::Op::ParenLeft)) {
                push(&mut self.tokens, t);
            }
            return;
        }
        let last_token = last_token.unwrap();


        let allow_insert = match last_token {
            // 数字后：
            token::Token::Operand(_) => {
                match t {
                    // 防止在数字后插入函数、数字或左括号。
                    token::Token::Function(_) | token::Token::Operand(_) | token::Token::Operation(token::Op::ParenLeft) => { false }
                    _ => { true }
                }
            }
            // 在右括号之后：
            token::Token::Operation(token::Op::ParenRight) => {
                match t {
                    token::Token::Operation(token::Op::ParenLeft) => { false }
                    token::Token::Operation(_) => { true }
                    _ => { false }
                }
            }
            // 除右括号外的操作之后：
            token::Token::Operation(_) => {
                match t {
                    token::Token::Operation(token::Op::ParenLeft) => { true }
                    // 防止在运算后插入运算（左括号异常）。
                    token::Token::Operation(_) => { false }
                    _ => { true }
                }
            }

            _ => { true }
        };

        if allow_insert {
            push(&mut self.tokens, t);
        } else {
            self.update_output(format!("Токен {} не может быть добавлен после {}", t, last_token).as_str());
        }
        let mut s = String::new();
        for token in &self.tokens {
            s.push('[');
            s.push_str(token.to_string().as_str());
            s.push(']');
            s.push(',');
        }
    }

    // 从缓冲区中删除该值并将其放置在标记向量的末尾。
    // 将尝试转换存储在缓冲区中的值。
  
    fn pop_buffer(&mut self) -> bool {
        if self.buffer.is_empty() { return true; }
        if let Ok(val) = self.buffer.parse::<f64>() {
            if val.is_sign_negative() {
                self.tokens.push(token::Token::Operation(token::Op::ParenLeft));
                self.tokens.push(token::Token::Operand(val));
                self.tokens.push(token::Token::Operation(token::Op::ParenRight));
                self.buffer.clear();
            } else {
                self.tokens.push(token::Token::Operand(val));
                self.buffer.clear();
            }
            true
        } else { false }
    }

    // 从标记向量中删除最后一个值。
    pub fn pop(&mut self) {
        if self.buffer.is_empty() {
            self.tokens.pop();
            // 如果删除的 token 之后还有一个 function token，那么也将其删除.
            if matches!(self.tokens.last(), Some(token::Token::Function(_))) { self.tokens.pop(); }
        } else {
            // 如果缓冲区中有值，那么首先从中删除这些值.
            self.buffer.pop();
        }
    }

    // 清除缓冲区和带有标记的向量.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.tokens.clear();
    }

    // 使用指定的字符串创建并添加新标记。
    // 对于任何添加，都会首先检查必要性，添加一行到缓冲区。
    // 如果到达可以解释的字符序列
    // 作为操作或函数，将尝试挤出当前值
    // 从缓冲区中取出，然后才会添加新值。
    pub fn add(&mut self, s: &str) -> bool {
        let allow_number_input = !matches!(
            self.tokens.last(),
            Some(token::Token::Operation(token::Op::ParenRight))
        );

        if s == "." && allow_number_input {
            if self.buffer.is_empty() {
                self.buffer = "0.".to_string();
                true
            } else if self.buffer.contains('.') {
                // мы не можем разрешить добавить больше чем одну точку.
                false
            } else {
                self.buffer.push('.');
                true
            }
        } else if s.parse::<u8>().is_ok() && allow_number_input {
            self.buffer.push_str(s);
            true
        } else if self.buffer.is_empty() && s == "-" && allow_number_input {
            self.buffer = s.to_string();
            true
        } else if let Ok(t) = token::Token::try_from(s) {
            self.pop_buffer();
            self.push_to_token(t);
            true
        } else { false }
    }

    pub fn calculate(&mut self) {
        self.pop_buffer();
        let rpn = yard(&self.tokens);
        match rpn {
            Err(e) => { self.output = e }
            Ok(tokens) => {
                let mut stack: Vec<token::Token> = Vec::new();
                for t in tokens {
                    if stack.is_empty() {
                        stack.push(t);
                    } else {
                        match t {
                            token::Token::Function(f) => {
                                if let Some(token::Token::Operand(val)) = stack.pop() {
                                    stack.push(match f {
                                        token::Func::Sin => {
                                            token::Token::Operand(val.sin())
                                        }
                                        token::Func::Cos => {
                                            token::Token::Operand(val.cos())
                                        }
                                        token::Func::Tg => {
                                            token::Token::Operand(val.sin() / val.cos())
                                        }
                                        token::Func::Ctg => {
                                            token::Token::Operand(val.cos() / val.sin())
                                        }
                                        token::Func::Sqrt => {
                                            token::Token::Operand(val.sqrt())
                                        }
                                    });
                                } else {
                                    self.output = "Ошибка вычисления".to_string()
                                }
                            }
                            token::Token::Operation(op) => {
                                if let Some(token::Token::Operand(second_val)) = stack.pop() {
                                    if let Some(token::Token::Operand(first_val)) = stack.pop() {
                                        if let Some(val) = match op {
                                            token::Op::Add => {
                                                Some(first_val + second_val)
                                            }
                                            token::Op::Sub => {
                                                Some(first_val - second_val)
                                            }
                                            token::Op::Multi => {
                                                Some(first_val * second_val)
                                            }
                                            token::Op::Div => {
                                                Some(first_val / second_val)
                                            }
                                            token::Op::Exp => {
                                                Some(first_val.powf(second_val))
                                            }
                                            _ => { None }
                                        } { stack.push(token::Token::Operand(val)) }
                                    } else { self.output = "Ошибка вычисления".to_string() }
                                } else { self.output = "Ошибка вычисления".to_string() }
                            }
                            token::Token::Operand(_) => { stack.push(t); }
                        }
                    }
                }
                self.output = if let Some(t) = stack.pop() {
                    self.buffer.clear();
                    self.tokens.clear();
                    t.to_string()
                } else { "Ошибка вычисления".to_string() };
            }
        }
    }
}

impl std::fmt::Display for MathExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut o = String::new();
        for token in &self.tokens {
            o.push_str(token.to_string().as_str());
        }
        o.push_str(self.buffer.as_str());
        write!(
            f,
            "{}",
            o
        )
    }
}


/// # 逆波兰表达式算法
///
/// [来自维基百科 - 自由的百科全书](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)
///
/// 逆波兰表达式算法是一种处理以中缀表示的数学表达式的方法。它可以用于以逆波兰表示法或抽象语法树的形式获取输出。
/// 该算法由艾兹赫·迪科斯彻提出，并由他命名为 "逆波兰表达式算法"，因为它类似于铁路的分类站的操作。
///
/// 与以逆波兰表示法计算表达式值类似，该算法也使用堆栈。中缀表示法的数学表达式通常由人类使用，例如：2+4 和 3+6*(3-2)。
/// 转换为逆波兰表示法时使用两个字符串：输入字符串和输出字符串，以及用于存储尚未添加到输出队列中的运算符的堆栈。
/// 在转换过程中，该算法读取一个字符并执行依赖于该字符的操作。

fn yard(input: &Vec<token::Token>) -> Result<Vec<token::Token>, String> {
    let mut output: Vec<token::Token> = vec![];
    let mut stack: Vec<token::Token> = vec![];
    for token in input {
        match token {
            token::Token::Operand(_o) => {
                // 如果 token 是数字，则将其添加到输出队列中。
                output.push(token.clone())
            }
            token::Token::Function(_f) => {
                stack.push(token.clone())
            }
            token::Token::Operation(token::Op::ParenLeft) => {
                stack.push(token.clone())
            }
            token::Token::Operation(token::Op::ParenRight) => {
                loop {
                    if let Some(last_token_in_stack) = stack.pop() {
                        match last_token_in_stack {
                            token::Token::Operation(token::Op::ParenLeft) => {
                                break;
                            }
                            _ => {
                                output.push(last_token_in_stack.clone())
                            }
                        }
                    } else {
                        return Err("В выражении отсутствует скобка.".to_string());
                    }
                }
            }
            token::Token::Operation(op1) => {
                if let Some(token::Token::Operation(op2)) = stack.pop() {
                    if op2.weight() >= op1.weight() {
                        output.push(token::Token::Operation(op2))
                    } else {
                        stack.push(token::Token::Operation(op2))
                    }
                }

                stack.push(token.clone())
            }
        }
    }
    while let Some(last_token_in_stack) = stack.pop() {
        match last_token_in_stack {
            token::Token::Operation(token::Op::ParenLeft) => {
                return Err("缺少括号".to_string());
            }
            _ => { output.push(last_token_in_stack) }
        }
    }
    Ok(output)
}