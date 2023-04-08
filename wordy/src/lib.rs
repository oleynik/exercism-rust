enum Token {
    ARGUMENT,
    COMMAND,
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
    let mut stack = vec![];
    let mut command: Option<&str> = None;
    loop {
        next = match next {
            Token::ARGUMENT => {
                let argument = input.split_whitespace().next();
                match argument {
                    Some(s) => stack.push(s.parse::<i32>().ok()?),
                    None => return None,
                }
                input = input
                    .split_whitespace()
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .join(SPACE);
                if stack.len() == 2 && command.is_some() {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    match command.unwrap() {
                        "plus" => stack.push(a + b),
                        "minus" => stack.push(a - b),
                        "multiplied" => stack.push(a * b),
                        "divided" => stack.push(a / b),
                        c => panic!("Unknown command: {}", c),
                    }
                    command = None;
                }
                Token::COMMAND
            }
            Token::COMMAND => {
                let com = input.split_whitespace().next();
                let skip = match com {
                    Some("plus") => {
                        command = Some("plus");
                        1
                    }
                    Some("minus") => {
                        command = Some("minus");
                        1
                    }
                    Some("multiplied") => {
                        command = Some("multiplied");
                        2
                    }
                    Some("divided") => {
                        command = Some("divided");
                        2
                    }
                    Some(_) => return None,
                    None => return stack.pop(),
                };
                input = input
                    .split_whitespace()
                    .skip(skip)
                    .collect::<Vec<&str>>()
                    .join(SPACE);
                Token::ARGUMENT
            }
        }
    }
}
