use std::fs::File;
use std::io::{Read, Write};
use std::error::Error;
use libc;

pub struct Interpreter<R: Read, W: Write> {
    cells: Vec<u8>,
    instructions: Vec<char>,
    instr_ptr: usize,
    cell_ptr: usize,
    loop_stack: Vec<usize>,
    input: R,
    output: W,
}

macro_rules! read_little_endian {
    ($cells:expr, $idx:expr, $t:ty) => {{
        let mut arr = [0u8; std::mem::size_of::<$t>()];
        arr.copy_from_slice(
            &$cells[$idx..$idx + std::mem::size_of::<$t>()]
        );
        <$t>::from_le_bytes(arr)
    }};
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn new(file_name: &str, input: R, output: W) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(file_name)?;
        let mut instructions = String::new();
        file.read_to_string(&mut instructions)?;

        Ok(Interpreter{
            cells: vec![0],
            instructions: instructions.chars().collect(),
            instr_ptr: 0,
            cell_ptr: 0,
            loop_stack: Vec::new(),
            input,
            output,
        })
    }

    pub fn step(&mut self) -> Result<(), Box<dyn Error>> {
        match self.instructions.get(self.instr_ptr) {
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
                self.output.flush()?;
            },
            Some(',') => {
                let mut buf = [0];
                self.input.read_exact(&mut buf)?;
                self.cells[self.cell_ptr] = buf[0];
            },
            Some('[') => {
                if self.cells[self.cell_ptr] != 0 {
                    self.loop_stack.push(self.instr_ptr);
                } else {
                    let mut nest_level = 1;
                    while nest_level > 0 {
                        self.instr_ptr += 1;
                        match self.instructions.get(self.instr_ptr) {
                            Some('[') => nest_level += 1,
                            Some(']') => nest_level -= 1,
                            Some(_) => {},
                            None => return Err("Unmatched '['".into())
                        };
                    }
                }
            },
            Some(']') => {
                if self.cells[self.cell_ptr] != 0 {
                    self.instr_ptr = *self.loop_stack.last().ok_or("Unmatched ']'")? + 1;
                    return Ok(());
                } else {
                    self.loop_stack.pop();
                }
            },
            Some('!') => unsafe {
                self.syscall()?;
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

    unsafe fn syscall(&self) -> Result<u64, Box<dyn Error>> {
        let args = self.collect_args(self.cell_ptr + 4)?;
        let num: u32 = read_little_endian!(self.cells, self.cell_ptr, u32);
        Ok(unsafe {
            match args.len() {
                0 => libc::syscall(num as i32),
                1 => libc::syscall(num as i32, args[0]),
                2 => libc::syscall(num as i32, args[0], args[1]),
                3 => libc::syscall(num as i32, args[0], args[1], args[2]),
                4 => libc::syscall(num as i32, args[0], args[1], args[2], args[3]),
                5 => libc::syscall(num as i32, args[0], args[1], args[2], args[3], args[4]),
                6 => libc::syscall(num as i32, args[0], args[1], args[2], args[3], args[4], args[5]),
                _ => { return Err("too many syscall arguments".into()) }
            }
        } as u64)
    }

    fn collect_args(&self, argc_idx: usize) -> Result<Vec<u64>, Box<dyn Error>> {
        let mut args = Vec::new();
        let argc = self.cells[argc_idx] as usize;
        let args_begin = argc_idx + 1;
        for i in 0..argc {
            let arg_begin = args_begin + 9*i;
            let arg_type = self.cells[arg_begin];
            let arg_raw: u64 = read_little_endian!(self.cells, arg_begin + 1, u64);
            let arg: u64 = match arg_type {
                0 => arg_raw,
                1 => {
                    let offset = arg_raw as usize;
                    if offset >= self.cells.len() {
                        return Err(format!("pointer out of bounds ({}) at cell {}", offset, arg_begin).into());
                    }
                    unsafe {
                        self.cells.as_ptr().add(offset) as u64
                    }
                },
                _ => return Err(format!("invalid arg type at cell {}", arg_begin).into())
            };
            args.push(arg);
        }

        Ok(args)
    }
}