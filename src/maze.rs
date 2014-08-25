#![warn(unstable)]

use std::rand;

#[allow(dead_code)]
static grid_size : uint = 9;

type Coord = (uint, uint);

#[deriving(PartialEq,Rand,Show)]
enum Colour {
    White
}

#[deriving(PartialEq,Show)]
enum CellType {
    Wall,
    Path(Colour),
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

        return count > 0;
    }

    fn explorable_neighbours (&mut self, coords : Coord)
                                     -> Vec<Coord> {
        let neighbours = self.collect_neighbours(coords);
        let mut explorable = Vec::with_capacity(4);

        for &cs in neighbours.iter() {
            if self.in_path(cs) || self.near_path(cs) {
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

    fn commit_path (&mut self) {
        let color : Colour = rand::random();

        for cell in self.path_in_construction.iter() {
            match *cell {
                ((x, y), _) => self.cells[x][y] = Path(color)
            }
        }

        self.path_in_construction.clear();
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

}
