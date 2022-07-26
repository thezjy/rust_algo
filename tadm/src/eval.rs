use std::{collections::HashMap, fmt::Binary};

use Associativity::*;
use Calculate::*;
use Symbol::*;
use Token::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Calculate {
    Unary(fn(u32) -> u32),
    Binary(fn(u32, u32) -> u32),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Associativity {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Operator(Calculate, Associativity, u8),
    LeftParen,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Operand(u32),
    Sym(Symbol),
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}

fn multi(a: u32, b: u32) -> u32 {
    a * b
}

fn divide(a: u32, b: u32) -> u32 {
    a / b
}

fn pow(a: u32, b: u32) -> u32 {
    a ^ b
}

pub fn calculate(s: String) -> i32 {
    let symbol_map: HashMap<char, Symbol> = HashMap::from_iter([
        ('+', Operator(Binary(add), Left, 2)),
        ('-', Operator(Binary(sub), Left, 2)),
        ('*', Operator(Binary(multi), Left, 3)),
        ('/', Operator(Binary(sub), Left, 3)),
        ('^', Operator(Binary(pow), Right, 4)),
        ('(', LeftParen),
    ]);

    let mut output = vec![];
    let mut operators = vec![];

    let mut current_num: Option<u32> = None;

    s.as_bytes().iter().for_each(|&c| {
        let c = c as char;

        if let Some(d) = c.to_digit(10) {
            if let Some(num) = current_num {
                current_num = Some(d + num * 10);
            } else {
                current_num = Some(d);
            }
        } else {
            if let Some(num) = current_num {
                output.push(Operand(num));
                current_num = None;
            }

            if !c.is_ascii_whitespace() {
                match c {
                    '+' | '-' | '*' | '/' | '(' | '^' => {
                        let symbol = symbol_map.get(&c).unwrap();

                        match symbol {
                            LeftParen => operators.push(symbol),
                            Operator(_, associativity, precedence) => {
                                if let Some(last_op) = operators.last() {
                                    match last_op {
                                        LeftParen => {
                                            operators.push(symbol);
                                        }
                                        Operator(_, _, last_precedence) => {
                                            if (precedence > last_precedence
                                                || matches!(associativity, Right))
                                            {
                                                dbg!(&symbol);
                                                operators.push(symbol);
                                            } else {
                                                let popped_last_op = operators.pop().unwrap();

                                                output.push(Sym(popped_last_op.clone()));
                                                operators.push(symbol);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    ')' => {
                        while let Some(s) = operators.pop() {
                            if *s == LeftParen {
                                break;
                            } else {
                                output.push(Sym(s.clone()));
                            }
                        }
                    }
                    c => panic!("unknow token: {c:?}!"),
                };
            }
        }
    });

    dbg!(&operators);
    dbg!(&output);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            calculate("321    + 4 * 2 / ( 1 - 5) ^ 2 ^ 3 ".to_string()),
            23
        );
        // assert_eq!(calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
        // assert_eq!(calculate(" 2-1 + 2 ".to_string()), 3);
        // assert_eq!(calculate("1 + 1".to_string()), 2);
    }
}
