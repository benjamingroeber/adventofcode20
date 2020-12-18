use helpers::read_file;
use std::error::Error;
use std::iter::Peekable;
use thiserror::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file("./assets/days/day18.txt")?;
    // Part 1
    // Before you can help with the homework, you need to understand it yourself. Evaluate the
    // expression on each line of the homework; what is the sum of the resulting values?
    let mut sum = 0;
    for equation in input.lines() {
        let mut chars = equation.chars().peekable();
        let tokens = parse_tokens(&mut chars)?;
        sum += evaluate_with_same_precedence(&tokens);
    }
    println!("Part 1: {}", sum);
    Ok(())
}

type Unit = u64;

fn evaluate_with_same_precedence(tokens: &[Token]) -> Unit {
    let mut value = 0;
    let mut operator = Operator::Add;
    for t in tokens {
        match t {
            Token::Value(v) => match operator {
                Operator::Add => value += v,
                Operator::Mult => value *= v,
            },
            Token::Operator(op) => operator = *op,
            Token::Par(values) => {
                let v = evaluate_with_same_precedence(values);
                match operator {
                    Operator::Add => value += v,
                    Operator::Mult => value *= v,
                }
            }
        }
    }
    value
}

#[derive(Copy, Clone, Debug)]
enum Operator {
    Add,
    Mult,
}

#[derive(Clone, Debug)]
enum Token {
    Value(Unit),
    Operator(Operator),
    Par(Vec<Token>),
}

fn parse_tokens<T: Iterator<Item = char>>(
    iter: &mut Peekable<T>,
) -> Result<Vec<Token>, OperationError> {
    let mut tokens = Vec::new();
    while let Some(&next) = iter.peek() {
        println!("{}", next);
        let t = match iter.next().unwrap() {
            digit @ '0'..='9' => {
                let mut number = digit
                    .to_digit(10)
                    .ok_or_else(|| OperationError::Tokenizer(digit.to_string()))?;
                // read whole number
                while iter.peek().map(|c| c.is_numeric()).unwrap_or(false) {
                    let nex = iter.next().unwrap();
                    let next = nex
                        .to_digit(10)
                        .ok_or_else(|| OperationError::Tokenizer(nex.to_string()))?;
                    println!(" {}", next);
                    number = number * 10 + next;
                }
                Some(Token::Value(number as Unit))
            }
            '+' => Some(Token::Operator(Operator::Add)),
            '*' => Some(Token::Operator(Operator::Mult)),
            // recurse
            '(' => Some(Token::Par(parse_tokens(iter)?)),
            // go up
            ')' => {
                return Ok(tokens);
            }
            ' ' => None,
            _ => unimplemented!(),
        };
        if let Some(token) = t {
            tokens.push(token);
        }
    }
    println!("{:?}", tokens);
    Ok(tokens)
}

#[derive(Clone, Debug, Error)]
enum OperationError {
    #[error("Could not tokenize {0}")]
    Tokenizer(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precedence() {
        let input = "100 + (2 * 3) + (4 * (5 + 6))";
        // no error
        // For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are as follows:
        // 1 + 2 * 3 + 4 * 5 + 6
        //   3   * 3 + 4 * 5 + 6
        //       9   + 4 * 5 + 6
        //          13   * 5 + 6
        //              65   + 6
        //                  71
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let mut chars = input.chars().peekable();
        let straight = parse_tokens(&mut chars).unwrap();

        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let mut chars = input.chars().peekable();
        let parentheses = parse_tokens(&mut chars).unwrap();
        // Parentheses can override this order; for example, here is what happens if parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
        //
        // 1 + (2 * 3) + (4 * (5 + 6))
        // 1 +    6    + (4 * (5 + 6))
        //      7      + (4 * (5 + 6))
        //      7      + (4 *   11   )
        //      7      +     44
        //             51
        assert_eq!(71, evaluate_with_same_precedence(&straight));
        assert_eq!(51, evaluate_with_same_precedence(&parentheses));
    }
}
