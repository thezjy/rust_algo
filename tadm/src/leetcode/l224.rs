#[derive(Debug)]
struct Operator {
    in_priority: u8,
    out_priority: u8,
    calc: Calc,
}

#[derive(Debug)]
enum Calc {
    LeftParen,
    Binary(fn(i32, i32) -> i32),
    Unary(fn(i32) -> i32),
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn neg(a: i32) -> i32 {
    -a
}
fn mul(a: i32, b: i32) -> i32 {
    a * b
}
fn div(a: i32, b: i32) -> i32 {
    a / b
}
fn pow(a: i32, b: i32) -> i32 {
    a.pow(b as u32)
}

fn parse_op(c: &char, is_sub: bool) -> Operator {
    match c {
        '+' => Operator {
            in_priority: 1,
            out_priority: 1,
            calc: Calc::Binary(add),
        },
        '-' => {
            if is_sub {
                Operator {
                    in_priority: 1,
                    out_priority: 1,
                    calc: Calc::Binary(sub),
                }
            } else {
                Operator {
                    in_priority: 5,
                    out_priority: 5,
                    calc: Calc::Unary(neg),
                }
            }
        }
        '*' => Operator {
            in_priority: 2,
            out_priority: 2,
            calc: Calc::Binary(mul),
        },
        '/' => Operator {
            in_priority: 2,
            out_priority: 2,
            calc: Calc::Binary(div),
        },
        '^' => Operator {
            in_priority: 3,
            out_priority: 4,
            calc: Calc::Binary(pow),
        },
        '(' => Operator {
            in_priority: 0,
            out_priority: 6,
            calc: Calc::LeftParen,
        },
        _ => panic!("invalid char: {c:?}"),
    }
}

fn do_calc(calc: Calc, operands: &mut Vec<i32>) {
    match calc {
        Calc::LeftParen => {}
        Calc::Binary(calc) => {
            let b = operands.pop().unwrap();
            let a = operands.pop().unwrap();
            operands.push(calc(a, b));
        }
        Calc::Unary(calc) => {
            let a = operands.pop().unwrap();
            operands.push(calc(a));
        }
    }
}

pub fn calculate(s: String) -> i32 {
    let mut operators = vec![Operator {
        in_priority: 0,
        out_priority: 6,
        calc: Calc::LeftParen,
    }];
    let mut operands = vec![];

    let mut prev_num = None;
    let mut next_is_sub = false;

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
                    next_is_sub = true;
                    operands.push(num);
                    prev_num = None;
                }

                if !c.is_ascii_whitespace() {
                    if c == ')' {
                        next_is_sub = true;
                        while let Some(operator) = operators.pop() {
                            match operator.calc {
                                Calc::LeftParen => {
                                    break;
                                }
                                _ => {
                                    do_calc(operator.calc, &mut operands);
                                }
                            }
                        }
                    } else {
                        let operator = parse_op(&c, next_is_sub);
                        next_is_sub = false;

                        while let Some(last_operator) = operators.last() {
                            if (last_operator.in_priority < operator.out_priority) {
                                break;
                            }

                            if let Some(operator) = operators.pop() {
                                match operator.calc {
                                    Calc::LeftParen => {
                                        break;
                                    }
                                    _ => {
                                        do_calc(operator.calc, &mut operands);
                                    }
                                }
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

    #[test]
    fn more_ops() {
        assert_eq!(calculate("3+4*((4+6)^2)/2 ".to_string()), 203);
    }
}
