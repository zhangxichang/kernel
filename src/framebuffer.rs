use limine::framebuffer::{FRAMEBUFFER_RGB, Framebuffer};

const GLYPH_WIDTH: usize = 16;
const GLYPH_HEIGHT: usize = 16;
const GLYPH_SPACING: usize = 2;
const MESSAGE: &str = "你好，世界";
const BACKGROUND: Color = Color::rgb(0x08, 0x0a, 0x0d);
const FOREGROUND: Color = Color::rgb(0xf4, 0xf7, 0xfb);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PixelFormat {
    bpp: u16,
    red_mask_size: u8,
    red_mask_shift: u8,
    green_mask_size: u8,
    green_mask_shift: u8,
    blue_mask_size: u8,
    blue_mask_shift: u8,
}

impl PixelFormat {
    pub const fn new(
        bpp: u16,
        red_mask_size: u8,
        red_mask_shift: u8,
        green_mask_size: u8,
        green_mask_shift: u8,
        blue_mask_size: u8,
        blue_mask_shift: u8,
    ) -> Option<Self> {
        if bpp % 8 != 0 || bpp == 0 || bpp > 32 {
            return None;
        }

        if !mask_fits(red_mask_size, red_mask_shift)
            || !mask_fits(green_mask_size, green_mask_shift)
            || !mask_fits(blue_mask_size, blue_mask_shift)
        {
            return None;
        }

        Some(Self {
            bpp,
            red_mask_size,
            red_mask_shift,
            green_mask_size,
            green_mask_shift,
            blue_mask_size,
            blue_mask_shift,
        })
    }

    pub fn from_limine(framebuffer: &Framebuffer) -> Option<Self> {
        Self::new(
            framebuffer.bpp,
            framebuffer.red_mask_size,
            framebuffer.red_mask_shift,
            framebuffer.green_mask_size,
            framebuffer.green_mask_shift,
            framebuffer.blue_mask_size,
            framebuffer.blue_mask_shift,
        )
    }

    pub fn bytes_per_pixel(self) -> usize {
        (self.bpp / 8) as usize
    }

    pub fn encode(self, color: Color) -> u32 {
        encode_channel(color.red, self.red_mask_size, self.red_mask_shift)
            | encode_channel(color.green, self.green_mask_size, self.green_mask_shift)
            | encode_channel(color.blue, self.blue_mask_size, self.blue_mask_shift)
    }
}

const fn mask_fits(size: u8, shift: u8) -> bool {
    size <= 8 && (size == 0 || (size as u16 + shift as u16) <= 32)
}

fn encode_channel(value: u8, mask_size: u8, mask_shift: u8) -> u32 {
    if mask_size == 0 {
        return 0;
    }

    let max = (1u32 << mask_size) - 1;
    let scaled = if mask_size == 8 {
        value as u32
    } else {
        ((value as u32 * max) + 127) / 255
    };

    (scaled & max) << mask_shift
}

pub struct FramebufferWriter<'a> {
    bytes: &'a mut [u8],
    width: usize,
    height: usize,
    pitch: usize,
    format: PixelFormat,
}

impl<'a> FramebufferWriter<'a> {
    pub fn new(
        bytes: &'a mut [u8],
        width: usize,
        height: usize,
        pitch: usize,
        format: PixelFormat,
    ) -> Option<Self> {
        let bytes_per_pixel = format.bytes_per_pixel();
        let visible_row_bytes = width.checked_mul(bytes_per_pixel)?;
        let framebuffer_bytes = pitch.checked_mul(height)?;

        if pitch < visible_row_bytes || bytes.len() < framebuffer_bytes {
            return None;
        }

        Some(Self {
            bytes,
            width,
            height,
            pitch,
            format,
        })
    }

