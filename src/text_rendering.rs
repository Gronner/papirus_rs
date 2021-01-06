const FONT_PATH: &str = "/usr/share/fonts/truetype/freefont/FreeMono.ttf";

use rusttype::{point, Font, Scale};
use image::{DynamicImage, Luma, ImageBuffer, Pixel};

pub struct ImageSize {
    x_size: u32,
    y_size: u32,
}

pub struct TextRenderer<'a> {
    font: Font<'a>,
    scale: Scale,
    size: ImageSize,
}

impl<'a> TextRenderer<'a> {
    pub fn new(font_path: &str, height_px: f32, width_scale_factor: f32, size: ImageSize) -> TextRenderer {
        let font = Font::try_from_vec(std::fs::read(&font_path).unwrap()).unwrap();
        let scale = Scale {
            x: height_px * width_scale_factor,
            y: height_px,
        };
        TextRenderer {
            font: font,
            scale: scale,
            size: size,
        }
    }

    pub fn render_image(&self, text: &str) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut image = DynamicImage::new_luma8(self.size.x_size, self.size.y_size).to_luma8();
        for (row, line) in text.split("\n").enumerate() {
            self.set_line(line, &mut image, (row + 1) as f32);
        }
        let image = DynamicImage::ImageLuma8(image);
        image.to_luma8()
    }

    pub fn render_text(&self, text: &str) -> Vec<u8> {
        let image = self.render_image(text);
        let mut data = Vec::new();
        let mut byte: u8 = 0;
        for pixel in image.enumerate_pixels() {
            if 0 < pixel.2.channels()[0] {
                let bit_offset = pixel.0 % 8;
                byte |= 1 << 7 - bit_offset;
            }
            if ((pixel.0 + 1) % 8) == 0 || pixel.0 == 199 {
                data.push(byte.reverse_bits());
                byte = 0;
            }
        }
        data
    }

    fn set_line(&self, line: &str, image: &mut ImageBuffer<Luma<u8>, Vec<u8>>, row: f32) {
        let v_metrics = self.font.v_metrics(self.scale);
        let glyphs: Vec<_> = self.font
            .layout(line, self.scale, point(3.0, 3.0 + (v_metrics.ascent + 5.0) * row))
            .collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        Luma([(v * 255.0) as u8]),
                    )
                });
            }
        }
    }
}

impl<'a> Default for TextRenderer<'a> {
    fn default() -> TextRenderer<'a> {
        let font = Font::try_from_vec(std::fs::read(FONT_PATH).unwrap()).unwrap();
        let scale = Scale::uniform(18.0);
        TextRenderer {
            font: font,
            scale: scale,
            size: ImageSize {
                x_size: 200,
                y_size: 90,
            },
        }
    }
}
