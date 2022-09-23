// use crate::error::Result;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Pixel<const N: usize>([u8; N]);

impl<const N: usize> Pixel<N> {

    pub const fn new() -> Self {
        Self([0; N])
    }

    pub const fn with_a(mut self, a: u8) -> Self {
        if N == 4 {
            self.0[3] = a;
        }
        self
    }
    
    pub const fn r(self) -> u8 {
        self.0[0]
    }

    pub const fn g(self) -> u8 {
        self.0[1]
    }

    pub const fn b(self) -> u8 {
        self.0[2]
    }

    pub fn update_rgb(&mut self, r: u8, g: u8, b: u8){
        self.0[0] = r;
        self.0[1] = g;
        self.0[2] = b;
    }

    pub fn update_rgba(&mut self, r: u8, g: u8, b: u8, a: u8){
        self.update_rgb(r,g,b);
        if N == 4 {
            self.0[3] = a;
        }
    }

    pub fn hash_index(self) -> u8 {
        match N {
            3 => (self.0[0] * 3 + self.0[1] * 5 + self.0[2] * 7 + 0xff * 11) % 64,
            4 => (self.0[0] * 3 + self.0[1] * 5 + self.0[2] * 7 + self.0[3] * 11) % 64,
            _ => unreachable!(),
        }
    }
}