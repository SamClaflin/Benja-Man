use bevy::prelude::*;
use crate::board;
use board::{Board, BoardTile};

pub fn is_centered_horizontally(transform: &Transform, board: &Board) -> bool {
    (transform.translation.x - board.offset()) % board.cell_size() == 0.
}

pub fn is_centered_vertically(transform: &Transform, board: &Board) -> bool {
    (transform.translation.y - board.offset()) % board.cell_size() == 0.
}

pub fn can_move_up(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_y = transform.translation.y + speed;
    let (i, j) = board.coordinates_to_indeces(transform.translation.x, new_y);
    let new_tile = board.try_get(i, j);
    is_centered_horizontally(&transform, &board) 
        && new_tile.is_some() 
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}

pub fn can_move_right(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_x = transform.translation.x + speed;
    let (i, j) = board.coordinates_to_indeces(new_x - speed + board.cell_size(), transform.translation.y);
    let new_tile = board.try_get(i, j);
    is_centered_vertically(&transform, &board) 
        && new_tile.is_some() 
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}

pub fn can_move_down(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_y = transform.translation.y - speed;
    let (i, j) = board.coordinates_to_indeces(transform.translation.x, new_y + speed - board.cell_size());
    let new_tile = board.try_get(i, j);
    is_centered_horizontally(&transform, &board) 
        && new_tile.is_some() 
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}

pub fn can_move_left(transform: &Transform, board: &Board, speed: f32) -> bool {
    let new_x = transform.translation.x - speed;
    let (i, j) = board.coordinates_to_indeces(new_x, transform.translation.y);
    let new_tile = board.try_get(i, j);
    is_centered_vertically(&transform, &board) 
        && new_tile.is_some() 
        && new_tile.unwrap() != BoardTile::Wall
        && new_tile.unwrap() != BoardTile::GhostGate
}
