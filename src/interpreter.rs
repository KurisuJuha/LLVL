use std::io::{self, Read, Write};

use anyhow::{Ok, Result};

use super::memory::Memory;

pub struct Interpreter {
    position: usize,
    memory: Memory,
}

impl Interpreter {
    pub fn new(memory: Memory) -> Interpreter {
        Interpreter {
            position: 0,
            memory,
        }
    }

    pub fn run(
        &mut self,
        program: &[u8],
        input: &mut impl Read,
        output: &mut impl Write,
    ) -> Result<()> {
        let mut program_counter = 0;
        while program_counter < program.len() {
            let instruction = program[program_counter];

            //            print!("{}", instruction as char);
            //            self.output.flush()?;

            match instruction {
                b'>' => {
                    self.position += 1;
                }
                b'<' => {
                    self.position -= 1;
                }
                b'+' => {
                    let value = self.memory.get(self.position);
                    self.memory.set(self.position, value.wrapping_add(1));
                }
                b'-' => {
                    let value = self.memory.get(self.position);
                    self.memory.set(self.position, value.wrapping_sub(1));
                }
                b'[' => {
                    let current_memory_value = self.memory.get(self.position);
                    if current_memory_value == 0 {
                        let loop_end = Self::get_loop_end(program, program_counter)?;
                        program_counter = loop_end;
                    }
                }
                b']' => {
                    let current_memory_value = self.memory.get(self.position);
                    if current_memory_value != 0 {
                        let loop_start = Self::get_loop_start(program, program_counter)?;
                        program_counter = loop_start;
                    }
                }
                b'.' => {
                    let value = self.memory.get(self.position);
                    output.write_all(&[value])?;
                }
                b',' => {
                    let mut buffer = [0];
                    input.read_exact(&mut buffer)?;
                    self.memory.set(self.position, buffer[0]);
                }
                _ => {}
            }

            program_counter += 1;
        }

        Ok(())
    }

    fn get_loop_start(program: &[u8], program_counter: usize) -> Result<usize> {
        let mut loop_end_count = 1;
        let mut program_counter = program_counter;

        while loop_end_count > 0 {
            program_counter -= 1;
            if program[program_counter] == b'[' {
                loop_end_count -= 1;
            } else if program[program_counter] == b']' {
                loop_end_count += 1;
            }
        }

        Ok(program_counter)
    }

    fn get_loop_end(program: &[u8], program_counter: usize) -> Result<usize> {
        let mut loop_start_count = 1;
        let mut program_counter = program_counter;

        while loop_start_count > 0 {
            program_counter += 1;
            if program[program_counter] == b']' {
                loop_start_count -= 1;
            } else if program[program_counter] == b'[' {
                loop_start_count += 1;
            }
        }

        Ok(program_counter)
    }
}

pub fn run(program: &[u8]) -> Result<Memory> {
    let memory = Memory::new();

    let mut input = io::stdin();
    let mut output = io::stdout();

    let mut interpreter = Interpreter::new(memory);

    let _ = interpreter.run(program, &mut input, &mut output);

    Ok(interpreter.memory)
}
