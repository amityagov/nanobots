use std::io::BufRead;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum CellState {
    Fill,
    Void,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub index: usize,
    pub state: CellState,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Model {
    cells: Vec<Cell>,
    r: usize,
}

fn get_state(byte: u8, position: usize) -> CellState {
    match (byte >> position) & 1 {
        1 => CellState::Fill,
        0 => CellState::Void,
        _ => unreachable!(),
    }
}

fn read_model_cells(reader: impl BufRead) -> impl Iterator<Item = CellState> {
    reader
        .bytes()
        .filter_map(Result::ok)
        .flat_map(|byte| (0..8).map(move |position| get_state(byte, position)))
}

pub fn read_model(reader: &mut impl BufRead) -> anyhow::Result<Model> {
    let mut r_bytes = [0u8; 1];
    let r_bytes_count = reader.read(&mut r_bytes)?;
    if r_bytes_count < 1 {
        return Err(anyhow::anyhow!("Not enough bytes for resolution"));
    }

    let mut cells_iterator = read_model_cells(reader);

    let r = r_bytes[0] as usize;
    let expected_cell_count = r * r * r;
    let mut cell_processed = 0;

    let mut cells = vec![];

    while let Some(state) = cells_iterator.next() {
        if cell_processed == expected_cell_count {
            return Err(anyhow::anyhow!(
                "Too many cells in model, expected {expected_cell_count}"
            ));
        }

        let x = cell_processed / (r * r); // Near to far
        let y = (cell_processed / r) % r; // Bottom to top
        let z = cell_processed % r; // Left to right

        cells.push(Cell {
            index: cell_processed,
            state,
            x,
            y,
            z,
        });

        cell_processed += 1;
    }

    if cell_processed < expected_cell_count {
        return Err(anyhow::anyhow!(
            "Too few cells in model, expected {expected_cell_count}, got {cell_processed}"
        ));
    }

    Ok(Model { cells, r })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::io::Cursor;

    #[test]
    fn test_model_loaded() -> anyhow::Result<()> {
        let data = include_bytes!("../../../data/FA004_tgt.mdl");

        let mut cursor = Cursor::new(data);
        let model = read_model(&mut cursor)?;

        let mut rng = thread_rng();
        let x = rng.gen_range(0..model.r);
        let y = rng.gen_range(0..model.r);
        let z = rng.gen_range(0..model.r);

        let index = x * model.r * model.r + y * model.r + z;
        let cell = &model.cells[index];

        assert_eq!(x, cell.x);
        assert_eq!(y, cell.y);
        assert_eq!(z, cell.z);

        Ok(())
    }
}
