use macroquad::prelude::*;
mod editor;
use crate::editor::Grid;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(GREEN);

        let mut grid = Grid::new(32.0);
        grid.draw();
        grid.show_info();

        next_frame().await
    }
}
