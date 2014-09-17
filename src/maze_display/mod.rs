extern crate image;
use maze;

use std::rand;
use self::image::{
    ImageBuf,
    Rgb,
};

use std::io::File;

static square_size : uint = 10;
static grid_size : uint = maze::grid_size;

#[deriving(PartialEq,Rand,Show)]
enum Color {
    Red,
    Green,
    Blue,
}

#[deriving(PartialEq)]
enum CellColor {
    White,
    Black,
    C(Color),
}

pub struct ColoredGrid {
    cells : [[CellColor, ..grid_size], ..grid_size]
}

impl ColoredGrid {
    pub fn from_grid (g : maze::Grid) -> ColoredGrid {
        let mut cg = ColoredGrid {
            cells : [[Black, ..grid_size], ..grid_size]
        };

        let mut cv = Vec::with_capacity(g.npath);

        for i in range(0,g.npath) {
            let c : Color = rand::random::<Color>();
            cv.push(c);
        }

        for i in range(0, grid_size) {
            for j in range(0, grid_size) {
                let cc = match g.cells[i][j] {
                    maze::Wall              => Black,
                    maze::Path(n) if n == 0 => White,
                    maze::Path(n)           => C(cv[n-1]),
                };
                cg.cells[i][j] = cc;
            }
        }

        cg
    }

    pub fn ugly_print (&self) {
        for i in range(0, grid_size) {
            for j in range(0, grid_size) {
                let c = match self.cells[i][j] {
                    White => 'O',
                    Black => '.',
                    C(x)  => match x {
                        Red   => 'R',
                        Green => 'G',
                        Blue  => 'B',
                    },
                };
                print!("{}", c);
            }
            println!("");
        }
    }

    pub fn gen_img (&self) {
        let image_size : u32 = (square_size * grid_size) as u32;
        let cl = |x : u32, y : u32| {
            match self.cells[(x / 10) as uint][(y / 10) as uint] {
                White => Rgb(255,255,255),
                Black => Rgb(0,0,0),
                C(x)  => match x {
                    Red   => Rgb(255,0,0),
                    Green => Rgb(0,255,0),
                    Blue  => Rgb(0,0,255)
                },
            }
        };
        let img : ImageBuf<Rgb<u8>> =
            ImageBuf::from_fn(image_size, image_size, cl);

        let fout = File::create(&Path::new("test.png")).unwrap();

        let _ = image::ImageRgb8(img).save(fout, image::PNG);
    }
}
