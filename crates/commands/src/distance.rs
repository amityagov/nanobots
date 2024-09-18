pub trait Mlen {
    fn mlen(&self) -> u32;
}

pub trait Clen {
    fn clen(&self) -> u32;
}

fn mlen(values: &[i8]) -> u32 {
    values
        .iter()
        .map(|value| value.abs() as u32)
        .fold(0, |acc, value| acc + value as u32)
}

fn clen(values: &[i8]) -> u32 {
    values
        .iter()
        .map(|value| value.abs() as u32)
        .max()
        .unwrap_or_default()
}

#[derive(Debug)]
pub enum DifferenceKind {
    Near,
    Far,
    ShortLinear,
    LongLinear,
}

#[derive(Debug)]
pub struct Difference {
    pub dx: i8,
    pub dy: i8,
    pub dz: i8,
    pub kind: DifferenceKind,
}

impl Difference {
    pub fn new(dx: i8, dy: i8, dz: i8, kind: DifferenceKind) -> Self {
        Self { dx, dy, dz, kind }
    }
}

impl Mlen for Difference {
    fn mlen(&self) -> u32 {
        mlen(&[self.dx, self.dy, self.dz])
    }
}

impl Clen for Difference {
    fn clen(&self) -> u32 {
        clen(&[self.dx, self.dy, self.dz])
    }
}

pub fn read_fd(buffer: &[u8; 3]) -> Difference {
    Difference::new(
        buffer[0] as i8 - 30,
        buffer[1] as i8 - 30,
        buffer[2] as i8 - 30,
        DifferenceKind::Far,
    )
}

pub fn read_nd(current: u8) -> Difference {
    let nd = current >> 3;

    let dz = (nd % 3) as i8 - 1;
    let dy = ((nd / 3) % 3) as i8 - 1;
    let dx = (nd / 9) as i8 - 1;

    Difference::new(dx, dy, dz, DifferenceKind::Near)
}

fn read_ld(a: u8, i: u8, delta: u8) -> (i8, i8, i8) {
    let delta = i as i8 - delta as i8;
    match a {
        0b01 => (delta, 0, 0),
        0b10 => (0, delta, 0),
        0b11 => (0, 0, delta),
        _ => unreachable!(),
    }
}

pub struct Sld;

impl Sld {
    pub fn read(a: u8, i: u8) -> Difference {
        let (dx, dy, dz) = read_ld(a, i, 5);
        Difference::new(dx, dy, dz, DifferenceKind::ShortLinear)
    }
}

pub struct Lld;

impl Lld {
    pub fn read(a: u8, i: u8) -> Difference {
        let (dx, dy, dz) = read_ld(a, i, 15);
        Difference::new(dx, dy, dz, DifferenceKind::LongLinear)
    }
}
