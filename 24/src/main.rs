use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};

use anyhow::{bail, Result};
use peg;
use text_io::try_read;

peg::parser! {
    grammar program_parser() for str {
        rule value() -> Value
            = n:$("-"?['0'..='9']+) {? n.parse().or(Err("Value")) }

        rule register() -> Register
            = r:$("w" / "x" / "y" / "z") {? match r {
                "w" => Ok(Register::W),
                "x" => Ok(Register::X),
                "y" => Ok(Register::Y),
                "z" => Ok(Register::Z),
                _ => Err("Register"),
            } }

        rule operand() -> Operand
            = r:register() { Operand::Var(r) } / v:value() { Operand::Immediate(v) }
        rule unary_instruction() -> Instruction
            = op:$("inp") " "+ r:register() {? match op {
                "inp" => Ok(Instruction::Inp(r)),
                _ => Err("unary instruction")
            } }

        rule binary_instruction() -> Instruction
            = op:$("add" / "mul" / "div" / "mod" / "eql") " "+ r:register() " "+ o:operand() {? match op {
                "add" => Ok(Instruction::Add(r, o)),
                "mul" => Ok(Instruction::Mul(r, o)),
                "div" => Ok(Instruction::Div(r, o)),
                "mod" => Ok(Instruction::Mod(r, o)),
                "eql" => Ok(Instruction::Eql(r, o)),
                _ => Err("binary instruction")
            } }
        rule instruction() -> Instruction
            = unary_instruction() / binary_instruction()

        pub rule program() -> Vec<Instruction>
            = instruction() ** "\n"
    }
}

pub type Value = i128;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Register {
    W,
    X,
    Y,
    Z,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::W => write!(f, "W"),
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
            Self::Z => write!(f, "Z"),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Var(Register),
    Immediate(Value),
}

#[derive(Debug)]
pub enum Instruction {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

#[derive(Clone, Debug)]
pub struct ALU {
    registers: HashMap<Register, Value>,
}

impl Display for ALU {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} = {}, ", Register::W, self.read(&Register::W))?;
        write!(f, "{} = {}, ", Register::X, self.read(&Register::X))?;
        write!(f, "{} = {}, ", Register::Y, self.read(&Register::Y))?;
        write!(f, "{} = {}", Register::Z, self.read(&Register::Z))
    }
}

impl ALU {
    pub fn new() -> Self {
        let mut registers = HashMap::with_capacity(4);
        registers.insert(Register::W, 0);
        registers.insert(Register::X, 0);
        registers.insert(Register::Y, 0);
        registers.insert(Register::Z, 0);
        registers.shrink_to_fit();
        ALU { registers }
    }

    pub fn execute(&mut self, prog: &[Instruction], inputs: &[Value]) -> Result<()> {
        let input_instructions = Self::count_inputs(prog);
        if inputs.len() != input_instructions {
            bail!(
                "Expected {} inputs, but got {}",
                input_instructions,
                inputs.len()
            )
        }
        let mut input_iter = inputs.iter();
        for instr in prog {
            match instr {
                Instruction::Inp(r) => self.write(r, *input_iter.next().expect("missing input")),
                i => self.execute_arithmetic_instruction(i)?,
            }
        }
        Ok(())
    }

