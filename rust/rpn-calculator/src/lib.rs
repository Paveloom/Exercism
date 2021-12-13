#[derive(Debug, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

macro_rules! evaluate {
    ($stack:ident, $expr:tt) => {
        if let Some(CalculatorInput::Value(b)) = $stack.pop() {
            if let Some(CalculatorInput::Value(a)) = $stack.pop() {
                $stack.push(CalculatorInput::Value(a $expr b));
            } else {
                return None
            }
        } else {
            return None;
        }
    };
}

#[must_use]
pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut input_stack = inputs.to_vec();
    let mut output_stack = Vec::<CalculatorInput>::new();
    input_stack.reverse();
    while let Some(input) = input_stack.pop() {
        match input {
            CalculatorInput::Add => evaluate!(output_stack, +),
            CalculatorInput::Subtract => evaluate!(output_stack, -),
            CalculatorInput::Multiply => evaluate!(output_stack, *),
            CalculatorInput::Divide => evaluate!(output_stack, /),
            CalculatorInput::Value(_) => output_stack.push(input),
        }
    }
    if output_stack.len() > 1 {
        None
    } else if let Some(CalculatorInput::Value(result)) = output_stack.pop() {
        Some(result)
    } else {
        None
    }
}
