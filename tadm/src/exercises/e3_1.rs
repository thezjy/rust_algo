// TODO: recursive solution

pub fn first_unbalanced_paren(s: &str) -> Option<usize> {
    let mut parens = vec![];
    let mut unmatched_left_positions = vec![];

    for (i, c) in s.chars().enumerate() {
        match c {
            '(' => {
                unmatched_left_positions.push(i);
                parens.push(c);
            }
            ')' => {
                unmatched_left_positions.pop();
                match parens.pop() {
                    Some(last_char) => {
                        if last_char != '(' {
                            return Some(i);
                        }
                    }
                    None => {
                        return Some(i);
                    }
                }
            }
            _ => {}
        }
    }

    if parens.len() == 0 {
        return None;
    } else {
        return Some(unmatched_left_positions[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balanced() {
        assert_eq!(first_unbalanced_paren(""), None);
        assert_eq!(first_unbalanced_paren("()"), None);
        assert_eq!(first_unbalanced_paren("(     )"), None);
        assert_eq!(first_unbalanced_paren("((()))"), None);
        assert_eq!(first_unbalanced_paren("((())())()"), None);
    }

    #[test]
    fn unbalanced() {
        assert_eq!(first_unbalanced_paren("()((()"), Some(2));
        assert_eq!(first_unbalanced_paren("()abc((()"), Some(5));
        assert_eq!(first_unbalanced_paren("("), Some(0));
        assert_eq!(first_unbalanced_paren("(()"), Some(0));
        assert_eq!(first_unbalanced_paren("())"), Some(2));
        assert_eq!(first_unbalanced_paren("(("), Some(0));
        assert_eq!(first_unbalanced_paren(")()("), Some(0));
    }
}
