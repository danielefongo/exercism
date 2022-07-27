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
    for input in inputs {
        match input {
            CalculatorInput::Value(val) => stack.push(val.clone()),
            CalculatorInput::Add => apply_op(&mut stack, |a, b| a + b)?,
            CalculatorInput::Subtract => apply_op(&mut stack, |a, b| a - b)?,
            CalculatorInput::Multiply => apply_op(&mut stack, |a, b| a * b)?,
            CalculatorInput::Divide => apply_op(&mut stack, |a, b| a / b)?,
        }
    }

    if stack.len() != 1 {
        None
    } else {
        stack.pop()
    }
}

pub fn apply_op(stack: &mut Vec<i32>, fun: impl FnOnce(i32, i32) -> i32) -> Option<()> {
    let v2 = stack.pop()?;
    let v1 = stack.pop()?;
    stack.push(fun(v1, v2));
    Some(())
}
