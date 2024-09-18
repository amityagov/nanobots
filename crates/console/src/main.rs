use state::StateCommand;
use std::io::Cursor;

fn main() -> anyhow::Result<()> {
    let model_data = include_bytes!("../../../data/FA104_tgt.mdl");
    let nbt_data = include_bytes!("../../../data/FA104.nbt");
    let mut matrix_reader = Cursor::new(model_data);
    let matrix = mdl::read_matrix(&mut matrix_reader)?;

    let mut nbt_reader = Cursor::new(nbt_data);
    let commands = nbt::read_commands(&mut nbt_reader)?;

    let empty_matrix = mdl::Matrix::new(matrix.r);
    let mut state = state::State::new(10, empty_matrix);

    let mut commands = &commands[..];

    while commands.len() > 0 {
        let command = &commands[0];
        commands = &commands[1..];
        command.apply(&mut state)?;
        let delta = 0;
        commands = &commands[delta..];
        state.end_step();
    }

    println!("{:?}", state.energy_spend_type);
    println!("energy {:?}", state.energy);
    println!("{:?}", state.bots[0]);
    assert_eq!(matrix, state.matrix);
    Ok(())
}

#[cfg(test)]
mod tests {
    use glob;
    use state::StateCommand;
    use std::fs::File;
    use std::io::{BufReader, Cursor};
    use std::path::{Path, PathBuf};
    use std::time::Instant;

    #[test]
    fn test() -> anyhow::Result<()> {
        let start = Instant::now();
        let pattern = "/Users/axel/Downloads/problemsF/FA*.nbt";
        let folder = Path::new("/Users/axel/Downloads/problemsF/");

        for entry in glob::glob(pattern)? {
            if let Ok(path) = entry {
                match run_one(&path, &folder) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
        }

        println!("done in {:?}", start.elapsed());
        Ok(())
    }

    fn run_one(path: &PathBuf, folder: &Path) -> anyhow::Result<()> {
        println!("Running {:?}", path);

        if let Some(stem) = path.file_stem() {
            let model_file = folder.join(format!("{}_tgt.mdl", stem.to_string_lossy()).to_string());
            println!("With model file {:?}", model_file);
            if model_file.exists() {
                let model_data = File::open(model_file)?;
                let mut matrix_reader = BufReader::new(model_data);
                let matrix = mdl::read_matrix(&mut matrix_reader)?;

                let mut nbt_reader = BufReader::new(File::open(path)?);
                let commands = nbt::read_commands(&mut nbt_reader)?;
                println!("Got commands {:?}", commands.len());
                let empty_matrix = mdl::Matrix::new(matrix.r);
                let mut state = state::State::new(10, empty_matrix);

                let mut commands = &commands[..];

                while commands.len() > 0 {
                    let command = &commands[0];
                    commands = &commands[1..];
                    command.apply(&mut state)?;
                    let delta = 0;
                    commands = &commands[delta..];
                    state.end_step();
                }
                assert_eq!(matrix, state.matrix);
            }
        }
        Ok(())
    }
}
