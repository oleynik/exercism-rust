#[derive(Clone, Copy)]
enum Token {
    ARGUMENT,
    POWER,
    OPERATOR,
    CALCULATION,
}

pub fn answer(command: &str) -> Option<i32> {
    const WHAT_IS: &str = "What is";
    const QUESTION_MARK: &str = "?";
    const EMPTY: &str = "";
    const SPACE: &str = " ";
    if !command.trim().starts_with(WHAT_IS) || !command.trim().ends_with(QUESTION_MARK) {
        return None;
    }
    let mut input = String::from(
        command
            .replace(WHAT_IS, EMPTY)
            .replace(QUESTION_MARK, EMPTY)
            .trim(),
    );
    let mut next = Token::ARGUMENT;
    let mut skip = 0;
    let mut stack = vec![];
    let mut command: Option<&str> = None;
    loop {
        (next, skip) = match next {
            Token::ARGUMENT => {
                match input.split_whitespace().next() {
                    Some(s) => stack.push(s.parse::<i32>().ok()?),
                    None => return None,
                };
                (Token::CALCULATION, 1)
            }
            Token::OPERATOR => match input.split_whitespace().next() {
                Some("plus") => {
                    command = Some("plus");
                    (Token::ARGUMENT, 1)
                }
                Some("minus") => {
                    command = Some("minus");
                    (Token::ARGUMENT, 1)
                }
                Some("multiplied") => {
                    command = Some("multiplied");
                    (Token::ARGUMENT, 2)
                }
                Some("divided") => {
                    command = Some("divided");
                    (Token::ARGUMENT, 2)
                }
                Some("raised") => {
                    command = Some("raised");
                    (Token::POWER, 3)
                }
                Some(_) => return None,
                None => return stack.pop(),
            },
            Token::POWER => {
                stack.push(
                    input
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse::<i32>()
                        .ok()?,
                );
                (Token::CALCULATION, 2)
            }
            Token::CALCULATION => {
                if stack.len() == 2 && command.is_some() {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    match command.unwrap() {
                        "plus" => stack.push(a + b),
                        "minus" => stack.push(a - b),
                        "multiplied" => stack.push(a * b),
                        "divided" => stack.push(a / b),
                        "raised" => stack.push(a.pow(b as u32)),
                        c => panic!("Unknown command: {}", c),
                    }
                    command = None;
                }
                (Token::OPERATOR, 0)
            }
        };
        input = input
            .split_whitespace()
            .skip(skip)
            .collect::<Vec<&str>>()
            .join(SPACE);
    }
}
