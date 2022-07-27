use Operator::*;

#[derive(Debug)]
enum Operator {
    LeftParen,
    Add,
    Sub,
    Multi,
    Divide,
}

impl Operator {
    fn inside_stack_priority(&self) -> u8 {
        match self {
            LeftParen => 0,
            Add => 1,
            Sub => 1,
            Multi => 2,
            Divide => 2,
        }
    }

    fn outside_stack_priority(&self) -> u8 {
        match self {
            LeftParen => 3,
            Add => 1,
            Sub => 1,
            Multi => 2,
            Divide => 2,
        }
    }
}

pub fn calculate(s: String) -> i32 {
    let mut operators = vec![LeftParen];
    let mut operands = vec![];

    let mut prev_num = None;

    s.as_bytes()
        .iter()
        .chain([')' as u8].iter())
        .for_each(|&b| {
            let c = b as char;

            if let Some(d) = c.to_digit(10) {
                if let Some(num) = prev_num {
                    prev_num = Some(num * 10 + d as i32);
                } else {
                    prev_num = Some(d as i32);
                }
            } else {
                if let Some(num) = prev_num {
                    operands.push(num);
                    prev_num = None;
                }

                if !c.is_ascii_whitespace() {
                    if c == ')' {
                        while let Some(operator) = operators.pop() {
                            match operator {
                                Operator::LeftParen => {
                                    break;
                                }
                                Operator::Add => {
                                    let b = operands.pop().unwrap();
                                    let a = operands.pop().unwrap();
                                    operands.push(a + b);
                                }
                                Operator::Sub => {
                                    let b = operands.pop().unwrap();
                                    let a = operands.pop().unwrap();
                                    operands.push(a - b);
                                }
                                Operator::Multi => {
                                    let b = operands.pop().unwrap();
                                    let a = operands.pop().unwrap();
                                    operands.push(a * b);
                                }
                                Operator::Divide => {
                                    let b = operands.pop().unwrap();
                                    let a = operands.pop().unwrap();
                                    operands.push(a / b);
                                }
                            }
                        }
                    } else {
                        let operator = match c {
                            '*' => Multi,
                            '/' => Divide,
                            '-' => Sub,
                            '+' => Add,
                            '(' => LeftParen,
                            _ => panic!("invalid token {c:?}"),
                        };

                        loop {
                            if let Some(last_operator) = operators.last() {
                                if last_operator.inside_stack_priority()
                                    < operator.outside_stack_priority()
                                {
                                    break;
                                }

                                if let Some(operator) = operators.pop() {
                                    match operator {
                                        Operator::LeftParen => {}
                                        Operator::Add => {
                                            let b = operands.pop().unwrap();
                                            let a = operands.pop().unwrap();
                                            operands.push(a + b);
                                        }
                                        Operator::Sub => {
                                            let b = operands.pop().unwrap();
                                            let a = operands.pop().unwrap();
                                            operands.push(a - b);
                                        }
                                        Operator::Multi => {
                                            let b = operands.pop().unwrap();
                                            let a = operands.pop().unwrap();
                                            operands.push(a * b);
                                        }
                                        Operator::Divide => {
                                            let b = operands.pop().unwrap();
                                            let a = operands.pop().unwrap();
                                            operands.push(a / b);
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                        }

                        operators.push(operator);
                    }
                }
            }
        });

    operands[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(calculate("1--(2 + 3)".to_string()), 6);
        assert_eq!(calculate("1-(2 + 3)".to_string()), -4);
        assert_eq!(calculate("1 + 1".to_string()), 2);
        assert_eq!(calculate(" 2-1 + 2 ".to_string()), 3);
        assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
    }
}
