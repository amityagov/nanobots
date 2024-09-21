pub use commands::Command;
use commands::{read_bits, Fill, Fission, FusionP, FusionS, GFill, GVoid, LMove, SMove, Void};
use std::io::BufRead;

pub fn read_commands(reader: &mut impl BufRead) -> anyhow::Result<Vec<Command>> {
    let mut buffer = [0u8; 1];
    let mut commands = vec![];

    while let Ok(count) = reader.read(&mut buffer) {
        if count == 0 {
            break;
        }

        let command = buffer[0];
        match command {
            0b11111111 => commands.push(Command::Halt),
            0b11111110 => commands.push(Command::Wait),
            0b11111101 => commands.push(Command::Flip),
            _ => commands.push(parse_command(command, reader)?),
        }
    }

    Ok(commands)
}

fn parse_command(current: u8, reader: &mut impl BufRead) -> anyhow::Result<Command> {
    let simple_command = read_bits(current, 3);
    match simple_command {
        0b00000100 => {
            let exact_command = read_bits(current, 4);

            match exact_command {
                0b00000100 => {
                    let mut buffer = [0u8; 1];
                    reader.read(&mut buffer)?;
                    Ok(Command::SMove(SMove::read(current, &buffer)))
                }
                0b00001100 => {
                    let mut buffer = [0u8; 1];
                    reader.read(&mut buffer)?;
                    Ok(Command::LMove(LMove::read(current, &buffer)))
                }
                _ => unreachable!(),
            }
        }
        0b00000111 => Ok(Command::FusionP(FusionP::read(current))),
        0b00000110 => Ok(Command::FusionS(FusionS::read(current))),
        0b00000101 => {
            let mut buffer = [0u8; 1];
            reader.read(&mut buffer)?;
            Ok(Command::Fission(Fission::read(current, &buffer)))
        }
        0b00000011 => Ok(Command::Fill(Fill::read(current))),
        0b00000010 => Ok(Command::Void(Void::read(current))),
        0b00000001 => {
            let mut buffer = [0u8; 3];
            reader.read(&mut buffer)?;
            Ok(Command::GFill(GFill::read(current, &buffer)))
        }
        0b00000000 => {
            let mut buffer = [0u8; 3];
            reader.read(&mut buffer)?;
            Ok(Command::GVoid(GVoid::read(current, &buffer)))
        }
        _ => Err(anyhow::anyhow!("Command not recognized {current:b}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::BufReader;

    #[test]
    fn test() -> anyhow::Result<()> {
        let data = include_bytes!("../../../data/FA001.nbt");
        let mut reader = BufReader::new(data.as_slice());
        let commands = read_commands(&mut reader)?;

        // println!("{:?}", commands);
        let mut counts = HashMap::new();

        fn update(name: &'static str, counts: &mut HashMap<&'static str, usize>) -> () {
            counts.entry(name).and_modify(|e| *e += 1).or_insert(1);
        }

        for x in commands {
            match &x {
                Command::Halt => update("Halt", &mut counts),
                Command::Wait => update("Wait", &mut counts),
                Command::Flip => update("Flip", &mut counts),
                Command::SMove(_) => update("SMove", &mut counts),
                Command::LMove(_) => update("LMove", &mut counts),
                Command::FusionP(_) => update("FusionP", &mut counts),
                Command::FusionS(_) => update("FusionS", &mut counts),
                Command::Fission(_) => update("Fission", &mut counts),
                Command::Fill(_) => update("Fill", &mut counts),
                Command::Void(_) => update("Void", &mut counts),
                Command::GFill(_) => update("GFill", &mut counts),
                Command::GVoid(_) => update("GVoid", &mut counts),
            }
        }

        println!("counts: {:#?}", counts);
        Ok(())
    }
}
