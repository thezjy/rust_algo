fn main() {
    let text = " 2-10 + 2 ";

    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in
        text.match_indices(|c: char| c == '(' || c == ')' || c == '+' || c == '-')
    {
        if last != index {
            result.push(text[last..index].trim());
        }
        result.push(matched.trim());
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..].trim());
    }

    println!("{:?}", result);
}
