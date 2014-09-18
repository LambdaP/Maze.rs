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
    Lime,
    Pacific,
    Purple,
    Stonewall,
    Zinger,
    Pumpkin,
    Tareum,
    Candy,
    Passion,
    Rower,
    Uruguayans,
    Pool,
    Ariel,
    Knickers,
    Hotpink,
    Warmth,
    Wasabi,
    Yogi,
    Poison,
    Lipstick,
    Bordello,
    Turquoise,
    Crayola
}

impl Color {
    pub fn to_rgb (&self) -> Rgb<u8> {
        match *self {
            Lime => Rgb(118,210,0),
            Pacific => Rgb(23,174,166),
            Purple => Rgb(131,21,104),
            Stonewall => Rgb(221,33,13),
            Zinger => Rgb(191,238,74),
            Pumpkin => Rgb(231,126,62),
            Tareum => Rgb(95,84,142),
            Candy => Rgb(248,160,152),
            Passion => Rgb(192,31,49),
            Rower => Rgb(11,76,134),
            Uruguayans => Rgb(126,12,93),
            Pool => Rgb(98,234,200),
            Ariel => Rgb(38,143,171),
            Knickers => Rgb(247,93,108),
            Hotpink => Rgb(194,33,132),
            Warmth => Rgb(254,139,54),
            Wasabi => Rgb(132,226,49),
            Yogi => Rgb(248,84,17),
            Poison => Rgb(194,26,3),
            Lipstick  => Rgb(182,33,69),
            Bordello  => Rgb(219,73,138),
            Turquoise => Rgb(17,182,223),
            Crayola => Rgb(24,148,196),
        }
    }
}

#[deriving(PartialEq)]
enum CellColor {
    White,
    Black,
    C(Color),
}

impl CellColor {
    pub fn to_rgb (&self) -> Rgb<u8> {
        match *self {
            White => Rgb(253,251,249),
            Black => Rgb(17,41,58),
            C(x)  => x.to_rgb(),
        }
    }
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

//    pub fn ugly_print (&self) {
//        for i in range(0, grid_size) {
//            for j in range(0, grid_size) {
//                let c = match self.cells[i][j] {
//                    White => 'O',
//                    Black => '.',
//                    C(x)  => match x {
//                        Red   => 'R',
//                        Green => 'G',
//                        Blue  => 'B',
//                    },
//                };
//                print!("{}", c);
//            }
//            println!("");
//        }
//    }

    pub fn gen_img (&self) {
        let image_size : u32 = (square_size * grid_size) as u32;
        let cl = |x : u32, y : u32| {
            let cc = self.cells[(x / 10) as uint][(y / 10) as uint];
            cc.to_rgb()
        };

        let img : ImageBuf<Rgb<u8>> =
            ImageBuf::from_fn(image_size, image_size, cl);

        let fout = File::create(&Path::new("test.png")).unwrap();

        let _ = image::ImageRgb8(img).save(fout, image::PNG);
    }
}
