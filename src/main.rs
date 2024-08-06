pub mod bus;
pub mod clint;
pub mod cpu;
pub mod csr;
pub mod dram;
pub mod exception;
pub mod param;
pub mod plic;
pub mod uart;

use cpu::Cpu;
use std::io::Read;
use std::{env, fs::File, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: rvemu <filename>");
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut cpu = Cpu::new(code);
    loop {
        // fetch
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(e) => {
                cpu.handle_exception(e);
                if e.is_fatal() {
                    println!("error in fetch: {}", e);
                    break;
                }
                continue;
            }
        };

        // decode
        // execute
        match cpu.execute(inst) {
            Ok(n_pc) => cpu.pc = n_pc,
            Err(e) => {
                cpu.handle_exception(e);
                if e.is_fatal() {
                    println!("error in execute: {}", e);
                    break;
                }
            }
        }
    }
    cpu.dump_registers();

    Ok(())
}
