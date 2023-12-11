const KX: u64 = 123456789;
const KY: u64 = 362436069;
const KZ: u64 = 521288629;
const KW: u64 = 88675123;

pub struct Rand {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}


impl Rand {
    pub fn new(seed: u64) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }

    // Xorshift 128, taken from German Wikipedia
    pub fn rand(&mut self) -> u64 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }
}