    pub fn clear(&mut self, color: Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, color);
            }
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        let bytes_per_pixel = self.format.bytes_per_pixel();
        let Some(offset) = y
            .checked_mul(self.pitch)
            .and_then(|row| row.checked_add(x.checked_mul(bytes_per_pixel)?))
        else {
            return;
        };
        let end = offset + bytes_per_pixel;

        if end > self.bytes.len() {
            return;
        }

        let encoded = self.format.encode(color).to_le_bytes();
        self.bytes[offset..end].copy_from_slice(&encoded[..bytes_per_pixel]);
    }

    fn draw_glyph(&mut self, x: usize, y: usize, glyph: &Glyph, color: Color, scale: usize) {
        if scale == 0 {
            return;
        }

        for glyph_y in 0..GLYPH_HEIGHT {
            let row = glyph.rows[glyph_y];

            for glyph_x in 0..GLYPH_WIDTH {
                let mask = 1 << (GLYPH_WIDTH - 1 - glyph_x);
                if row & mask == 0 {
                    continue;
                }

                let pixel_x = x + glyph_x * scale;
                let pixel_y = y + glyph_y * scale;

                for dy in 0..scale {
                    for dx in 0..scale {
                        self.put_pixel(pixel_x + dx, pixel_y + dy, color);
                    }
                }
            }
        }
    }
}

pub fn draw_text(
    writer: &mut FramebufferWriter<'_>,
    x: usize,
    y: usize,
    text: &str,
    color: Color,
    scale: usize,
) {
    let mut cursor_x = x;
    let advance = (GLYPH_WIDTH + GLYPH_SPACING) * scale.max(1);

    for character in text.chars() {
        if let Some(glyph) = glyph_for(character) {
            writer.draw_glyph(cursor_x, y, glyph, color, scale);
        }

        cursor_x += advance;
    }
}

pub fn draw_hello_world(writer: &mut FramebufferWriter<'_>) {
    writer.clear(BACKGROUND);

    let scale = if writer.width >= 640 && writer.height >= 360 {
        4
    } else if writer.width >= 320 && writer.height >= 180 {
        2
    } else {
        1
    };
    let glyph_count = MESSAGE.chars().count();
    let text_width = glyph_count * GLYPH_WIDTH * scale + (glyph_count - 1) * GLYPH_SPACING * scale;
    let text_height = GLYPH_HEIGHT * scale;
    let x = writer.width.saturating_sub(text_width) / 2;
    let y = writer.height.saturating_sub(text_height) / 2;

    draw_text(writer, x, y, MESSAGE, FOREGROUND, scale);
}

pub fn draw_hello_world_to_framebuffer(framebuffer: &Framebuffer) {
    if framebuffer.memory_model != FRAMEBUFFER_RGB {
        return;
    }

    let Some(format) = PixelFormat::from_limine(framebuffer) else {
        return;
    };

    let width = framebuffer.width as usize;
    let height = framebuffer.height as usize;
    let pitch = framebuffer.pitch as usize;
    let bytes = unsafe { framebuffer.as_slice_mut() };

    if let Some(mut writer) = FramebufferWriter::new(bytes, width, height, pitch, format) {
        draw_hello_world(&mut writer);
    }
}

struct Glyph {
    rows: [u16; GLYPH_HEIGHT],
}

fn glyph_for(character: char) -> Option<&'static Glyph> {
    match character {
        '你' => Some(&NI),
        '好' => Some(&HAO),
        '，' => Some(&FULLWIDTH_COMMA),
        '世' => Some(&SHI),
        '界' => Some(&JIE),
        _ => None,
    }
}

static NI: Glyph = Glyph {
    rows: [
        0b0001000001000000,
        0b0001000001000000,
        0b0010000011111100,
        0b0010011001000100,
        0b0111100001000100,
        0b0010000001001000,
        0b0010000011111000,
        0b0010000001010000,
        0b0010000011011000,
        0b0010000101010100,
        0b0010001001010010,
        0b0010010001000000,
        0b0010100001000000,
        0b0011000001000000,
        0b0010000001000000,
        0b0000000000000000,
    ],
};

