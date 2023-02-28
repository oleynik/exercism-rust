use std::ops::Fn;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn operate<F>(a: Option<i32>, b: Option<i32>, operator: F) -> Option<i32>
where
    F: Fn(i32, i32) -> i32,
{
    if a == None || b == None {
        return None;
    }
    Some(operator(b.unwrap(), a.unwrap()))
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for x in inputs {
        let match_result = match x {
            CalculatorInput::Value(v) => Some(*v),
            CalculatorInput::Add => operate(stack.pop(), stack.pop(), |a, b| a + b),
            CalculatorInput::Subtract => operate(stack.pop(), stack.pop(), |a, b| a - b),
            CalculatorInput::Multiply => operate(stack.pop(), stack.pop(), |a, b| a * b),
            CalculatorInput::Divide => operate(stack.pop(), stack.pop(), |a, b| a / b),
        };

        match match_result {
            Some(res) => stack.push(res),
            None => return None,
        }
    }

    match stack.len() {
        1 => stack.pop(),
        _ => None,
    }
}
