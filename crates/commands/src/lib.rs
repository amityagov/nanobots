#[derive(Debug)]
pub struct Sld {
    pub a: u8,
    pub i: u8,
}

#[derive(Debug)]
pub struct Lld {
    pub a: u8,
    pub i: u8,
}

#[derive(Debug)]
pub struct Nd {
    pub dx: i8,
    pub dy: i8,
    pub dz: i8,
}

impl Nd {
    pub fn read(current: u8) -> Self {
        let nd = current >> 3;

        let dz = (nd % 3) as i8 - 1;
        let dy = ((nd / 3) % 3) as i8 - 1;
        let dx = (nd / 9) as i8 - 1;

        Self { dx, dy, dz }
    }
}

#[derive(Debug)]
pub struct Fd {
    pub dx: i8,
    pub dy: i8,
    pub dz: i8,
}

impl Fd {
    pub fn read(buffer: &[u8; 3]) -> Self {
        Self {
            dx: buffer[0] as i8 - 30,
            dy: buffer[1] as i8 - 30,
            dz: buffer[2] as i8 - 30,
        }
    }
}
#[derive(Debug)]
pub enum Command {
    None,
    NotParsed(u8),
    Halt,
    Wait,
    Flip,
    SMove(SMove),
    LMove(LMove),
    FusionP(FusionP),
    FusionS(FusionS),
    Fission(Fission),
    Fill(Fill),
    Void(Void),
    GFill(GFill),
    GVoid(GVoid),
}

pub fn read_bits(byte: u8, bit_count: u8) -> u8 {
    if bit_count > 8 {
        panic!("bit_count cannot be greater than 8");
    }

    let mask = (1 << bit_count) - 1; // Create a mask for the last `bit_count` bits
    byte & mask
}

#[derive(Debug)]
pub struct SMove {
    pub lld: Lld,
}

impl SMove {
    pub fn read(current: u8, buffer: &[u8; 1]) -> Self {
        let a = current >> 4 & 0b11;
        let i = read_bits(buffer[0], 5);

        Self { lld: Lld { a, i } }
    }
}

#[derive(Debug)]
pub struct LMove {
    pub sld1: Sld,
    pub sld2: Sld,
}

impl LMove {
    pub fn read(current: u8, value: &[u8; 1]) -> Self {
        let a1 = current >> 4 & 0b11;
        let a2 = current >> 6 & 0b11;
        let i1 = value[0] & 0b1111;
        let i2 = value[0] >> 4 & 0b1111;

        Self {
            sld1: Sld { a: a1, i: i1 },
            sld2: Sld { a: a2, i: i2 },
        }
    }
}

#[derive(Debug)]
pub struct FusionP {
    pub nd: Nd,
}

impl FusionP {
    pub fn read(current: u8) -> Self {
        let nd = Nd::read(current);
        Self { nd }
    }
}

#[derive(Debug)]
pub struct FusionS {
    pub nd: Nd,
}

impl FusionS {
    pub fn read(current: u8) -> Self {
        let nd = Nd::read(current);
        Self { nd }
    }
}
#[derive(Debug)]
pub struct Fission {
    pub nd: Nd,
    pub m: u8,
}

impl Fission {
    pub fn read(current: u8, buffer: &[u8; 1]) -> Self {
        let nd = Nd::read(current);
        let m = buffer[0];
        Self { nd, m }
    }
}

#[derive(Debug)]
pub struct Fill {
    pub nd: Nd,
}

impl Fill {
    pub fn read(current: u8) -> Self {
        Self {
            nd: Nd::read(current),
        }
    }
}

#[derive(Debug)]
pub struct Void {
    pub nd: Nd,
}

impl Void {
    pub fn read(current: u8) -> Self {
        Self {
            nd: Nd::read(current),
        }
    }
}

#[derive(Debug)]
pub struct GFill {
    pub nd: Nd,
    pub fd: Fd,
}

impl GFill {
    pub fn read(current: u8, buffer: &[u8; 3]) -> Self {
        let nd = Nd::read(current);
        let fd = Fd::read(buffer);
        Self { nd, fd }
    }
}

#[derive(Debug)]
pub struct GVoid {
    pub nd: Nd,
    pub fd: Fd,
}

impl GVoid {
    pub fn read(current: u8, buffer: &[u8; 3]) -> Self {
        let nd = Nd::read(current);
        let fd = Fd::read(buffer);
        Self { nd, fd }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gfill_read() {
        let current = 0b01010001;
        let buffer = [0b00101000, 0b00001111, 0b00110010];
        let g_fill = GFill::read(current, &buffer);
        println!("{:?}", g_fill);
    }

    #[test]
    fn gvoid_read() {
        let current = 0b10110000;
        let buffer = [0b00100011, 0b00100011, 0b00011001];
        let g_void = GVoid::read(current, &buffer);
        println!("{:?}", g_void);
    }
}
