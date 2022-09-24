use crate::consts::{QOI_OP_LUMA, QOI_OP_RGB, QOI_OP_RGBA};
use crate::error::Result;
use core::panic;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Pixel<const N: usize>([u8; N]);

impl<const N: usize> Pixel<N> {
    pub const fn new() -> Self {
        Self([0; N])
    }

    pub fn with_a(mut self, a: u8) -> Self {
        if N == 4 {
            self.0[3] = a;
        }

        self
    }

    pub fn update_rgb(&mut self, r: u8, g: u8, b: u8) {
        self.0[0] = r;
        self.0[1] = g;
        self.0[2] = b;
    }

    pub fn update_rgba(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.update_rgb(r, g, b);
        if N == 4 {
            self.0[3] = a;
        }
    }

    pub fn from(self, a: u8) -> Pixel<4> {
        let mut out = Pixel::new();

        if N == 4 {
            out.update_rgba(self.0[0], self.0[1], self.0[2], self.0[3]);
        } else {
            out.update_rgba(self.0[0], self.0[1], self.0[2], a);
        }

        out
    }

    pub fn read(&mut self, chunk: &[u8]) {
        match N {
            3 => self.update_rgb(chunk[0], chunk[1], chunk[2]),
            4 => self.update_rgba(chunk[0], chunk[1], chunk[2], chunk[3]),
            _ => unreachable!(),
        }
    }

    pub fn hash_index(self) -> u8 {
        match N {
            3 => (self.0[0] * 3 + self.0[1] * 5 + self.0[2] * 7 + 0xff * 11) % 64,
            4 => (self.0[0] * 3 + self.0[1] * 5 + self.0[2] * 7 + self.0[3] * 11) % 64,
            _ => unreachable!(),
        }
    }

    pub fn encode(&self, px_prev: Self) -> Result<Vec<u8>> {
        let mut out = Vec::new();
        if N == 3 || self.a_or(0) == px_prev.a_or(0) {
            let vr = self.r().wrapping_sub(px_prev.r());
            let vg = self.g().wrapping_sub(px_prev.g());
            let vb = self.b().wrapping_sub(px_prev.b());
            let vg_r = vr.wrapping_sub(vg);
            let vg_b = vb.wrapping_sub(vg);
            let (vr_2, vg_2, vb_2) = (vr.wrapping_add(2), vg.wrapping_add(2), vb.wrapping_add(2));

            // X | 0bYY == 0bYY
            // Return all number under 0bYY
            // YY is bitset
            if (vr_2 | vg_2 | vb_2 | 3) == 3 {
                out.push(0b01 | vr_2 << 4 | vg_2 << 2 | vb_2);
            } else {
                let (vg_r_8, vg_b_8) = (vg_r.wrapping_add(8), vg_b.wrapping_add(8));
                if (vg_r_8 | vg_b_8 | 15) == 15 {
                    out.push(QOI_OP_LUMA | vg << 8);
                    out.push(vg_r_8 << 4 | vg_r_8);
                } else {
                    let mut vec = vec![QOI_OP_RGB, self.r(), self.g(), self.b()];
                    out.append(&mut vec);
                }
            }
        } else {
            let mut vec = vec![QOI_OP_RGBA, self.r(), self.g(), self.b(), self.a()];
            out.append(&mut vec);
        }

        Ok(out)
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

    pub const fn a(self) -> u8 {
        match N {
            4 => self.0[3],
            _ => panic!(),
        }
    }

    pub const fn a_or(self, value: u8) -> u8 {
        match N {
            4 => self.0[3],
            _ => value,
        }
    }
}

impl<const N: usize> From<Pixel<N>> for [u8; N] {
    fn from(pixel: Pixel<N>) -> Self {
        pixel.0
    }
}
