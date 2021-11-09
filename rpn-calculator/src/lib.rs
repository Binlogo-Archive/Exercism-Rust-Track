#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];
    for item in inputs {
        match item {
            CalculatorInput::Add => {
                if let (Some(val1), Some(val2)) = (stack.pop(), stack.pop()) {
                    stack.push(val1 + val2);
                }
            }
            CalculatorInput::Subtract => {
                if let (Some(val1), Some(val2)) = (stack.pop(), stack.pop()) {
                    stack.push(val2 - val1);
                }
            }
            CalculatorInput::Multiply => {
                if let (Some(val1), Some(val2)) = (stack.pop(), stack.pop()) {
                    stack.push(val1 * val2);
                }
            }
            CalculatorInput::Divide => {
                if let (Some(val1), Some(val2)) = (stack.pop(), stack.pop()) {
                    stack.push(val2 / val1);
                }
            }
            CalculatorInput::Value(val) => stack.push(*val),
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
