use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const CROSS_MATRIX: [(isize, isize); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    pub const DIAGONAL_MATRIX: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn new_relative(&self, x: isize, y: isize) -> Option<Pos> {
        let new_x = self.x.checked_add_signed(x)?;
        let new_y = self.y.checked_add_signed(y)?;
        Some(Self::new(new_x, new_y))
    }

    pub fn manhattan_distance(&self, other: &Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_rows<R, I>(rows: R) -> Self
    where
        R: IntoIterator<Item = I>,
        I: IntoIterator<Item = T>,
    {
        let mut data = Vec::new();
        let mut width = None;
        let mut height = 0;
        for row in rows {
            let start = data.len();
            data.extend(row);
            let w = data.len() - start;
            match width {
                None => width = Some(w),
                Some(expected) => assert_eq!(w, expected, "ragged row {height}"),
            }
            height += 1;
        }
        Self {
            width: width.unwrap_or(0),
            height,
            data,
        }
    }

    pub const fn width(&self) -> usize {
        self.width
    }
    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn center(&self) -> Pos {
        Pos::new(self.width() / 2, self.height() / 2)
    }

    fn idx(&self, p: Pos) -> Option<usize> {
        (p.x < self.width && p.y < self.height).then(|| p.y * self.width + p.x)
    }

    pub fn get(&self, p: Pos) -> Option<&T> {
        self.idx(p).map(|i| &self.data[i])
    }

    pub fn get_mut(&mut self, p: Pos) -> Option<&mut T> {
        self.idx(p).map(|i| &mut self.data[i])
    }

    /// Wraps horizontally only (what the scale-armour puzzle needs).
    pub fn get_wrap_x(&self, x: isize, y: usize) -> Option<&T> {
        let x = x.rem_euclid(self.width as isize) as usize;
        self.get(Pos::new(x, y))
    }

    pub fn find(&self, pred: impl Fn(Pos, &T) -> bool) -> Option<Pos> {
        self.iter_pos()
            .find_map(|(pos, cell)| pred(pos, cell).then_some(pos))
    }

    pub fn row(&self, y: usize) -> &[T] {
        &self.data[y * self.width..(y + 1) * self.width]
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.width)
    }

    /// All cells, row-major.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// All cells with their positions, row-major.
    pub fn iter_pos(&self) -> impl Iterator<Item = (Pos, &T)> {
        let w = self.width;
        self.data
            .iter()
            .enumerate()
            .map(move |(i, v)| (Pos::new(i % w, i / w), v))
    }

    pub fn iter_pos_mut(&mut self) -> impl Iterator<Item = (Pos, &mut T)> {
        let w = self.width;
        self.data
            .iter_mut()
            .enumerate()
            .map(move |(i, v)| (Pos::new(i % w, i / w), v))
    }

    /// Flat row-major data with known dimensions.
    pub fn from_vec(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(
            data.len(),
            width * height,
            "data length {} != {width}x{height}",
            data.len()
        );
        Self {
            width,
            height,
            data,
        }
    }

    /// Build each cell from its position.
    pub fn from_fn(width: usize, height: usize, mut f: impl FnMut(Pos) -> T) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                data.push(f(Pos::new(x, y)));
            }
        }
        Self {
            width,
            height,
            data,
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Returns orthogonal neighbors within bounds (Up, Down, Right, Left).
    pub fn neighbors4(&self, p: Pos) -> impl Iterator<Item = (Pos, &T)> {
        self.filter_neighbours(Pos::CROSS_MATRIX.into_iter(), p)
    }

    /// Returns all 8 neighbors within bounds (orthogonal and diagonal).
    pub fn neighbors8(&self, p: Pos) -> impl Iterator<Item = (Pos, &T)> {
        self.filter_neighbours(Pos::CROSS_MATRIX.into_iter().chain(Pos::DIAGONAL_MATRIX), p)
    }

    fn filter_neighbours(
        &self,
        iter: impl Iterator<Item = (isize, isize)>,
        p: Pos,
    ) -> impl Iterator<Item = (Pos, &T)> {
        iter.filter_map(move |(dx, dy)| {
            let neighbor_pos = p.new_relative(dx, dy)?;
            let cell = self.get(neighbor_pos)?;
            Some((neighbor_pos, cell))
        })
    }
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        Self {
            width,
            height,
            data: vec![fill; width * height],
        }
    }

    /// Out-of-bounds cells become `None`, so "outside" stays distinct from real values.
    pub fn subgrid(&self, center: Pos, w: usize, h: usize) -> Grid<Option<T>> {
        let (cx, cy) = (center.x as isize, center.y as isize);
        let (hx, hy) = ((w / 2) as isize, (h / 2) as isize);
        Grid::from_rows((0..h as isize).map(|dy| {
            (0..w as isize).map(move |dx| {
                let (x, y) = (cx + dx - hx, cy + dy - hy);
                if x < 0 || y < 0 {
                    None
                } else {
                    self.get(Pos::new(x as usize, y as usize)).cloned()
                }
            })
        }))
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;
    fn index(&self, p: Pos) -> &T {
        self.get(p)
            .unwrap_or_else(|| panic!("{p:?} out of bounds ({}x{})", self.width, self.height))
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    fn index_mut(&mut self, p: Pos) -> &mut T {
        let (w, h) = (self.width, self.height);
        self.get_mut(p)
            .unwrap_or_else(|| panic!("{p:?} out of bounds ({w}x{h})"))
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GRID({}x{})", self.width, self.height)?;
        for row in self.rows() {
            writeln!(f, "{row:?}")?;
        }
        Ok(())
    }
}

impl Grid<u8> {
    /// Formats the byte grid as an ASCII string with newlines.
    pub fn display_ascii(&self) -> String {
        let mut s = String::with_capacity((self.width + 1) * self.height);
        for row in self.rows() {
            for &b in row {
                s.push(b as char);
            }
            s.push('\n');
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_manhattan_distance() {
        let p1 = Pos::new(2, 3);
        let p2 = Pos::new(5, 7);
        assert_eq!(p1.manhattan_distance(&p2), 3 + 4);
        assert_eq!(p2.manhattan_distance(&p1), 7);
    }

    #[test]
    fn test_grid_neighbors() {
        let grid = Grid::new(3, 3, 0);
        let center = Pos::new(1, 1);
        assert_eq!(grid.neighbors4(center).count(), 4);
        assert_eq!(grid.neighbors8(center).count(), 8);

        let corner = Pos::new(0, 0);
        assert_eq!(grid.neighbors4(corner).count(), 2);
        assert_eq!(grid.neighbors8(corner).count(), 3);
    }

    #[test]
    fn test_grid_display_ascii() {
        let rows = vec![b"###".to_vec(), b"#.#".to_vec(), b"###".to_vec()];
        let grid = Grid::from_rows(rows);
        assert_eq!(grid.display_ascii(), "###\n#.#\n###\n");
    }
}
