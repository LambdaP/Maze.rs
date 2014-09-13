use std::rand;
use std::collections::TreeSet;

static grid_size : uint = 9;

type Coord = (uint, uint);

#[deriving(PartialEq,Show)]
enum CellType {
    Wall,
    Path(uint),
}

pub struct Grid {
    cells                : [[CellType, ..grid_size], ..grid_size],
    path_in_construction : Vec<(Coord, Vec<Coord>)>,
    clear_cells          : TreeSet<Coord>,
    npath                : uint,
}

impl Grid {
    pub fn new() -> Grid {
        let mut g = Grid {
            cells : [[Wall, ..grid_size], ..grid_size],
            path_in_construction : Vec::new(),
            clear_cells : TreeSet::new(),
            npath : 0,
        };

        for x in range(0, grid_size) {
            for y in range(0, grid_size) {
                g.clear_cells.insert((x,y));
            }
        }

        return g;
    }

    fn at (&self, coords : Coord) -> CellType {
        match coords {
            (x, y) => self.cells[x][y]
        }
    }

    fn collect_neighbours (&self, coords : Coord)
                             -> Vec<(Coord)> {
        let mut neighbours = Vec::with_capacity(4);

        let (x, y) = coords;

        if x > 0 {
            neighbours.push((x-1, y));
        }
        if y > 0 {
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

    fn near_maze (&self, coords : Coord) -> bool {
        let neighbours = self.collect_neighbours(coords);

        for &cs in neighbours.iter() {
            match self.at(cs) {
                Path(_) => return true,
                _       => continue,
            }
        }

        return false;
    }

    fn in_path (&self, coords : Coord) -> bool {
        for cell in self.path_in_construction.iter() {
            match *cell {
                (cs, _) if cs == coords => return true,
                _   => continue
            }
        }

        return false;
    }

    fn near_path (&self, coords : Coord) -> bool {
        let neighbours = self.collect_neighbours(coords);

        let mut count : int = 0;

        for &cs in neighbours.iter() {
            if self.in_path(cs) {
                count += 1;
            }
        }

        return count > 0;
    }

    fn near_several_maze (&self, coords : Coord) -> bool {
        let neighbours = self.collect_neighbours(coords);

        let mut count : int = 0;

        for &cs in neighbours.iter() {
            match self.at(cs) {
                Wall => continue,
                _    => count += 1
            }
        }

        return count > 1;
    }

    fn cjm (&self, origin : Coord, connex : &mut TreeSet<Coord>) -> bool {
        if self.near_maze(origin) {
            return true;
        }

        let neighbours = self.collect_neighbours(origin);

        // Scan as far as you can.
        for &cs in neighbours.iter() {
            if connex.insert(cs) {

                // Don't cross the path.
                if self.near_path(cs)
                    || self.in_path(cs) {
                    continue;
                }

                // Explore recursively.
                if self.cjm(cs, connex) {
                    return true;
                }
            }
        }

        return false;
    }

    fn can_join_maze (&self, coords : Coord) -> bool {
        let mut connex : TreeSet<Coord> = TreeSet::new();
        connex.insert(coords);

        return self.cjm(coords, &mut connex);
    }

    fn explorable_neighbours (&self, coords : Coord)
                                     -> Vec<Coord> {
        let neighbours = self.collect_neighbours(coords);
        let mut explorable = Vec::with_capacity(4);

        for &cs in neighbours.iter() {
            if self.in_path(cs)
                || self.near_path(cs)
                /* || self.near_several_maze(cs) */ {
                continue;
            }

            explorable.push(cs);
        }

        return explorable;
    }

    fn randab (a : uint, b : uint) -> uint {
        a + (rand::random::<uint>() % (b - a))
    }

    fn select_at_random<T> (leads : &mut Vec<T>) -> Option<T> {
        let l = leads.len();

        if l == 0 {
            return None;
        }

        leads.remove(Grid::randab(0, l))
    }

    fn add_cell_to_path(&mut self, coords : Coord) {
        let ns = self.explorable_neighbours(coords);

        self.path_in_construction.push((coords, ns));
    }

    fn extend_path (&mut self) -> bool {
        let option_new_cell = {
            let mut rem0 = false;
            let mut rem1 = false;

            { // Test for connexity with the maze.
                let expl = match self.path_in_construction.last() {
                    Some(x) => (*x).ref1(),
                    None    => fail!("Empty path.")
                };

                if expl.len() == 2 {
                    if !self.can_join_maze(expl[0]) {
                        rem0 = true;
                    }
                    if !self.can_join_maze(expl[1]) {
                        rem1 = true;
                    }
                }
            }

            let expl = match self.path_in_construction.mut_last(){
                    Some(x) => (*x).mut1(),
                    None => fail!("Empty path.")
                };

            if rem0 {
                if rem1 {
                    expl.remove(1);
                }
                expl.remove(0);
            }

            if rem1 {
                expl.remove(1);
            }

            Grid::select_at_random(expl)
        };

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
            let cs = {
                let option_last = self.path_in_construction.last();

                match option_last {
                    Some(x) => *x.ref0(),
                    None => fail!("Empty path.")
                }
            };

            if self.near_maze(cs) {
                break;
            }

            if !self.extend_path() {
                self.backtrack();
            }
        }
    }

    fn update_clear_cells (&mut self) {
        for cell in self.path_in_construction.iter() {
            match *cell {
                (cs, _) => {
                    self.clear_cells.remove(&cs);
                    let ns = self.collect_neighbours(cs);
                    for c in ns.iter() {
                        if self.near_several_maze(*c) {
                            self.clear_cells.remove(c);
                        }
                    }
                }
            }
        }
    }

    fn commit_path (&mut self) {
        for cell in self.path_in_construction.iter() {
            match *cell {
                ((x, y), _) => self.cells[x][y] = Path(self.npath)
            }
        }

        self.npath += 1;

        self.update_clear_cells();

        self.path_in_construction.clear();
    }

    fn new_random_origin (&self) -> Coord {
        let l = self.clear_cells.len();
        let ri = Grid::randab(0, l);
        match self.clear_cells.iter().skip(ri).next() {
            None => fail!("Could not pick new origin."),
            Some(x) => *x
        }
    }

    fn set_origin (&mut self, coord : Coord) {
        self.new_path_origin(coord);
        self.commit_path();
    }

    pub fn run (&mut self) {
        self.set_origin((0,0));

        while !self.clear_cells.is_empty() {
            let cs = self.new_random_origin();
            self.new_path(cs);
            self.commit_path();
        }

        self.ugly_print();

    }

    fn hexchar (u : uint) -> char {
        let v = u % 16;

        match v {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'a',
            11 => 'b',
            12 => 'c',
            13 => 'd',
            14 => 'e',
            15 => 'f',
            _  => '?',
        }
    }

    fn cell_rep (c : CellType) -> char {
        match c {
            Wall => '.',
            Path(x) => Grid::hexchar(x),
        }
    }

    fn ugly_print (&self) {

        for i in range(0, grid_size) {
            for j in range(0, grid_size) {
                print!("{}", Grid::cell_rep(self.cells[i][j]));
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod test {
    use super::Grid;

    #[test]
    fn test_at() {
        let mut g = Grid::new();

        assert_eq!(g.at((0,0)), super::Wall);
    }

    #[test]
    fn test_collect_neighbours() {
        let mut g = Grid::new();

        {
            let ns = g.collect_neighbours((0,0));

            assert_eq!(ns.len(), 2);

            {
                let (x, y) = ns[0];
                assert_eq!(x, 1);
                assert_eq!(y, 0);
            }

            {
                let (x, y) = ns[1];
                assert_eq!(x, 0);
                assert_eq!(y, 1);
            }
        }

        {
            let ns = g.collect_neighbours((1,1));

            assert_eq!(ns.len(), 4);

            {
                let (x, y) = ns[0];
                assert_eq!(x, 0);
                assert_eq!(y, 1);
            }

            {
                let (x, y) = ns[1];
                assert_eq!(x, 1);
                assert_eq!(y, 0);
            }

            {
                let (x, y) = ns[2];
                assert_eq!(x, 2);
                assert_eq!(y, 1);
            }

            {
                let (x, y) = ns[3];
                assert_eq!(x, 1);
                assert_eq!(y, 2);
            }
        }

    }

    #[test]
    fn test_near_maze() {
        let mut g = Grid::new();

        g.cells[0][0] = super::Path(super::White);

        assert!(g.near_maze((0,1)));
        assert!(!g.near_maze((1,1)));
    }

    #[test]
    fn test_in_path() {
        let mut g = Grid::new();

        let v = Vec::new();
        g.path_in_construction.push(((0,0),v));

        assert!(g.in_path((0,0)));
        assert!(!g.in_path((1,1)));
    }

    #[test]
    fn test_near_path() {
        let mut g = Grid::new();

        let v = Vec::new();
        g.path_in_construction.push(((0,0),v));

        assert!(g.near_path((1,0)));
        assert!(!g.near_path((1,1)));
    }

    #[test]
    fn test_explorable_neighbours() {
        let mut g = Grid::new();

        let v1 = Vec::new();
        let v2 = Vec::new();
        g.path_in_construction.push(((0,0),v1));
        g.path_in_construction.push(((0,1),v2));

        let ns = g.explorable_neighbours((1,1));

        assert_eq!(ns.len(), 2);
    }

    #[test]
    fn test_add_cell_to_path() {
        let mut g = Grid::new();

        let v1 = Vec::new();
        let v2 = Vec::new();
        g.path_in_construction.push(((0,0),v1));
        g.path_in_construction.push(((0,1),v2));

        assert_eq!(g.path_in_construction.len(), 2);

        g.add_cell_to_path((1,1));

        assert_eq!(g.path_in_construction.len(), 3);
    }

    #[test]
    fn test_new_path_origin() {
        let mut g = Grid::new();

        assert_eq!(g.path_in_construction.len(), 0);

        g.new_path_origin((0,0));
        assert_eq!(g.path_in_construction.len(), 1);
    }

    #[test]
    fn test_extend_path_1() {
        let mut g = Grid::new();

        g.new_path_origin((0,0));
        g.extend_path();

        assert_eq!(g.path_in_construction.len(), 2);

        let cs = *(g.path_in_construction[1].ref0());

        assert_eq!(cs.val0() + cs.val1(), 1);
    }

    #[test]
    fn test_extend_path_2() {
        let mut g = Grid::new();

        let v1 = Vec::new();
        g.path_in_construction.push(((0,0),v1));

        assert!(!g.extend_path());
    }

    #[test]
    fn test_new_path() {
        let mut g = Grid::new();

        g.cells[1][1] = super::Path(super::White);

        g.new_path((0,0));

        assert_eq!(g.path_in_construction.len(), 2);
    }

    #[test]
    fn test_update_clear_cells() {
        let mut g = Grid::new();

        g.new_path_origin((0,0));

        assert!(g.clear_cells.contains(&(0,0)));
        assert!(g.clear_cells.contains(&(0,1)));
        assert!(g.clear_cells.contains(&(1,0)));

        g.update_clear_cells();

        assert!(!g.clear_cells.contains(&(0,0)));
        assert!(!g.clear_cells.contains(&(0,1)));
        assert!(!g.clear_cells.contains(&(1,0)));
    }

    #[test]
    fn test_commit_path() {
        let mut g = Grid::new();

        g.cells[1][1] = super::Path(super::White);

        g.add_cell_to_path((0,0));
        g.add_cell_to_path((0,1));

        g.commit_path();

        assert_eq!(g.cells[0][0], super::Path(super::White));
        assert_eq!(g.cells[0][1], super::Path(super::White));
    }
}
