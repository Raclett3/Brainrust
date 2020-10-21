use std::iter::Iterator;

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionTypes {
    Increment,
    Decrement,
    PointerIncrement,
    PointerDecrement,
    GetChar,
    PutChar,
    While(SyntaxTree),
}

pub type Instruction = (usize, InstructionTypes);

type SyntaxTree = Vec<Instruction>;

fn parse_from_iter<T: Iterator<Item = (usize, char)>>(
    iter: &mut T,
    is_inside: bool,
) -> Result<SyntaxTree, String> {
    let mut tree: SyntaxTree = vec![];
    while let Some((index, ch)) = iter.next() {
        let inst = match ch {
            ']' => {
                if is_inside {
                    return Ok(tree);
                } else {
                    return Err(format!("Unexpected ']' at {}", index));
                }
            }
            '[' => InstructionTypes::While(parse_from_iter(iter, true)?),
            '+' => InstructionTypes::Increment,
            '-' => InstructionTypes::Decrement,
            '>' => InstructionTypes::PointerIncrement,
            '<' => InstructionTypes::PointerDecrement,
            '.' => InstructionTypes::PutChar,
            ',' => InstructionTypes::GetChar,
            _ => continue,
        };
        tree.push((index, inst));
    }
    if is_inside {
        Err("Unexpected EOF".to_string())
    } else {
        Ok(tree)
    }
}

pub fn parse(source: &str) -> Result<SyntaxTree, String> {
    parse_from_iter(&mut source.chars().enumerate(), false)
}