static HAO: Glyph = Glyph {
    rows: [
        0b0000100000010000,
        0b0000100000010000,
        0b0111110001111110,
        0b0001000000000010,
        0b0001000000000100,
        0b0010100000001000,
        0b0010100000010000,
        0b0100010001111100,
        0b1000001000010000,
        0b0000010000010000,
        0b0000100000010000,
        0b0001000000010000,
        0b0010000000010000,
        0b0100000000010000,
        0b0000000000100000,
        0b0000000000000000,
    ],
};

static FULLWIDTH_COMMA: Glyph = Glyph {
    rows: [
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000000000000000,
        0b0000001110000000,
        0b0000001110000000,
        0b0000000100000000,
        0b0000001000000000,
        0b0000010000000000,
        0b0000000000000000,
    ],
};

static SHI: Glyph = Glyph {
    rows: [
        0b0000010001000000,
        0b0000010001000000,
        0b0111111111111110,
        0b0001010001000000,
        0b0001010001000000,
        0b0001010001000000,
        0b0001011111000000,
        0b0001010001000000,
        0b0001010001000000,
        0b0001010001000000,
        0b0001111111000000,
        0b0001000000000000,
        0b0001000000000010,
        0b0001111111111110,
        0b0000000000000000,
        0b0000000000000000,
    ],
};

static JIE: Glyph = Glyph {
    rows: [
        0b0011111111111000,
        0b0010001000101000,
        0b0010001000101000,
        0b0011111111111000,
        0b0010001000101000,
        0b0010001000101000,
        0b0011111111111000,
        0b0000010001000000,
        0b0000100000100000,
        0b0001000000010000,
        0b0011111111111000,
        0b0001001001000000,
        0b0001001001000000,
        0b0010001000100000,
        0b0100001000010000,
        0b0000000000000000,
    ],
};

#[cfg(test)]
mod tests {
    use super::{Color, FramebufferWriter, PixelFormat, draw_text};
    use std::vec;

    fn has_ink(
        bytes: &[u8],
        pitch: usize,
        x_range: core::ops::Range<usize>,
        y_range: core::ops::Range<usize>,
    ) -> bool {
        for y in y_range {
            for x in x_range.clone() {
                let offset = y * pitch + x * 4;
                if bytes[offset..offset + 4] != [0, 0, 0, 0] {
                    return true;
                }
            }
        }

        false
    }

    #[test]
    fn pixel_format_packs_rgb_using_limine_masks() {
        let format = PixelFormat::new(32, 8, 16, 8, 8, 8, 0).unwrap();

        assert_eq!(format.encode(Color::rgb(0x12, 0x34, 0x56)), 0x0012_3456);
    }

    #[test]
    fn draw_text_renders_hello_world_across_pitch_padded_rows() {
        let width = 128;
        let height = 40;
        let pitch = 640;
        let mut bytes = vec![0; pitch * height];
        let format = PixelFormat::new(32, 8, 16, 8, 8, 8, 0).unwrap();

        {
            let mut writer =
                FramebufferWriter::new(&mut bytes, width, height, pitch, format).unwrap();
            draw_text(
                &mut writer,
                4,
                4,
                "你好，世界",
                Color::rgb(0xff, 0xff, 0xff),
                1,
            );
        }

        let glyph_width = 16;
        let spacing = 2;
        let y_range = 4..20;

        for glyph_index in 0..5 {
            let start_x = 4 + glyph_index * (glyph_width + spacing);
            assert!(
                has_ink(
                    &bytes,
                    pitch,
                    start_x..start_x + glyph_width,
                    y_range.clone()
                ),
                "glyph cell {glyph_index} should contain ink"
            );
        }

        assert_eq!(
            &bytes[width * 4..pitch],
            vec![0; pitch - width * 4].as_slice(),
            "padding bytes after the visible row must not be touched"
        );
    }
}
