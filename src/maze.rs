#[allow(dead_code)]
static grid_size : uint = 9;

type Coord = (uint, uint);

#[deriving(PartialEq,Rand)]
enum Color {
    White
}

#[deriving(PartialEq)]
enum CellType {
    Wall,
    Path(Color),
}

#[allow(dead_code)]
struct Grid {
    cells : [[CellType, ..grid_size], ..grid_size],
    path_in_construction : Vec<(Coord, Vec<Coord>)>,
}

#[allow(dead_code)]
impl Grid {

    fn new() -> Grid {
        Grid {
            cells : [[Wall, ..grid_size], ..grid_size],
            path_in_construction : Vec::new()
        }
    }

    fn at (&mut self, coords : Coord) -> CellType {
        match coords {
            (x, y) => self.cells[x][y]
        }
    }

    fn collect_neighbours (&mut self, coords : Coord)
                             -> Vec<(Coord)> {
        let mut neighbours = Vec::with_capacity(4);

        let (x, y) = coords;

        if x > 1 {
            neighbours.push((x-1, y));
        }
        if y > 1 {
            neighbours.push((x, y-1));
        }
        if x + 1 < grid_size {
            neighbours.push((x+1, y));
        }
        if y + 1 < grid_size {
            neighbours.push((x, y+1));
        }

        return neighbours;
    }

    fn near_maze (&mut self, coords : Coord) -> bool {
        let neighbours = self.collect_neighbours(coords);

        for &cs in neighbours.iter() {
            match self.at(cs) {
                Path(_) => return true,
                _       => continue,
            }
        }

        return false;
    }

    fn in_path (&mut self, coords : Coord) -> bool {
        for cell in self.path_in_construction.iter() {
            match *cell {
                (cs, _) if cs == coords => return true,
                _   => continue
            }
        }

        return false;
    }

    fn near_path (&mut self, coords : Coord) -> bool {
        let neighbours = self.collect_neighbours(coords);

        let mut count : int = 0;

        for &cs in neighbours.iter() {
            if self.in_path(cs) {
                count += 1;
            }
        }

        return count > 1;
    }

    fn explorable_neighbours (&mut self, coords : Coord)
                                     -> Vec<Coord> {
        let neighbours = self.collect_neighbours(coords);
        let mut explorable = Vec::with_capacity(4);

        for &cs in neighbours.iter() {
            if self.at(cs) == Wall || self.near_path(cs) {
                continue;
            }

            explorable.push(cs);
        }

        return explorable;
    }

    fn select_at_random<T> (leads : &mut Vec<T>) -> Option<T> {
        leads.pop() // TODO: do something more random.
    }

    fn add_cell_to_path(&mut self, coords : Coord) {
        let ns = self.explorable_neighbours(coords);

        self.path_in_construction.push((coords, ns));
    }

    fn extend_path (&mut self) -> bool {
        let option_new_cell =
            Grid::select_at_random(self.path_in_construction
                                       .get_mut(0)
                                       .mut1());

        match option_new_cell {
            Some(cs) => self.add_cell_to_path(cs),
            None        => return false
        }

        return true;
    }

    fn new_path_origin (&mut self, coords : Coord) {
        let neighbours = self.collect_neighbours(coords);

        self.path_in_construction.push((coords, neighbours));
    }

    fn backtrack (&mut self) {
        self.path_in_construction.pop();
    }

    fn new_path (&mut self, coords : Coord) {
        self.new_path_origin(coords);

        loop {
            let cs = *(self.path_in_construction[0].ref0());

            if self.near_maze(cs) {
                break;
            }

            if !self.extend_path() {
                self.backtrack();
            }
        }
    }

//    fn commit_path (&mut self) {
//        let color : Color = std::rand::random();
//
//        for cell in self.path_in_construction.iter() {
//            match *cell {
//                (x, y) => self.cells[x][y] = Path(color)
//            }
//        }
//
//        self.path_in_construction.clear();
//    }

}

fn main () {
    println!("In progress.");
}
