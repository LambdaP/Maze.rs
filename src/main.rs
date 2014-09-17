pub mod maze;
pub mod maze_display;

fn main () {
    let mut g = maze::Grid::new();
    g.run();
    let gc = maze_display::ColoredGrid::from_grid(g);
    gc.gen_img();
}
