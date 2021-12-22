mod board;
mod ben;
mod enums;
mod dot;
mod ghost;
mod utils;

use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, WindowOrigin}
};
use board::{Board, BoardTile};
use ben::{Ben, BenBundle, BenAnimationTimer, BenSpeed, BenDirection, BenNextDirection};
use enums::Direction;
use dot::{Dot, DotBundle, DotCoordinates};

struct BenMaterials {
    ben_default: Handle<ColorMaterial>,
    ben_up: Handle<ColorMaterial>,
    ben_right: Handle<ColorMaterial>,
    ben_down: Handle<ColorMaterial>,
    ben_left: Handle<ColorMaterial>,
}

#[derive(SystemLabel, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Label {
    BenControllerSystem,
    BenMovementSystem
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
        .add_system_set(
            SystemSet::new()
                .with_system(ben_controller_system.system().label(Label::BenControllerSystem))
                .with_system(ben_movement_system.system().label(Label::BenMovementSystem).after(Label::BenControllerSystem))
                .with_system(ben_collision_system.system().after(Label::BenMovementSystem))
                .with_system(ben_animation_system.system().after(Label::BenMovementSystem))
        )
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

    // Board
    let board_material_handle = materials.add(asset_server.load("../assets/board.png").into());
    commands.spawn_bundle(SpriteBundle {
        material: board_material_handle.clone(),
        transform: Transform {
            translation: Vec3::new(board.width() as f32 * board.cell_size() / 2., board.height() as f32 * board.cell_size() / 2., 1.),
            ..Default::default()
        },
        ..Default::default()
    });

    // Dots
    let dot_material_handle = materials.add(asset_server.load("../assets/cookie.png").into());
    for i in 0..board.height() {
        for j in 0..board.width() {
            let (x, y) = board.indeces_to_coordinates(i, j);
            match board.try_get(i, j).unwrap() {
                BoardTile::Dot => {
                    commands.spawn_bundle(DotBundle {
                        coordinates: DotCoordinates(x, y),
                        sprite_bundle: SpriteBundle {
                            material: dot_material_handle.clone(),
                            sprite: Sprite::new(Vec2::new(board.cell_size() as f32, board.cell_size() as f32)),
                            transform: Transform {
                                translation: Vec3::new(x, y, 2.),
                                scale: Vec3::new(1./1.75, 1./1.75, 1.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                },
                _ => continue
            }
        }
    }

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

fn ben_controller_system(
    mut query: Query<&mut BenNextDirection, With<Ben>>,
    keys: Res<Input<KeyCode>>
) {
    let mut ben_next_direction = query.single_mut().unwrap();
    if keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) {
        ben_next_direction.0 = Some(Direction::Up);
    } else if keys.just_pressed(KeyCode::D) || keys.just_pressed(KeyCode::Right) {
        ben_next_direction.0 = Some(Direction::Right);
    } else if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down) {
        ben_next_direction.0 = Some(Direction::Down);
    } else if keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Left) {
        ben_next_direction.0 = Some(Direction::Left);
    }
}

fn ben_movement_system(
    mut query: Query<(&mut Transform, &mut BenNextDirection , &mut BenDirection, &BenSpeed), With<Ben>>,
    board: Res<Board>
) {
    let (mut transform, mut ben_next_direction, mut ben_direction, ben_speed) = query.single_mut().unwrap();
    let speed = ben_speed.0;
    let can_move_up = utils::can_move_up(&transform, &board, speed);
    let can_move_right = utils::can_move_right(&transform, &board, speed);
    let can_move_down = utils::can_move_down(&transform, &board, speed);
    let can_move_left = utils::can_move_left(&transform, &board, speed);

    // Determine if the direction needs to be changed
    let next_direction = ben_next_direction.0;
    if next_direction.is_some() {
        match next_direction.unwrap() {
            Direction::Up => {
                if can_move_up {
                    ben_direction.0 = Direction::Up;
                    ben_next_direction.0 = None;
                }
            },
            Direction::Right => {
                if can_move_right {
                    ben_direction.0 = Direction::Right;
                    ben_next_direction.0 = None;
                }
            },
            Direction::Down => {
                if can_move_down {
                    ben_direction.0 = Direction::Down;
                    ben_next_direction.0 = None;
                }
            },
            Direction::Left => {
                if can_move_left {
                    ben_direction.0 = Direction::Left;
                    ben_next_direction.0 = None;
                }
            },
        }
    }

    // Perform the movement
    let direction = ben_direction.0;
    match direction {
        Direction::Up => {
            if can_move_up {
                transform.translation.y += speed;
            }
        },
        Direction::Right => {
            if can_move_right {
                transform.translation.x += speed;
            }
        },
        Direction::Down => {
            if can_move_down {
                transform.translation.y -= speed;
            }
        },
        Direction::Left => {
            if can_move_left {
                transform.translation.x -= speed;
            }
        }
    }
}

fn ben_animation_system() {
}

fn ben_collision_system(
    mut commands: Commands,
    query_set: QuerySet<(
        Query<&Transform, With<Ben>>,
        Query<(Entity, &Transform), With<Dot>>
    )>,
    board: Res<Board>
) {
    let ben_transform = query_set.q0().single().unwrap();

    if utils::is_centered_horizontally(ben_transform, &board) && utils::is_centered_vertically(ben_transform, &board) {
        // Consume dot
        for (dot_entity, dot_transform) in query_set.q1().iter() {
            if dot_transform.translation.x == ben_transform.translation.x && dot_transform.translation.y == ben_transform.translation.y {
                commands.entity(dot_entity).despawn();
                break;
            }
        }
    }
}
