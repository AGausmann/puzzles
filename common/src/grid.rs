use std::convert::identity;

use glam::{ivec2, IVec2};

use crate::math::IRect;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn from_rows<RowIter, CellIter>(rows: RowIter) -> Self
    where
        RowIter: Iterator<Item = CellIter>,
        CellIter: Iterator<Item = T>,
    {
        let mut width = None;
        let mut height = 0;
        let cells = rows
            .into_iter()
            .flat_map(|row| {
                height += 1;
                let row: Vec<T> = row.into_iter().collect();
                let row_width = row.len();
                assert_eq!(*width.get_or_insert(row_width), row_width);
                row
            })
            .collect();

        Self {
            cells,
            width: width.unwrap_or(0),
            height,
        }
    }

    pub fn map_chars<F>(s: &str, mapper: F) -> Self
    where
        F: Copy + Fn(char) -> T,
    {
        Self::from_rows(s.lines().map(|line| line.chars().map(mapper)))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn shape(&self) -> IRect {
        IRect {
            lower: IVec2::ZERO,
            upper: ivec2(
                self.width.saturating_sub(1).try_into().unwrap(),
                self.height.saturating_sub(1).try_into().unwrap(),
            ),
        }
    }

    pub fn contains(&self, location: IVec2) -> bool {
        self.shape().contains(location)
    }

    fn cell_coordinate(&self, location: IVec2) -> Option<usize> {
        self.contains(location)
            .then(|| location.x as usize + location.y as usize * self.width)
    }

    pub fn get(&self, location: IVec2) -> Option<&T> {
        self.cell_coordinate(location).map(|i| &self.cells[i])
    }

    pub fn get_mut(&mut self, location: IVec2) -> Option<&mut T> {
        self.cell_coordinate(location).map(|i| &mut self.cells[i])
    }

    pub fn find(&self, value: &T) -> Option<IVec2>
    where
        T: PartialEq,
    {
        (0..self.width()).find_map(|x| {
            (0..self.height()).find_map(move |y| {
                let coord = ivec2(x as _, y as _);
                (self.get(coord).unwrap() == value).then_some(coord)
            })
        })
    }

    /// Generate coordinates of the Von Neumann neighborhood of the cell at the
    /// given location.
    ///
    /// If the cell is on the boundary of the grid, the out-of-bounds
    /// coordinates will not be generated.
    pub fn neighbors_4(&self, location: IVec2) -> impl Iterator<Item = IVec2> {
        let offsets = [IVec2::X, IVec2::Y, -IVec2::X, -IVec2::Y];
        let shape = self.shape();
        offsets
            .into_iter()
            .flat_map(move |k| shape.contains(location + k).then_some(location + k))
    }

    /// Generate coordinates of the Moore neighborhood of the cell at the given
    /// location.
    ///
    /// If the cell is on the boundary of the grid, the out-of-bounds
    /// coordinates will not be generated.
    pub fn neighbors_8(&self, location: IVec2) -> impl Iterator<Item = IVec2> {
        let offsets = [
            IVec2::X,
            IVec2::X + IVec2::Y,
            IVec2::Y,
            -IVec2::X + IVec2::Y,
            -IVec2::X,
            -IVec2::X - IVec2::Y,
            -IVec2::Y,
            IVec2::X - IVec2::Y,
        ];
        let shape = self.shape();
        offsets
            .into_iter()
            .flat_map(move |k| shape.contains(location + k).then_some(location + k))
    }
}

impl Grid<char> {
    pub fn from_chars(s: &str) -> Self {
        Self::map_chars(s, identity)
    }
}
