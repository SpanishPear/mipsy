pub mod error;
pub mod inst;
pub mod yaml;
pub mod util;
pub mod compile;
pub mod decompile;
pub mod runtime;

use error::RSpimResult;
use std::io::{stdin, Read};

#[allow(dead_code)]
fn pause() {
    stdin().read(&mut [0]).unwrap();
}

fn print_info(s: String) {
    if false {
        println!("{}", s);
    }
}

fn main() -> RSpimResult<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <file>", &args[0]);
        return Ok(());
    }

    let file_name = &args[1];
    let file_contents = std::fs::read_to_string(file_name).expect("Could not read file {}");


    let yaml = yaml::parse::get_instructions();
    // println!("Parsed mips.yaml: \n\n{:#x?}\n\n", yaml);

    let iset = inst::instruction::InstSet::new(&yaml)?;
    // println!("Loaded instruction set: \n\n{:#x?}\n\n", iset);

    let tokens = compile::lexer::tokenise(&file_contents)?;
    print_info(format!("Lexed {} into tokens: \n\n{:x?}\n\n", file_name, tokens));
    // pause();

    let program = compile::compiler::generate(tokens, &iset)?;
    // println!("Successfully generated program: \n\n{:#010x?}\n\n", program);

    let decompiled = decompile::decompile(&program, &iset);
    print_info(format!("Successfully compiled program: \n\n{}\n\n", decompiled));
    // pause();

    print_info(format!("Labels: "));
    for (label, addr) in &program.labels {
        print_info(format!("    {:9} => 0x{:08x}", label, addr));
    }
    print_info(format!("\n"));
    // pause();

    let mut runtime = runtime::Runtime::new(&program);
    print_info(format!("Loaded runtime: {:}", runtime.state()));
    // pause();

    // LOL
    println!("Loaded: /home/zac/uni/teach/comp1521/20T2/work/spim-simulator/CPU/exceptions.s");

    loop {
        match runtime.step() {
            Ok(_) => {},
            Err(error::RSpimError::Runtime(error::runtime_error::RuntimeError::UninitializedRegister(31))) => { break; }
            Err(e) => {
                println!("Error: {:x?}", e);
                let timeline_len = runtime.timeline_len();
                println!("Timeline length: {}", timeline_len);

                for i in (1..=5).rev() {
                    if (timeline_len as isize - i) < 0 {
                        continue;
                    }

                    println!("{}", runtime.nth_state(timeline_len - i as usize).unwrap());
                }

                break;
            }
        }
        // println!("Current state: {:}", runtime.state());
        // pause();
    }

    Ok(())
}

