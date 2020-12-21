#[allow(dead_code)]

pub struct Flags {
    negative: bool,
    zero: bool,
    positive: bool,
}

impl Flags {
    pub fn update(&mut self, reg: u16) {
        match reg {
            0x0000 => {
                self.negative = false;
                self.zero = true;
                self.positive = false;
            }
            0x0001..=0x7fff => {
                self.negative = false;
                self.zero = false;
                self.positive = true;
            }
            0x8000..=0xffff => {
                self.negative = true;
                self.zero = false;
                self.positive = false;
            }
        }
    }
}