    pub fn execute_interactive(&mut self, prog: &[Instruction]) -> Result<()> {
        for instr in prog {
            match instr {
                Instruction::Inp(r) => {
                    print!("Enter an integer: ");
                    io::stdout().flush()?;
                    let mut input: Result<Value, _> = try_read!("{}");
                    while !input.is_ok() {
                        print!("This is not an integer, try again: ");
                        io::stdout().flush()?;
                        input = try_read!("{}");
                    }
                    let input = input.unwrap();
                    self.write(r, input)
                }
                i => self.execute_arithmetic_instruction(i)?,
            }
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        self.write(&Register::W, 0);
        self.write(&Register::X, 0);
        self.write(&Register::Y, 0);
        self.write(&Register::Z, 0);
    }

    fn execute_arithmetic_instruction(&mut self, instr: &Instruction) -> Result<()> {
        match instr {
            Instruction::Inp(_) => bail!("Cannot handle inp"),
            Instruction::Add(r, o) => self.write(r, self.read(r) + self.get_operand_value(o)),
            Instruction::Mul(r, o) => self.write(r, self.read(r) * self.get_operand_value(o)),
            Instruction::Div(r, o) => self.write(r, self.read(r) / self.get_operand_value(o)),
            Instruction::Mod(r, o) => self.write(r, self.read(r) % self.get_operand_value(o)),
            Instruction::Eql(r, o) => self.write(
                r,
                if self.read(&r) == self.get_operand_value(o) {
                    1
                } else {
                    0
                },
            ),
        }
        Ok(())
    }

    pub fn read(&self, r: &Register) -> Value {
        *self.registers.get(r).expect("missing register")
    }

    pub fn save(&self) -> HashMap<Register, Value> {
        self.registers.clone()
    }

    pub fn restore(&mut self, state: HashMap<Register, Value>) {
        self.registers = state;
    }

    fn get_operand_value(&self, operand: &Operand) -> Value {
        match operand {
            Operand::Immediate(v) => *v,
            Operand::Var(r) => self.read(r),
        }
    }

    fn write(&mut self, r: &Register, v: Value) {
        *self.registers.get_mut(r).expect("missing register") = v
    }

    fn count_inputs(prog: &[Instruction]) -> usize {
        prog.iter()
            .filter(|i| matches!(i, Instruction::Inp(_)))
            .count()
    }

    pub fn has_state(&self, state: &HashMap<Register, Value>) -> bool {
        self.registers.len() == state.len() && state.iter().all(|(k, v)| self.registers.get(k).map(|sv| v == sv).unwrap_or(false))
    }
}

fn generate_model_numbers() -> Box<dyn Iterator<Item=Vec<Value>>> {
    Box::new((11111111111111..99999999999999_i128)
        .map(|mut i| {
            let mut digits = Vec::new();
            while i > 9 {
                digits.push(i % 10);
                i /= 10;
            }
            digits.push(i);
            digits.reverse();
            digits
        })
        .rev())
}

// fn test_model_numbers(monad: &[Instruction], alu: &mut ALU, seen_states: &mut Vec<HashMap<Register, Value>>) -> Option<Vec<u8>> {
//     let first_inp = monad.iter().position(|i| matches!(i, Instruction::Inp(_)));
//     if let Some(i) = first_inp {
//         alu.execute(&monad[..i], &[]).expect("execution failed");
//         let state = alu.save();
//         let seen_states = Vec::new();
//         for inp in (1..=9).rev() {
//             alu.restore(state.clone());
//             alu.execute(&monad[i..i+1], &[inp]).expect("execution failed");
//             seen_states.push(alu.save)
//             if let Some(mut model_number) = test_model_numbers(&monad[i+1..], alu) {
//                 model_number.push(inp as u8);
//                 return Some(model_number)
//             }
//         }
//         None
//     } else {
//         alu.execute(monad, &[]).expect("execution failed");
//         if alu.read(&Register::Z) == 0 {
//             Some(vec![])
//         } else {
//             None
//         }
//     }
// }

fn test_model_numbers2(monad: &[Instruction], alu: &mut ALU, p: u32) -> Option<Vec<u8>> {
    if !monad.is_empty() {
        // First instruction should be inp
        let mut seen_states = Vec::new();
        let state = alu.save();
        for inp in (1..=9).rev() {
            alu.restore(state.clone());
            alu.execute(&monad[0..1], &[inp]).expect("first instruction was not an input");
            // Add 1 since we already executed the first instruction
            let first_inp = monad[1..].iter().position(|i| matches!(i, Instruction::Inp(_))).unwrap_or(monad[1..].len()) + 1;
            
            alu.execute(&monad[1..first_inp], &[]).expect("execution failed");

            if seen_states.iter().any(|s| alu.has_state(s)) {
                println!("Skipping {} at level {}", inp, p);
                continue
            } else {
                seen_states.push(alu.save());
            }

            if let Some(mut model_number) = test_model_numbers2(&monad[first_inp..], alu, p - 1) {
                model_number.push(inp as u8);
                return Some(model_number)
            }
        }
        None
    } else {
        if alu.read(&Register::Z) == 0 {
            Some(vec![])
        } else {
            None
        }
    }
}

fn main() -> Result<()> {
    let inputs = include_str!("../input").trim();
    let monad = program_parser::program(inputs)?;
    let mut alu = ALU::new();
    let mut model_number = test_model_numbers2(monad.as_slice(), &mut alu, 14).expect("no valid model number found");
    model_number.reverse();
    println!("{:?}", model_number);
    Ok(())
}
