use crate::parser::{Instruction, InstructionTypes};
use std::iter::Iterator;

struct State {
    array: Vec<u8>,
    pointer: usize,
}

impl State {
    fn increment(&mut self) {
        self.array[self.pointer] = self.array[self.pointer].overflowing_add(1).0;
    }

    fn decrement(&mut self) {
        self.array[self.pointer] = self.array[self.pointer].overflowing_sub(1).0;
    }

    fn ptr_increment(&mut self) {
        self.pointer += 1;
        self.pointer %= self.array.len();
    }

    fn ptr_decrement(&mut self) {
        self.pointer += self.array.len() - 1;
        self.pointer %= self.array.len();
    }

    fn get(&mut self) -> u8 {
        self.array[self.pointer]
    }

    fn set(&mut self, value: u8) {
        self.array[self.pointer] = value;
    }
}

fn process_instructions<T: Iterator<Item = u8>>(
    tree: &[Instruction],
    result: &mut Vec<u8>,
    state: &mut State,
    stdin: &mut T,
) {
    for (_, inst) in tree {
        match &inst {
            InstructionTypes::Increment => state.increment(),
            InstructionTypes::Decrement => state.decrement(),
            InstructionTypes::PointerIncrement => state.ptr_increment(),
            InstructionTypes::PointerDecrement => state.ptr_decrement(),
            InstructionTypes::GetChar => state.set(stdin.next().unwrap_or(0)),
            InstructionTypes::PutChar => result.push(state.get()),
            InstructionTypes::While(inside) => {
                while state.get() != 0 {
                    process_instructions(inside, result, state, stdin);
                }
            }
        }
    }
}

pub fn execute(tree: &[Instruction], array_size: usize, stdin: &str) -> String {
    let mut result = Vec::new();
    let mut state = State {
        array: vec![0; array_size],
        pointer: 0,
    };
    process_instructions(tree, &mut result, &mut state, &mut stdin.bytes());
    result.iter().map(|&x| x as char).collect()
}
