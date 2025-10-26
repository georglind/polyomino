use crate::parser::parse;
use algox::algox::Matrix;
use std::collections::HashSet;
use std::ops;

// Size
#[derive(Debug, PartialEq, Eq)]
struct Size {
    width: usize,
    height: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Size {
            width: width,
            height: height,
        }
    }
}

// Point
#[derive(PartialEq, Eq, Hash, Debug, Clone, PartialOrd, Ord)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x: x, y: y }
    }

    pub fn from(x: &usize, y: &usize) -> Self {
        Point {
            x: isize::try_from(*x)
                .ok()
                .expect("Could not convert to isize"),
            y: isize::try_from(*y)
                .ok()
                .expect("Could not convert to isize"),
        }
    }
}

impl ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x.clone(),
            y: -self.y.clone(),
        }
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Ord, PartialOrd, Debug)]
struct Tile {
    name: String,
    points: Vec<Point>,
}

impl Tile {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
        }
    }

    pub fn from_str(name: &str, contents: &str) -> Self {
        let mut row: usize = 0;
        let mut col: usize = 0;

        let mut points = Vec::new();

        for c in contents.chars() {
            match c {
                'x' => {
                    points.push(Point::from(&col, &row));
                    col += 1;
                }
                '\n' => {
                    row += 1;
                    col = 0;
                }
                _ => col += 1,
            }
        }

        let mut tile = Self {
            name: name.to_string(),
            points: points,
        };
        // move tile top-left to origo
        tile.translate(&(-tile.offset()));
        return tile;
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    // mirror x (along y-axis).
    pub fn mirror(&mut self) {
        for p in self.points.iter_mut() {
            p.x = -p.x;
        }
    }

    // rotate 90 degrees counter-clockwise.
    pub fn rotate(&mut self) {
        let mut v: isize;
        for p in self.points.iter_mut() {
            v = p.x;
            p.x = p.y;
            p.y = -v.clone();
        }
    }

    pub fn translate(&mut self, offset: &Point) {
        for p in self.points.iter_mut() {
            p.x += offset.x;
            p.y += offset.y;
        }
    }

    // top left offset
    pub fn offset(&self) -> Point {
        let mut x: isize = self.points[0].x;
        let mut y: isize = self.points[0].y;

        for p in self.points.iter() {
            if p.x < x {
                x = p.x;
            }
            if p.y < y {
                y = p.y;
            }
        }

        Point::new(x, y)
    }

    pub fn index(&self, point: &Point) -> Option<usize> {
        self.points.iter().position(|r| r == point)
    }

    pub fn size(&self) -> Size {
        if self.points.len() == 0 {
            return Size {
                width: 0,
                height: 0,
            };
        }

        let mut xmin = self.points[0].x;
        let mut xmax = xmin;
        let mut ymin = self.points[0].y;
        let mut ymax = ymin;

        for p in self.points.iter() {
            if p.x > xmax {
                xmax = p.x;
            }
            if p.x < xmin {
                xmin = p.x;
            }
            if p.y > ymax {
                ymax = p.y;
            }
            if p.y < ymin {
                ymin = p.y;
            }
        }
        Size {
            width: usize::try_from(xmax - xmin + 1).unwrap(),
            height: usize::try_from(ymax - ymin + 1).unwrap(),
        }
    }
}

pub struct Game {
    board: Tile,
    tiles: Vec<Tile>,
}

impl Game {
    pub fn from_yaml(yaml: &str) -> Self {
        let contents = parse(yaml).unwrap();
        let mut board: Tile = Tile::new("Board");
        let mut tiles: Vec<Tile> = Vec::new();

        for (name, part) in contents.iter() {
            let tile = Tile::from_str(name, part);
            match *name {
                "Board" => {
                    board = tile;
                }
                _ => {
                    tiles.push(tile);
                }
            }
        }
        Self {
            board: board,
            tiles: tiles,
        }
    }

    pub fn len(&self) -> usize {
        self.board.points.len()
    }

    // Build our data structure from the existing game.
    pub fn build_matrix(&mut self) -> Matrix {
        let n_cols = self.board.len() + self.tiles.len();
        let mut m = Matrix::new(n_cols);

        let mut uniqs: HashSet<(Tile, usize)> = HashSet::new();
        for (index, tile) in self.tiles.iter().enumerate() {
            let mut t = tile.clone();
            for o in 0..8 {
                t.rotate();
                if o == 4 {
                    t.mirror();
                }
                t.translate(&-t.offset());
                t.points.sort();

                uniqs.insert((t.clone(), index));
            }
        }

        // hash set of board points for easy checking
        let board_points: HashSet<Point> = HashSet::from_iter(self.board.points.iter().cloned());
        let size = self.board.size();

        // order tiles predictably
        let mut uniqs = Vec::from_iter(uniqs.iter());
        uniqs.sort();

        for (tile, index) in uniqs.iter() {
            let mut t = tile.clone();
            for i in 0..isize::try_from(size.width).unwrap() {
                for j in 0..isize::try_from(size.height).unwrap() {
                    t.translate(&(Point::new(i, j) - t.offset()));

                    let mut contains = true;
                    for point in &t.points {
                        if !board_points.contains(&point) {
                            contains = false;
                            break;
                        }
                    }
                    if !contains {
                        continue;
                    }

                    // build row
                    let mut row = Vec::with_capacity(t.points.len() + 1);
                    for point in &t.points {
                        let p = self.board.index(&point).unwrap();
                        row.push(p);
                    }
                    row.push(self.board.len() + index);

                    m.add_row(&row);
                }
            }
        }
        return m;
    }

