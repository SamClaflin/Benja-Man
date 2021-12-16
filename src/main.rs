mod board;
mod ben;
mod enums;

use std::collections::HashMap;
use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, WindowOrigin},
};
use board::{Board, BoardTile, WallType};
use ben::{Ben, BenBundle, BenAnimationTimer, BenSpeed, BenDirection};
use enums::Direction;

struct WallMaterials {
    material_dict: HashMap<WallType, Handle<ColorMaterial>> 
}

struct BenMaterials {
    ben_default: Handle<ColorMaterial>,
    ben_up: Handle<ColorMaterial>,
    ben_right: Handle<ColorMaterial>,
    ben_down: Handle<ColorMaterial>,
    ben_left: Handle<ColorMaterial>,
}

fn main() {
    let board = Board::new(32., 16.);

    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "Benja-Man".to_string(),
            width: board.width() as f32 * board.cell_size(),
            height: board.height() as f32 * board.cell_size(),
            ..Default::default()
        })
        .insert_resource(board)
        .add_startup_system(setup.system())
        .add_system(ben_animation_system.system())
        .add_system(ben_movement_system.system())
        .add_system(ben_controller_system.system())
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    board: Res<Board>
) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle {
        orthographic_projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..OrthographicCameraBundle::new_2d()
    });

    // Walls, dots, and fruit
    let dot_material_handle = materials.add(asset_server.load("../assets/cookie.png").into());
    let wall_materials = WallMaterials {
        material_dict: HashMap::from([
            (WallType::CornerBottomLeft, materials.add(asset_server.load("../assets/walls/corner_bottom_left.png").into())),
            (WallType::CornerBottomLeftEnclosed, materials.add(asset_server.load("../assets/walls/corner_bottom_left_enclosed.png").into())),
            (WallType::CornerBottomLeftSolid, materials.add(asset_server.load("../assets/walls/corner_bottom_left_solid.png").into())),
            (WallType::CornerBottomRight, materials.add(asset_server.load("../assets/walls/corner_bottom_right.png").into())),
            (WallType::CornerBottomRightEnclosed, materials.add(asset_server.load("../assets/walls/corner_bottom_right_enclosed.png").into())),
            (WallType::CornerBottomRightSolid, materials.add(asset_server.load("../assets/walls/corner_bottom_right_solid.png").into())),
            (WallType::CornerTopLeft, materials.add(asset_server.load("../assets/walls/corner_top_left.png").into())),
            (WallType::CornerTopLeftSolid, materials.add(asset_server.load("../assets/walls/corner_top_left_solid.png").into())),
            (WallType::CornerTopRight, materials.add(asset_server.load("../assets/walls/corner_top_right.png").into())),
            (WallType::CornerTopRightSolid, materials.add(asset_server.load("../assets/walls/corner_top_right_solid.png").into())),
            (WallType::WallBottomHollow, materials.add(asset_server.load("../assets/walls/wall_bottom_hollow.png").into())),
            (WallType::WallBottomSolid, materials.add(asset_server.load("../assets/walls/wall_bottom_solid.png").into())),
            (WallType::WallLeftHollow, materials.add(asset_server.load("../assets/walls/wall_left_hollow.png").into())),
            (WallType::WallLeftSolid, materials.add(asset_server.load("../assets/walls/wall_left_solid.png").into())),
            (WallType::WallRightHollow, materials.add(asset_server.load("../assets/walls/wall_right_hollow.png").into())),
            (WallType::WallRightSolid, materials.add(asset_server.load("../assets/walls/wall_right_solid.png").into())),
            (WallType::WallTopHollow, materials.add(asset_server.load("../assets/walls/wall_top_hollow.png").into())),
            (WallType::WallTopSolid, materials.add(asset_server.load("../assets/walls/wall_top_solid.png").into())),
        ])
    };
    for i in 0..board.height() {
        for j in 0..board.width() {
            let curr_tile = board.try_get(i, j).unwrap();
            let (x, y) = board.indeces_to_coordinates(i, j);
            let material: Handle<ColorMaterial>;
            let scale_factor: f32;
            match curr_tile {
                BoardTile::Empty => continue,
                BoardTile::Wall(wall_type) => {
                    material = wall_materials.material_dict.get(&wall_type).unwrap().clone();
                    scale_factor = 1.;
                },
                BoardTile::Dot => {
                    material = dot_material_handle.clone();
                    scale_factor = 1.75; 
                },
                BoardTile::Fruit => {
                    material = dot_material_handle.clone();
                    scale_factor = 8.; 
                }
            }

            let scale = Vec3::new(1./scale_factor, 1./scale_factor, 1.);
            commands.spawn_bundle(SpriteBundle {
                material,
                sprite: Sprite::new(Vec2::new(board.cell_size() as f32, board.cell_size() as f32)),
                transform: Transform {
                    translation: Vec3::new(x,y,1.),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
    commands.insert_resource(wall_materials);

    // Ben
    let (_, ben_y_init) = board.indeces_to_coordinates(24, 0);
    let ben_materials = BenMaterials {
        ben_default: materials.add(asset_server.load("../assets/ben/ben.png").into()),
        ben_up: materials.add(asset_server.load("../assets/ben/ben_up.png").into()),
        ben_right: materials.add(asset_server.load("../assets/ben/ben_right.png").into()),
        ben_down: materials.add(asset_server.load("../assets/ben/ben_down.png").into()),
        ben_left: materials.add(asset_server.load("../assets/ben/ben_left.png").into()),
    };
    commands.spawn_bundle(BenBundle {
        sprite_bundle: SpriteBundle {
            material: ben_materials.ben_default.clone(),
            transform: Transform {
                translation: Vec3::new(board.width() as f32 * board.cell_size() / 2., ben_y_init, 10.),
                scale: Vec3::new(1./6., 1./6., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(ben_materials);
}

fn ben_animation_system(
    ben_materials: Res<BenMaterials>,
    time: Res<Time>,
    mut query: Query<(&mut Handle<ColorMaterial>, &mut BenAnimationTimer, &BenDirection), With<Ben>>
) {
    let (mut material_handle, mut ben_animation_timer, ben_direction) = query.single_mut().unwrap();
    let timer = &mut ben_animation_timer.0;
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    }

    material_handle.id = if material_handle.id != ben_materials.ben_default.id {
        ben_materials.ben_default.id
    } else {
        let direction = ben_direction.0;
        match direction {
            Direction::Up => ben_materials.ben_up.id,
            Direction::Right => ben_materials.ben_right.id,
            Direction::Down => ben_materials.ben_down.id,
            Direction::Left => ben_materials.ben_left.id,
        }
    }
}

fn ben_movement_system(
    board: Res<Board>,
    mut query: Query<(&mut Transform, &BenDirection, &BenSpeed), With<Ben>>
) {
    let (mut transform, ben_direction, ben_speed) = query.single_mut().unwrap();
    let speed = ben_speed.0;
    let direction = ben_direction.0;
    let x = transform.translation.x + board.offset();
    let y = transform.translation.y - board.offset();
    let (i, j) = board.coordinates_to_indeces(x, y);

    let top_tile = if i == 0 { None } else { board.try_get(i - 1, j) };
    let right_tile = board.try_get(i, j + 1);
    let bottom_tile = board.try_get(i + 1, j);
    let left_tile = if j == 0 { None } else { board.try_get(i, j - 1) };

    let is_centered_horizontally = x % board.cell_size() == 0.;
    let is_centered_vertically = y % board.cell_size() == 0.;
    let can_move_up= !is_centered_vertically || top_tile.is_some() && match top_tile.unwrap() { BoardTile::Wall(_) => false, _ => true };
    let can_move_right = !is_centered_horizontally || right_tile.is_some() && match right_tile.unwrap() { BoardTile::Wall(_) => false, _ => true };
    let can_move_down = !is_centered_vertically || bottom_tile.is_some() && match bottom_tile.unwrap() { BoardTile::Wall(_) => false, _ => true };
    let can_move_left = !is_centered_horizontally || left_tile.is_some() && match left_tile.unwrap() { BoardTile::Wall(_) => false, _ => true };

    match direction {
        Direction::Up => if can_move_up { transform.translation.y += speed }, 
        Direction::Right => if can_move_right { transform.translation.x += speed }, 
        Direction::Down => if can_move_down { transform.translation.y -= speed }, 
        Direction::Left => if can_move_left { transform.translation.x -= speed }, 
    }
}

fn ben_controller_system(
    mut query: Query<&mut BenDirection, With<Ben>>,
    keys: Res<Input<KeyCode>>
) {
    let mut ben_direction = query.single_mut().unwrap();
    if keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) {
        ben_direction.0 = Direction::Up;
    } else if keys.just_pressed(KeyCode::D) || keys.just_pressed(KeyCode::Right) {
        ben_direction.0 = Direction::Right;
    } else if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down) {
        ben_direction.0 = Direction::Down;
    } else if keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Left) {
        ben_direction.0 = Direction::Left;
    } 
}
