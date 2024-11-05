use macroquad::{color::*, shapes::*, text::*, window::*};

pub struct Grid {
    active: bool,
    color: macroquad::color::Color,
    size: f32,
    vertical_lines: u64,
    horizontal_lines: u64,
}

impl Grid {
    pub fn new(size: f32) -> Self {
        Grid {
            size,
            active: true,
            color: WHITE,
            vertical_lines: 0,
            horizontal_lines: 0,
        }
    }

    pub fn draw(&mut self) {
        if self.active {
            // drawing horizontal lines
            while self.horizontal_lines as f32 * self.size < screen_height() {
                self.horizontal_lines = self.horizontal_lines + 1;
                let position_y = self.horizontal_lines as f32 * self.size;
                draw_line(0.0, position_y, screen_width(), position_y, 1.0, self.color);
            }

            // drawing vertical lines
            while self.vertical_lines as f32 * self.size < screen_width() {
                self.vertical_lines = self.vertical_lines + 1;
                let position_x = self.vertical_lines as f32 * self.size;
                draw_line(
                    position_x,
                    0.0,
                    position_x,
                    screen_height(),
                    1.0,
                    self.color,
                );
            }
        }
    }

    pub fn show_info(&self) {
        if self.active {
            let horizontal_lines_number = format!(
                "{}{}",
                "number of horizontal lines: ", self.horizontal_lines
            );
            let vertical_lines_number =
                format!("{}{}", "number of vertical lines: ", self.vertical_lines);
            draw_text(horizontal_lines_number.as_str(), 20.0, 20.0, 14.0, BLACK);
            draw_text(vertical_lines_number.as_str(), 20.0, 30.0, 14.0, BLACK);
        }
    }
}
