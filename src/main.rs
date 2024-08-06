pub mod bus;
pub mod clint;
pub mod cpu;
pub mod csr;
pub mod dram;
pub mod exception;
pub mod interrupt;
pub mod param;
pub mod plic;
pub mod uart;
pub mod virtio;
pub mod virtqueue;

use cpu::Cpu;
use std::io::Read;
use std::{env, fs::File, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 && args.len() != 3 {
        panic!("Usage: rvemu <filename> [option]<file-image>");
    }

    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;

    let mut disk_image = Vec::new();
    if args.len() == 3 {
        let mut file = File::open(&args[2])?;
        file.read_to_end(&mut disk_image)?;
    }

    let mut cpu = Cpu::new(code, disk_image);
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

        match cpu.check_pending_interrupt() {
            Some(interrupt) => cpu.handle_interrupt(interrupt),
            None => (),
        }
    }
    cpu.dump_registers();

    Ok(())
}
