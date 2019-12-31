use std::{error::Error, fs::File, io::Read};

use advent_utils::{get_config, Part};

use intcode::IntcodeInterpreter;

fn main() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;

    let mut code_str = String::new();
    File::open(config.input_file)?.read_to_string(&mut code_str)?;

    let code: Vec<i32> = code_str
        .trim()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    match config.part {
        Part::One => {
            let mut interpreter: IntcodeInterpreter = code.into();
            interpreter.set_input([1].iter().copied());

            let halted = interpreter.run().unwrap();

            println!("diagnostics output is: {:?}", halted.get_output());
        }
        Part::Two => {
            let mut interpreter: IntcodeInterpreter = code.into();
            interpreter.set_input([5].iter().copied());

            let halted = interpreter.run().unwrap();

            println!("diagnostics output is: {:?}", halted.get_output());
        }
    }

    Ok(())
}
