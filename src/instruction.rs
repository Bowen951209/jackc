use std::{
    borrow::Cow,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

use crate::stack::{self, U15};

#[macro_export]
macro_rules! instr {
    [ $( $x:expr ),* $(,)? ] => {{
        Box::new([
            $(Instruction::from($x)),*
        ])
    }};
}

pub struct Instruction(pub Cow<'static, str>);

impl From<&'static str> for Instruction {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl From<String> for Instruction {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

pub struct VmLine(String);

impl From<VmLine> for Box<[Instruction]> {
    fn from(val: VmLine) -> Self {
        let mut cmds = val.0.split(' ');

        match cmds.next().unwrap() {
            "push" => match cmds.next().unwrap() {
                "constant" => stack::push_constant(U15::from_str(cmds.next().unwrap()).unwrap()),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

pub fn compile(filepath: impl AsRef<Path>) -> io::Result<Vec<Instruction>> {
    let vm_lines = read_vm_code(filepath)?;

    let mut result = Vec::new();
    for vm_line in vm_lines {
        let instructions: Box<[Instruction]> = vm_line.into();
        result.extend(instructions.into_iter());
    }

    Ok(result)
}

fn read_vm_code(filepath: impl AsRef<Path>) -> io::Result<Vec<VmLine>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.map(VmLine)).collect()
}
