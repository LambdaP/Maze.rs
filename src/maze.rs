use std::rand;

static grid_size : uint = 4;

type Coord = (uint, uint);

#[deriving(Rand)]
enum Color {
    White
}

enum Cell {
    Wall,
    Path(Color),
}

struct Grid {
    cells : [[Cell, ..grid_size], ..grid_size],
    path_in_construction : Vec<Coord>,
}

impl Grid {

    fn new() -> Grid {
        Grid {
            cells : [[Wall, ..grid_size], ..grid_size],
            path_in_construction : Vec::new()
        }
    }

    fn collect_neighbours (&mut self, x : uint, y : uint)
                             -> Vec<(Coord, Cell)> {
        let mut neighbours = Vec::with_capacity(4);

        if x > 1 {
            neighbours.push(((x-1, y), self.cells[x-1][y]));
        }
        if y > 1 {
            neighbours.push(((x, y-1), self.cells[x][y-1]));
        }
        if x < grid_size - 1 {
            neighbours.push(((x+1, y), self.cells[x+1][y]));
        }
        if y < grid_size - 1 {
            neighbours.push(((x, y+1), self.cells[x][y+1]));
        }

        return neighbours;
    }

    fn near_maze (&mut self, x : uint, y : uint) -> bool {
        let neighbours = self.collect_neighbours(x, y);

        for neigh in neighbours.iter() {
            match *neigh {
                (_, Path(_)) => return true,
                _            => continue,
            }
        }

        return false;
    }

    fn near_path (&mut self, x : uint, y : uint) -> bool {
        let neighbours = self.collect_neighbours(x, y);

        let mut count : int = 0;

        for neigh in neighbours.iter() {
            match *neigh {
                (cs, _) => if self.path_in_construction.contains(&cs) {
                count += 1;
                }
            }
        }

        return (count > 1);
    }

    fn commit_path (&mut self) {
        let color : Color = std::rand::random();

        for cell in self.path_in_construction.iter() {
            match *cell {
                (x, y) => self.cells[x][y] = Path(color)
            }
        }

        self.path_in_construction.clear();
    }

}

fn main () {
    println!("In progress.");
}
