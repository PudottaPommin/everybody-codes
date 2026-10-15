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
