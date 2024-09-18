mod distance;

pub use distance::*;

#[derive(Debug)]
pub enum Command {
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
    pub lld: Difference,
}

impl SMove {
    pub fn read(current: u8, buffer: &[u8; 1]) -> Self {
        let a = current >> 4 & 0b11;
        let i = read_bits(buffer[0], 5);

        Self {
            lld: Lld::read(a, i),
        }
    }
}

#[derive(Debug)]
pub struct LMove {
    pub sld1: Difference,
    pub sld2: Difference,
}

impl LMove {
    pub fn read(current: u8, value: &[u8; 1]) -> Self {
        let a1 = current >> 4 & 0b11;
        let a2 = current >> 6 & 0b11;
        let i1 = value[0] & 0b1111;
        let i2 = value[0] >> 4 & 0b1111;

        Self {
            sld1: Sld::read(a1, i1),
            sld2: Sld::read(a2, i2),
        }
    }
}

#[derive(Debug)]
pub struct FusionP {
    pub nd: Difference,
}

impl FusionP {
    pub fn read(current: u8) -> Self {
        let nd = read_nd(current);
        Self { nd }
    }
}

#[derive(Debug)]
pub struct FusionS {
    pub nd: Difference,
}

impl FusionS {
    pub fn read(current: u8) -> Self {
        let nd = read_nd(current);
        Self { nd }
    }
}
#[derive(Debug)]
pub struct Fission {
    pub nd: Difference,
    pub m: u8,
}

impl Fission {
    pub fn read(current: u8, buffer: &[u8; 1]) -> Self {
        let nd = read_nd(current);
        let m = buffer[0];
        Self { nd, m }
    }
}

#[derive(Debug)]
pub struct Fill {
    pub nd: Difference,
}

impl Fill {
    pub fn read(current: u8) -> Self {
        Self {
            nd: read_nd(current),
        }
    }
}

#[derive(Debug)]
pub struct Void {
    pub nd: Difference,
}

impl Void {
    pub fn read(current: u8) -> Self {
        Self {
            nd: read_nd(current),
        }
    }
}

#[derive(Debug)]
pub struct GFill {
    pub nd: Difference,
    pub fd: Difference,
}

impl GFill {
    pub fn read(current: u8, buffer: &[u8; 3]) -> Self {
        let nd = read_nd(current);
        let fd = read_fd(buffer);
        Self { nd, fd }
    }
}

#[derive(Debug)]
pub struct GVoid {
    pub nd: Difference,
    pub fd: Difference,
}

impl GVoid {
    pub fn read(current: u8, buffer: &[u8; 3]) -> Self {
        let nd = read_nd(current);
        let fd = read_fd(buffer);
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
