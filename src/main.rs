pub mod bus;
pub mod cpu;
pub mod dram;
pub mod exception;
pub mod param;
pub mod csr;

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
                println!("{}", e);
                break;
            }
        };

        // decode
        // execute
        match cpu.execute(inst) {
            Ok(n_pc) => cpu.set_pc(n_pc),
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
    cpu.dump_registers();

    Ok(())
}