    pub fn solve(&mut self) -> (Matrix, Vec<Vec<usize>>) {
        let mut m = self.build_matrix();
        let solutions = m.solve();
        return (m, solutions);
    }

    pub fn solution(&self, m: Matrix, solution: Vec<usize>) -> Vec<Tile> {
        let mut tiles: Vec<Tile> = Vec::new();

        for r in solution {
            let indices = m.row(r);

            let mut points: Vec<Point> = Vec::new();
            let mut name = "";
            for i in indices {
                if i < self.board.len() {
                    points.push(self.board.points[i].clone());
                } else {
                    name = &self.tiles[i].name;
                }
            }
            tiles.push(Tile {
                name: name.to_string(),
                points: points,
            });
        }
        tiles
    }
}

#[cfg(test)]
mod test {
    use super::Game;
    use super::Point;
    use super::Size;
    use super::Tile;

    #[test]
    fn point() {
        let point1 = Point::new(1, 2);
        let point2 = Point::new(3, 4);

        assert_eq!(-point1.clone(), Point::new(-1, -2));
        assert_eq!(point1 + point2, Point::new(4, 6));
    }

    #[test]
    fn board() {
        // New
        let board = Tile::new("Board");
        assert_eq!(board.size(), Size::new(0, 0));

        // From string
        let contents = "xxx-xx\nxxxx-";
        let board = Tile::from_str("Board", &contents);

        assert!(board.points.contains(&Point::new(0, 0)));
        assert!(board.points.contains(&Point::new(3, 1)));
        assert!(!board.points.contains(&Point::new(3, 0)))
    }

    #[test]
    fn tile() {
        // Empty
        let mut tile = Tile::new("X");
        tile.rotate();
        tile.mirror();
        tile.translate(&Point::new(1, 1));

        // From str
        let contents = "xxx-xx\nxxxx-";
        let mut tile = Tile::from_str("X", &contents);

        assert_eq!(tile.points[0], Point::new(0, 0));
        assert_eq!(tile.points[1], Point::new(1, 0));
        assert_eq!(tile.points[8], Point::new(3, 1));

        tile.mirror();
        assert_eq!(tile.points[0], Point::new(0, 0));
        assert_eq!(tile.points[1], Point::new(-1, 0));
        assert_eq!(tile.points[8], Point::new(-3, 1));

        tile.rotate();
        assert_eq!(tile.points[0], Point::new(0, 0));
        assert_eq!(tile.points[1], Point::new(0, 1));
        assert_eq!(tile.points[8], Point::new(1, 3));

        tile.translate(&Point::new(1, 2));
        assert_eq!(tile.points[0], Point::new(1, 2));
        assert_eq!(tile.points[1], Point::new(1, 3));
        assert_eq!(tile.points[8], Point::new(2, 5));

        let point = tile.offset();
        assert_eq!(point, Point::new(1, 2));

        tile.translate(&-point);
        assert_eq!(tile.points[0], Point::new(0, 0));
        assert_eq!(tile.points[1], Point::new(0, 1));
        assert_eq!(tile.points[8], Point::new(1, 3));
    }

    #[test]
    fn solve1() {
        // From string
        let contents = "xx\nxx";
        let board = Tile::from_str("Board", &contents);

        let mut tiles: Vec<Tile> = Vec::new();
        tiles.push(Tile::from_str("T1", "xx\nx"));
        tiles.push(Tile::from_str("T2", "x"));

        let mut game = Game {
            board: board,
            tiles: tiles,
        };

        let (m, solutions) = game.solve();
        assert_eq!(solutions.len(), 4);
    }

    #[test]
    fn solve2() {
        // From string
        let board = Tile::from_str("Board", "xxxxx\nxxxxx\nxxxxx\nxxxxx");

        let tiles: Vec<Tile> = vec![
            Tile::from_str("T1", "xxxx\n x  "),
            Tile::from_str("T2", "xxxx\n x  "),
            Tile::from_str("P1", "xxx\nxx "),
            Tile::from_str("P2", "xxx\nxx "),
        ];

        let mut game = Game {
            board: board,
            tiles: tiles,
        };
        assert_eq!(game.len(), 20);

        let (m, solutions) = game.solve();
        assert_eq!(solutions.len(), 48);

        assert_eq!(solutions[0], vec!(25, 457, 997, 1315));
    }
}
