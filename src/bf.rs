use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;

pub struct Interpreter<R: Read, W: Write> {
    cells: Vec<u8>,
    instructions: String,
    instr_ptr: usize,
    cell_ptr: usize,
    loop_stack: Vec<usize>,
    input: R,
    output: W,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(file_name: &str, input: R, output: W) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(file_name)?;
        let mut instructions = String::new();
        file.read_to_string(&mut instructions)?;

        // always use stdin/stdout for now
        Ok(Interpreter{
            cells: vec![0],
            instructions,
            instr_ptr: 0,
            cell_ptr: 0,
            loop_stack: Vec::new(),
            input,
            output,
        })
    }

    pub fn step(&mut self) -> Result<(), Box<dyn Error>> {
        match self.instructions.chars().nth(self.instr_ptr) {
            Some('+') => {
                self.cells[self.cell_ptr] = self.cells[self.cell_ptr].wrapping_add(1);
            },
            Some('-') => {
                self.cells[self.cell_ptr] = self.cells[self.cell_ptr].wrapping_sub(1);
            },
            Some('>') => {
                self.cell_ptr += 1;
                if self.cell_ptr >= self.cells.len() {
                    self.cells.push(0);
                }
            },
            Some('<') => {
                if self.cell_ptr > 0 {
                    self.cell_ptr -= 1;
                }
            },
            Some('.') => {
                self.output.write_all(&[self.cells[self.cell_ptr]])?;
            },
            Some(',') => {
                let mut buf = [0];
                self.input.read_exact(&mut buf)?;
                self.cells[self.cell_ptr] = buf[0];
            },
            Some('[') => {
                self.loop_stack.push(self.instr_ptr);
            },
            Some(']') => {
                if self.cells[self.cell_ptr] != 0 {
                    self.instr_ptr = self.loop_stack.last().copied().unwrap(); // TODO: handle unmatched loop error
                    return Ok(());
                } else {
                    self.loop_stack.pop();
                }
            },
            Some(_) => {},
            None => {}
        };
        self.instr_ptr += 1;
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        while self.instr_ptr < self.instructions.len() {
            self.step()?;
        }
        Ok(())
    }
}