use commands::Difference;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Position {
    pub fn zero() -> Position {
        Self { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug)]
pub struct Bot {
    pub index: usize,
    pub seeds: Vec<u8>,
    position: Position,
}

impl Bot {
    pub fn initial(bot_count: u8) -> Self {
        let seeds = (0..bot_count - 1)
            .enumerate()
            .map(|(index, _)| index as u8 + 2)
            .collect();
        Self::new(1, seeds, Position::zero())
    }

    pub fn new(index: usize, seeds: Vec<u8>, position: Position) -> Self {
        Self {
            index,
            seeds,
            position,
        }
    }

    pub fn apply_position_diff(&mut self, diff: &Difference) {
        self.position = get_position_by_diff(&self.position, diff);
    }

    pub fn get_position_by_diff(&self, diff: &Difference) -> Position {
        get_position_by_diff(&self.position, diff)
    }
}

pub fn get_position_by_diff(position: &Position, diff: &Difference) -> Position {
    let mut position = position.clone();
    if diff.dx < 0 {
        position.x -= diff.dx.abs() as u8;
    } else {
        position.x += diff.dx as u8;
    }

    if diff.dy < 0 {
        position.y -= diff.dy.abs() as u8;
    } else {
        position.y += diff.dy as u8;
    }
    if diff.dz < 0 {
        position.z -= diff.dz.abs() as u8;
    } else {
        position.z += diff.dz as u8;
    }

    position
}
