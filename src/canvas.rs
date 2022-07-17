use crate::approx_eq::{assert_approx_eq, ApproxEq};
use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

const MAX_COL: usize = 255;

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y * self.width + x] = c;
    }
    pub fn to_ppm(&self) -> String {
        let mut result = format!("P3\n{} {}\n{}\n", self.width, self.height, MAX_COL);
        for y in 0..self.height {
            let mut parts: Vec<f64> = Vec::with_capacity(3 * self.width);
            for x in 0..self.width {
                let color = self.pixel_at(x, y);
                parts.push(color.red);
                parts.push(color.green);
                parts.push(color.blue);
            }
            let mut line = String::new();
            for p in parts {
                let mut c = p * (MAX_COL as f64);
                if c < 0.0 {
                    c = 0.0;
                }
                if c > MAX_COL as f64 {
                    c = MAX_COL as f64;
                }
                let s = (c.round() as usize).to_string();
                if line.len() + 1 + s.len() > 70 {
                    result += &line;
                    result += "\n";
                    line = String::new();
                }
                if line.is_empty() {
                    line = s;
                } else {
                    line += " ";
                    line += &s;
                }
            }
            result += &line;
            result += "\n";
        }
        result
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        let black = Color::new(0.0, 0.0, 0.0);
        for y in 0..20 {
            for x in 0..10 {
                assert_approx_eq!(c.pixel_at(x, y), black);
            }
        }
    }

    #[test]
    fn test_writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);
        assert_approx_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn test_constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert_eq!(
            ppm.lines().take(3).collect::<Vec<_>>(),
            vec!["P3", "5 3", "255"]
        );
    }

    #[test]
    fn test_constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        assert_eq!(
            ppm.lines().skip(3).collect::<Vec<_>>(),
            vec![
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ]
        );
    }

    #[test]
    fn test_splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let col = Color::new(1.0, 0.8, 0.6);
        for y in 0..c.height() {
            for x in 0..c.width() {
                c.write_pixel(x, y, col);
            }
        }
        let ppm = c.to_ppm();
        assert_eq!(
            ppm.lines().skip(3).collect::<Vec<_>>(),
            vec![
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153"
            ]
        );
    }

    #[test]
    fn test_ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();
        assert!(ppm.ends_with("\n"));
    }
}
