pub const QOI_HEADER_SIZE: usize = 14;

pub const QOI_OP_INDEX: u8 = 0b00000000;
pub const QOI_OP_DIFF : u8 = 0b01000000;
pub const QOI_OP_LUMA : u8 = 0b10000000;
pub const QOI_OP_RUN  : u8 = 0b11000000;
pub const QOI_OP_RGB  : u8 = 0b11111110;
pub const QOI_OP_RGBA : u8 = 0b11111111;

pub const QOI_MASK: u8 = 0b11000000;

pub const QOI_PADDING: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0x01];
pub const QOI_PADDING_SIZE: usize = 8;

pub const QOI_MAGIC: u32 = u32::from_be_bytes(*b"qoif");

pub const QOI_PIXELS_MAX: usize = 400_000_000;