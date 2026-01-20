mod instruction;
mod stack;

use std::{
    env,
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use anyhow::anyhow;

use crate::instruction::Instruction;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path: PathBuf = args.get(1).ok_or(anyhow!("No input provided"))?.into();

    let instructions = instruction::compile(&input_path)?;

    let mut output_path = input_path.clone();
    output_path.set_extension("hasm");

    save(&instructions, &File::create(output_path)?)?;

    Ok(())
}

fn save(instructions: &[Instruction], file: &File) -> io::Result<()> {
    let mut writer = BufWriter::new(file);

    for instruction in instructions {
        writeln!(&mut writer, "{}", instruction.0)?;
    }

    Ok(())
}
