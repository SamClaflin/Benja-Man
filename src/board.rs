use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WallType {
    CornerBottomLeft,
    CornerBottomLeftSolid,
    CornerBottomRight,
    CornerBottomRightSolid,
    CornerTopLeft,
    CornerTopLeftSolid,
    CornerTopRight,
    CornerTopRightSolid,
    WallBottomHollow,
    WallBottomSolid,
    WallLeftHollow,
    WallLeftSolid,
    WallRightHollow,
    WallRightSolid,
    WallTopHollow,
    WallTopSolid,
    CornerBottomLeftEnclosed,
    CornerBottomRightEnclosed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardTile {
    Empty,
    Wall(WallType),
    Dot,
    Fruit
}

pub struct Board {
    matrix: Vec<Vec<BoardTile>>,
    cell_size: f32,
    offset: f32,
}

impl Board {
    pub fn new(cell_size: f32, offset: f32) -> Self {
        let key_map = HashMap::from([
            ('_', BoardTile::Empty),
            ('.', BoardTile::Dot),
            ('f', BoardTile::Fruit),
            ('<', BoardTile::Wall(WallType::CornerBottomLeft)),
            ('>', BoardTile::Wall(WallType::CornerBottomRight)),
            ('^', BoardTile::Wall(WallType::CornerTopLeft)),
            ('v', BoardTile::Wall(WallType::CornerTopRight)),
            ('a', BoardTile::Wall(WallType::WallBottomHollow)),
            ('b', BoardTile::Wall(WallType::WallBottomSolid)),
            ('c', BoardTile::Wall(WallType::WallLeftHollow)),
            ('d', BoardTile::Wall(WallType::WallLeftSolid)),
            ('e', BoardTile::Wall(WallType::WallRightHollow)),
            ('f', BoardTile::Wall(WallType::WallRightSolid)),
            ('g', BoardTile::Wall(WallType::WallTopHollow)),
            ('h', BoardTile::Wall(WallType::WallTopSolid)),
            ('i', BoardTile::Wall(WallType::CornerBottomLeftSolid)),
            ('j', BoardTile::Wall(WallType::CornerBottomRightSolid)),
            ('k', BoardTile::Wall(WallType::CornerTopLeftSolid)),
            ('l', BoardTile::Wall(WallType::CornerTopRightSolid)),
            ('m', BoardTile::Wall(WallType::CornerBottomLeftEnclosed)),
            ('n', BoardTile::Wall(WallType::CornerBottomRightEnclosed)),
        ]);

        let mock_matrix = [
            ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
            ['j', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'm', 'n', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'h', 'i'],
            ['d', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'f'],
            ['d', '.', '>', 'a', 'a', '<', '.', '>', 'a', 'a', 'a', '<', '.', 'e', 'c', '.', '>', 'a', 'a', 'a', '<', '.', '>', 'a', 'a', '<', '.', 'f'],
            ['d', '.', 'e', '_', '_', 'c', '.', 'e', '_', '_', '_', 'c', '.', 'e', 'c', '.', 'e', '_', '_', '_', 'c', '.', 'e', '_', '_', 'c', '.', 'f'],
            ['d', '.', 'v', 'g', 'g', '^', '.', 'v', 'g', 'g', 'g', '^', '.', 'v', '^', '.', 'v', 'g', 'g', 'g', '^', '.', 'v', 'g', 'g', '^', '.', 'f'],
            ['d', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'f'],
            ['d', '.', '>', 'a', 'a', '<', '.', '>', '<', '.', '>', 'a', 'a', 'a', 'a', 'a', 'a', '<', '.', '>', '<', '.', '>', 'a', 'a', '<', '.', 'f'],
            ['d', '.', 'v', 'g', 'g', '^', '.', 'e', 'c', '.', 'v', 'g', 'g', '<', '>', 'g', 'g', '^', '.', 'e', 'c', '.', 'v', 'g', 'g', '^', '.', 'f'],
            ['d', '.', '.', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', '.', '.', 'f'],
            ['l', 'b', 'b', 'b', 'b', '<', '.', 'e', 'v', 'a', 'a', '<', '.', 'e', 'c', '.', '>', 'a', 'a', '^', 'c', '.', '>', 'b', 'b', 'b', 'b', 'k'],
            ['_', '_', '_', '_', '_', 'd', '.', 'e', '>', 'g', 'g', '^', '.', 'v', '^', '.', 'v', 'g', 'g', '<', 'c', '.', 'f', '_', '_', '_', '_', '_'],
            ['_', '_', '_', '_', '_', 'd', '.', 'e', 'c', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'e', 'c', '.', 'f', '_', '_', '_', '_', '_'],
            ['_', '_', '_', '_', '_', 'd', '.', 'e', 'c', '.', '>', 'b', 'b', '_', '_', 'b', 'b', '<', '.', 'e', 'c', '.', 'f', '_', '_', '_', '_', '_'],
            ['h', 'h', 'h', 'h', 'h', '^', '.', 'v', '^', '.', 'f', '_', '_', '_', '_', '_', '_', 'd', '.', 'v', '^', '.', 'v', 'h', 'h', 'h', 'h', 'h'],
            ['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'f', '_', '_', '_', '_', '_', '_', 'd', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            ['b', 'b', 'b', 'b', 'b', '<', '.', '>', '<', '.', 'f', '_', '_', '_', '_', '_', '_', 'd', '.', '>', '<', '.', '>', 'b', 'b', 'b', 'b', 'b'],
            ['_', '_', '_', '_', '_', 'd', '.', 'e', 'c', '.', 'v', 'h', 'h', 'h', 'h', 'h', 'h', '^', '.', 'e', 'c', '.', 'f', '_', '_', '_', '_', '_'],
            ['_', '_', '_', '_', '_', 'd', '.', 'e', 'c', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'e', 'c', '.', 'f', '_', '_', '_', '_', '_'],
            ['_', '_', '_', '_', '_', 'd', '.', 'e', 'c', '.', '>', 'a', 'a', 'a', 'a', 'a', 'a', '<', '.', 'e', 'c', '.', 'f', '_', '_', '_', '_', '_'],
            ['j', 'h', 'h', 'h', 'h', '^', '.', 'v', '^', '.', 'v', 'g', 'g', '<', '>', 'g', 'g', '^', '.', 'v', '^', '.', 'v', 'h', 'h', 'h', 'h', 'i'],
            ['d', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'f'],
            ['d', '.', '>', 'a', 'a', '<', '.', '>', 'a', 'a', 'a', '<', '.', 'e', 'c', '.', '>', 'a', 'a', 'a', '<', '.', '>', 'a', 'a', '<', '.', 'f'],
            ['d', '.', 'v', 'g', '<', 'c', '.', 'v', 'g', 'g', 'g', '^', '.', 'v', '^', '.', 'v', 'g', 'g', 'g', '^', '.', 'e', '>', 'g', '^', '.', 'f'],
            ['d', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', 'f'],
            ['v', 'a', '<', '.', 'e', 'c', '.', '>', '<', '.', '>', 'a', 'a', 'a', 'a', 'a', 'a', '<', '.', '>', '<', '.', 'e', 'c', '.', '>', 'a', '^'],
            ['>', 'g', '^', '.', 'v', '^', '.', 'e', 'c', '.', 'v', 'g', 'g', '<', '>', 'g', 'g', '^', '.', 'e', 'c', '.', 'v', '^', '.', 'v', 'g', '<'],
            ['d', '.', '.', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', 'e', 'c', '.', '.', '.', '.', '.', '.', 'f'],
            ['d', '.', '>', 'a', 'a', 'a', 'a', '^', 'v', 'a', 'a', '<', '.', 'e', 'c', '.', '>', 'a', 'a', '^', 'v', 'a', 'a', 'a', 'a', '<', '.', 'f'],
            ['d', '.', 'v', 'g', 'g', 'g', 'g', 'g', 'g', 'g', 'g', '^', '.', 'v', '^', '.', 'v', 'g', 'g', 'g', 'g', 'g', 'g', 'g', 'g', '^', '.', 'f'],
            ['d', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'f'],
            ['l', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'k'],
            ['_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        ];

        let mut matrix: Vec<Vec<BoardTile>> = Vec::new();
        for row in mock_matrix.iter() {
            let mut curr_row = Vec::new();
            for val in row.iter() {
                curr_row.push(*key_map.get(val).unwrap());
            }
            matrix.push(curr_row);
        }

        Self {
            matrix,
            cell_size,
            offset
        }
    }

    pub fn get_at(&self, i: usize, j: usize) -> Option<BoardTile> {
        if self.indeces_valid(i, j) {
            Some(self.matrix[i][j])
        } else {
            None
        }
    }

    pub fn set_at(&mut self, i: usize, j: usize, val: BoardTile) {
        self.validate_indeces(i, j);
        self.matrix[i][j] = val;
    }

    pub fn width(&self) -> usize {
        self.matrix[0].len()
    }

    pub fn height(&self) -> usize {
        self.matrix.len()
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    pub fn offset(&self) -> f32 {
        self.offset
    }

    pub fn indeces_to_coordinates(&self, i: usize, j: usize) -> (f32, f32) {
        self.validate_indeces(i, j);
        (j as f32, (self.height() - i - 1) as f32)
    }

    fn validate_indeces(&self, i: usize, j: usize) {
        if !self.indeces_valid(i, j) {
            panic!("Received invalid indeces: ({}, {})", i, j);
        }
    }

    fn indeces_valid(&self, i: usize, j: usize) -> bool {
        i < self.matrix.len() && j < self.matrix[0].len()
    }
}
