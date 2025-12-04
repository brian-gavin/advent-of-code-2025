use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Coord {
    row: isize,
    col: isize,
}

impl<T: Into<isize>> From<(T, T)> for Coord {
    fn from((row, col): (T, T)) -> Self {
        Self {
            row: row.into(),
            col: col.into(),
        }
    }
}

impl Coord {
    pub fn origin() -> Coord {
        Coord { row: 0, col: 0 }
    }

    pub fn north(self, n: isize) -> Coord {
        Coord {
            row: self.row - n,
            ..self
        }
    }

    pub fn east(self, n: isize) -> Coord {
        Coord {
            col: self.col + n,
            ..self
        }
    }

    pub fn south(self, n: isize) -> Coord {
        Coord {
            row: self.row + n,
            ..self
        }
    }

    pub fn west(self, n: isize) -> Coord {
        Coord {
            col: self.col - n,
            ..self
        }
    }

    pub fn northeast(self, n: isize) -> Coord {
        self.north(n).east(n)
    }

    pub fn southeast(self, n: isize) -> Coord {
        self.south(n).east(n)
    }

    pub fn southwest(self, n: isize) -> Coord {
        self.south(n).west(n)
    }

    pub fn northwest(self, n: isize) -> Coord {
        self.north(n).west(n)
    }

    /// produces all 8 neighbors of self, in a clockwise order starting from N.
    /// N, NE, E, SE, S, SW, W, NW
    pub fn neighbors(self) -> [Coord; 8] {
        [
            self.north(1),
            self.northeast(1),
            self.east(1),
            self.southeast(1),
            self.south(1),
            self.southwest(1),
            self.west(1),
            self.northwest(1),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Grid<V> {
    map: HashMap<Coord, V>,
}

impl<V: From<char>> Grid<V> {
    pub fn from_input(s: &str) -> Self {
        Self::from_input_fn(s, V::from)
    }
}

impl<V> Grid<V> {
    pub fn from_input_fn<F>(s: &str, from_char: F) -> Self
    where
        F: Fn(char) -> V,
    {
        Self::from_iter(s.lines().enumerate().flat_map(|(row, l)| {
            l.char_indices()
                .map(|(col, c)| (col, from_char(c)))
                .map(move |(col, v)| (Coord::from((row as isize, col as isize)), v))
        }))
    }
}

impl<V> Grid<V> {
    pub fn at(&self, coord: Coord) -> Option<&V> {
        self.map.get(&coord)
    }

    pub fn at_mut(&mut self, coord: Coord) -> Option<&mut V> {
        self.map.get_mut(&coord)
    }

    pub fn insert(&mut self, coord: Coord, v: V) -> Option<V> {
        self.map.insert(coord, v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Coord, &V)> {
        self.map.iter()
    }

    pub fn swap(&mut self, a: Coord, b: Coord) {
        assert_ne!(a, b);
        let ap = self.map.get_mut(&a).unwrap() as *mut _;
        let bp = self.map.get_mut(&b).unwrap() as *mut _;
        // SAFETY: should be fine
        unsafe {
            std::ptr::swap(ap, bp);
        };
    }
}

impl<V> FromIterator<(Coord, V)> for Grid<V> {
    fn from_iter<T: IntoIterator<Item = (Coord, V)>>(iter: T) -> Self {
        Grid {
            map: HashMap::from_iter(iter),
        }
    }
}
