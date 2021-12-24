mod board;
mod ben;
mod enums;
mod dot;
mod ghost;
mod utils;
mod score;
mod events;
mod power_up;
mod path;

use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, WindowOrigin}
};
use ghost::{
    Ghost, 
    GhostPath, 
    GhostState, 
    GhostStateComponent,
    GhostBundle,
    Caleb,
    CalebBundle, 
    CalebMaterials,
    CalebPathChangeTimer, 
    Harris, 
    HarrisMaterials,
    HarrisBundle, 
    Claflin, 
    ClaflinMaterials,
    ClaflinBundle, 
    Samson, 
    SamsonMaterials,
    SamsonBundle,
    GhostScareTimer
};
use board::{Board, BoardTile};
use ben::{Ben, BenBundle, BenAnimationTimer, BenSpeed, BenDirection, BenNextDirection, BenMaterials};
use enums::Direction;
use dot::{Dot, DotBundle};
use score::{Score, ScoreBundle, PointValues};
use events::{BenDirectionChangedEvent, PowerUpConsumedEvent};
use power_up::{PowerUp, PowerUpBundle, PowerUpMaterials, PowerUpAnimationTimer};
use path::Path;

fn main() {
    let board = Board::new(32., 16.);

    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: "Benja-Man".to_string(),
            width: board.width() as f32 * board.cell_size(),
            height: board.height() as f32 * board.cell_size() + 128.,
            ..Default::default()
        })
        .insert_resource(board)
        .init_resource::<PointValues>()
        .init_resource::<GhostScareTimer>()
        .add_event::<BenDirectionChangedEvent>()
        .add_event::<PowerUpConsumedEvent>()
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_system(ben_controller_system.system())
                .with_system(ben_movement_system.system())
                .with_system(ben_dot_collision_system.system())
                .with_system(ben_power_up_collision_system.system())
                .with_system(ben_ghost_collision_system.system())
                .with_system(ben_animation_system.system())
                .with_system(scare_ghosts_system.system())
                .with_system(caleb_movement_system.system())
                .with_system(harris_movement_system.system())
                .with_system(claflin_movement_system.system())
                .with_system(samson_movement_system.system())
                .with_system(caleb_animation_system.system())
                .with_system(harris_animation_system.system())
                .with_system(claflin_animation_system.system())
                .with_system(samson_animation_system.system())
                .with_system(power_up_animation_system.system())
                .with_system(score_system.system())
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
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

    // Dots and power-ups
    let dot_material_handle = materials.add(asset_server.load("../assets/cookie.png").into());
    let power_up_materials = PowerUpMaterials {
        material_1: materials.add(asset_server.load("../assets/arizona_1.png").into()),
        material_2: materials.add(asset_server.load("../assets/arizona_2.png").into()),
    };
    for i in 0..board.height() {
        for j in 0..board.width() {
            let (x, y) = board.indeces_to_coordinates(i, j);
            match board.try_get(i, j).unwrap() {
                BoardTile::Dot => {
                    commands.spawn_bundle(DotBundle {
                        sprite_bundle: SpriteBundle {
                            material: dot_material_handle.clone(),
                            transform: Transform {
                                translation: Vec3::new(x, y, 2.),
                                scale: Vec3::new(1./12., 1./12., 1.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                },
                BoardTile::PowerUp => {
                    commands.spawn_bundle(PowerUpBundle {
                        sprite_bundle: SpriteBundle {
                            material: power_up_materials.material_1.clone(),
                            transform: Transform {
                                translation: Vec3::new(x, y, 2.),
                                scale: Vec3::new(1./24., 1./24., 1.),
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
    commands.insert_resource(power_up_materials);

    // Ben
    let ben_init_x = board.width() as f32 * board.cell_size() / 2.;
    let (_, ben_init_y) = board.indeces_to_coordinates(23, 0);
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
                translation: Vec3::new(ben_init_x, ben_init_y, 10.),
                scale: Vec3::new(1./6., 1./6., 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(ben_materials);

    // Caleb
    let caleb_materials = CalebMaterials {
        default_material: materials.add(asset_server.load("../assets/ghosts/caleb.png").into()),
        scared_material: materials.add(asset_server.load("../assets/ghosts/caleb_scared.png").into()),
    };
    let caleb_init_x = board.cell_size() * board.width() as f32 / 2.;
    let (_, caleb_init_y) = board.indeces_to_coordinates(11, 0);
    commands.spawn_bundle(CalebBundle {
        ghost_bundle: GhostBundle {
            sprite_bundle: SpriteBundle {
                material: caleb_materials.default_material.clone(),
                transform: Transform {
                    translation: Vec3::new(caleb_init_x, caleb_init_y, 9.),
                    scale: Vec3::new(1./6., 1./6., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(caleb_materials);

    // Harris 
    let harris_materials = HarrisMaterials {
        default_material: materials.add(asset_server.load("../assets/ghosts/sam_h.png").into()),
        scared_material: materials.add(asset_server.load("../assets/ghosts/sam_h_scared.png").into()),
    };
    let harris_init_x = board.cell_size() * board.width() as f32 / 2. - board.cell_size() * 2.;
    let (_, harris_init_y) = board.indeces_to_coordinates(14, 0);
    commands.spawn_bundle(HarrisBundle {
        ghost_bundle: GhostBundle {
            sprite_bundle: SpriteBundle {
                material: harris_materials.default_material.clone(),
                transform: Transform {
                    translation: Vec3::new(harris_init_x, harris_init_y, 9.),
                    scale: Vec3::new(1./6., 1./6., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(harris_materials);

    // Claflin 
    let claflin_materials = ClaflinMaterials {
        default_material: materials.add(asset_server.load("../assets/ghosts/sam_c.png").into()),
        scared_material: materials.add(asset_server.load("../assets/ghosts/sam_c_scared.png").into()),
    };
    let claflin_init_x = board.cell_size() * board.width() as f32 / 2.;
    let (_, claflin_init_y) = board.indeces_to_coordinates(14, 0);
    commands.spawn_bundle(ClaflinBundle {
        ghost_bundle: GhostBundle {
            sprite_bundle: SpriteBundle {
                material: claflin_materials.default_material.clone(),
                transform: Transform {
                    translation: Vec3::new(claflin_init_x, claflin_init_y, 9.),
                    scale: Vec3::new(1./6., 1./6., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(claflin_materials);

    // Samson 
    let samson_materials = SamsonMaterials {
        default_material: materials.add(asset_server.load("../assets/ghosts/samson.png").into()),
        scared_material: materials.add(asset_server.load("../assets/ghosts/samson_scared.png").into()),
    };
    let samson_init_x = board.cell_size() * board.width() as f32 / 2. + board.cell_size() * 2.;
    let (_, samson_init_y) = board.indeces_to_coordinates(14, 0);
    commands.spawn_bundle(SamsonBundle {
        ghost_bundle: GhostBundle {
            sprite_bundle: SpriteBundle {
                material: samson_materials.default_material.clone(),
                transform: Transform {
                    translation: Vec3::new(samson_init_x, samson_init_y, 9.),
                    scale: Vec3::new(1./6., 1./6., 1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands.insert_resource(samson_materials);

    // Score
    let font = asset_server.load("../assets/font.ttf");
    let text_style = TextStyle {
        font,
        font_size: 35.,
        color: Color::WHITE
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Top,
        horizontal: HorizontalAlign::Center
    };
    commands.spawn_bundle(ScoreBundle {
        text_bundle: Text2dBundle {
            text: Text::with_section("", text_style, text_alignment),
            transform: Transform {
                translation: Vec3::new(board.width() as f32 * board.cell_size() / 2., board.height() as f32 * board.cell_size(), 10.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
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
    mut direction_changed_event: EventWriter<BenDirectionChangedEvent>,
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
        let initial_direction = ben_direction.0;
        match next_direction.unwrap() {
            Direction::Up => {
                if can_move_up {
                    ben_direction.0 = Direction::Up;
                }
            },
            Direction::Right => {
                if can_move_right {
                    ben_direction.0 = Direction::Right;
                }
            },
            Direction::Down => {
                if can_move_down {
                    ben_direction.0 = Direction::Down;
                }
            },
            Direction::Left => {
                if can_move_left {
                    ben_direction.0 = Direction::Left;
                }
            },
        }

        // Direction changed -> publish event
        if ben_direction.0 != initial_direction {
            direction_changed_event.send(BenDirectionChangedEvent(ben_direction.0));
            ben_next_direction.0 = None;
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

fn ben_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &mut BenAnimationTimer, &BenDirection), With<Ben>>,
    mut event_direction_changed: EventReader<BenDirectionChangedEvent>,
    time: Res<Time>,
    ben_materials: Res<BenMaterials>
) {
    let (mut material_handle, mut ben_animation_timer, ben_direction) = query.single_mut().unwrap();

    // Always update the sprite if the direction was just changed
    for event in event_direction_changed.iter() {
        update_ben_sprite(&mut material_handle, event.0, &ben_materials);
        return;
    }

    // Throttle the refresh rate
    let timer = &mut ben_animation_timer.0;
    timer.tick(time.delta());
    if !timer.finished() {
        return;
    }

    if material_handle.id != ben_materials.ben_default.id {
        material_handle.id = ben_materials.ben_default.id;
    } else {
        update_ben_sprite(&mut material_handle, ben_direction.0, &ben_materials);
    }
}

fn ben_dot_collision_system(
    mut commands: Commands,
    mut query_set: QuerySet<(
        Query<&Transform, With<Ben>>,
        Query<(Entity, &Transform), With<Dot>>,
        Query<&mut Score>,
    )>,
    board: Res<Board>,
    point_values: Res<PointValues>,
) {
    let ben_transform = query_set.q0().single().unwrap().clone();
    if utils::is_centered_horizontally(&ben_transform, &board) && utils::is_centered_vertically(&ben_transform, &board) {
        for (dot_entity, dot_transform) in query_set.q1().iter() {
            if dot_transform.translation.x == ben_transform.translation.x && dot_transform.translation.y == ben_transform.translation.y {
                commands.entity(dot_entity).despawn();
                query_set.q2_mut().single_mut().unwrap().0 += point_values.dot;
                break;
            }
        }
    }
}

fn ben_power_up_collision_system(
    mut commands: Commands,
    mut query_set: QuerySet<(
        Query<&Transform, With<Ben>>,
        Query<(Entity, &Transform, &mut Handle<ColorMaterial>), With<PowerUp>>,
        Query<&mut Score>
    )>,
    mut power_up_consumed_event: EventWriter<PowerUpConsumedEvent>,
    board: Res<Board>,
    point_values: Res<PointValues>,
    power_up_materials: Res<PowerUpMaterials>,
) {
    let ben_transform = query_set.q0().single().unwrap().clone();
    if utils::is_centered_horizontally(&ben_transform, &board) && utils::is_centered_vertically(&ben_transform, &board) {
        for (power_up_entity, power_up_transform, mut power_up_material) in query_set.q1_mut().iter_mut() {
            if power_up_transform.translation.x == ben_transform.translation.x && power_up_transform.translation.y == ben_transform.translation.y {
                reset_power_up_sprite(&mut power_up_material, &power_up_materials);
                commands.entity(power_up_entity).despawn();
                query_set.q2_mut().single_mut().unwrap().0 += point_values.power_up;
                power_up_consumed_event.send(PowerUpConsumedEvent);
                break;
            }
        }
    }
}

fn ben_ghost_collision_system(
    mut commands: Commands,
    mut query_set: QuerySet<(
        Query<&Transform, With<Ben>>,
        Query<(Entity, &Transform, &GhostStateComponent), With<Ghost>>,
        Query<&mut Score>
    )>,
    board: Res<Board>,
    point_values: Res<PointValues>,
) {
    let ben_transform = query_set.q0().single().unwrap().clone();
    for (ghost_entity, ghost_transform, ghost_state_component) in query_set.q1().iter() {
        if utils::did_collide(ghost_transform, &ben_transform, &board) {
            match ghost_state_component.0 {
                GhostState::Default => println!("Player dies"),
                GhostState::Respawning => { },
                GhostState::Scared => println!("Ghost dies"),
            }
        }
    }
}

fn power_up_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &mut PowerUpAnimationTimer), With<PowerUp>>,
    power_up_materials: Res<PowerUpMaterials>,
    time: Res<Time>
) {
    for (mut material_handle, mut power_up_animation_timer) in query.iter_mut() {
        let timer = &mut power_up_animation_timer.0; 
        timer.tick(time.delta());
        if !timer.finished() {
            continue;
        }

        material_handle.id = if material_handle.id == power_up_materials.material_1.id {
            power_up_materials.material_2.id
        } else {
            power_up_materials.material_1.id
        }
    }
}

fn scare_ghosts_system(
    mut query: Query<&mut GhostStateComponent, With<Ghost>>,
    mut power_up_consumed_event: EventReader<PowerUpConsumedEvent>,
    mut ghost_scare_timer: ResMut<GhostScareTimer>,
    time: Res<Time>
) {
    for _ in power_up_consumed_event.iter() {
        for mut ghost_state_component in query.iter_mut() {
            if ghost_state_component.0 == GhostState::Default {
                ghost_state_component.0 = GhostState::Scared;
            }
        }
    }

    let mut scared = false;
    for ghost_state_component in query.iter_mut() {
        if ghost_state_component.0 == GhostState::Scared {
            scared = true;
            break;
        }
    }

    if scared {
        let timer = &mut ghost_scare_timer.0;
        timer.tick(time.delta());
        if timer.finished() {
            for mut ghost_state_component in query.iter_mut() {
                if ghost_state_component.0 == GhostState::Scared {
                    ghost_state_component.0 = GhostState::Default; 
                }
            }

            timer.reset();
        }
    }
}

fn caleb_movement_system(
    mut query_set: QuerySet<(
        Query<(&mut Transform, &mut CalebPathChangeTimer, &mut GhostPath), With<Caleb>>,
        Query<&Transform, With<Ben>>
    )>,
    board: Res<Board>,
    time: Res<Time>
) {
    let ben_transform = query_set.q1().single().unwrap().clone(); 
    let (mut caleb_transform, mut caleb_path_change_timer, mut ghost_path) = query_set.q0_mut().single_mut().unwrap(); 

    if let Some((x, y)) = ghost_path.0.pop() {
        caleb_transform.translation.x = x;
        caleb_transform.translation.y = y;
    }

    let timer = &mut caleb_path_change_timer.0;
    timer.tick(time.delta());
    if !(timer.finished()) {
        return;
    }

    ghost_path.0 = Path::shortest_to_transform(&caleb_transform, &ben_transform, &board, 2.);
}

fn harris_movement_system(
    mut query: Query<(&mut Transform), With<Harris>>,
    board: Res<Board>
) {
    // TODO: Implement
    let harris_transform = query.single_mut().unwrap();
}

fn claflin_movement_system(
    mut query: Query<(&mut Transform), With<Claflin>>,
    board: Res<Board>
) {
    // TODO: Implement
    let claflin_transform = query.single_mut().unwrap();
}

fn samson_movement_system(
    mut query: Query<(&mut Transform), With<Samson>>,
    board: Res<Board>
) {
    // TODO: Implement
    let samson_transform = query.single_mut().unwrap();
}

fn caleb_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &GhostStateComponent), With<Caleb>>,
    caleb_materials: Res<CalebMaterials>
) {
    let (mut material_handle, ghost_state_component) = query.single_mut().unwrap();
    material_handle.id = match ghost_state_component.0 {
        GhostState::Default => caleb_materials.default_material.id,
        GhostState::Scared => caleb_materials.scared_material.id,
        _ => caleb_materials.default_material.id
    };
}

fn harris_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &GhostStateComponent), With<Harris>>,
    harris_materials: Res<HarrisMaterials>
) {
    let (mut material_handle, ghost_state_component) = query.single_mut().unwrap();
    material_handle.id = match ghost_state_component.0 {
        GhostState::Default => harris_materials.default_material.id,
        GhostState::Scared => harris_materials.scared_material.id,
        _ => harris_materials.default_material.id
    };
}

fn claflin_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &GhostStateComponent), With<Claflin>>,
    claflin_materials: Res<ClaflinMaterials>
) {
    let (mut material_handle, ghost_state_component) = query.single_mut().unwrap();
    material_handle.id = match ghost_state_component.0 {
        GhostState::Default => claflin_materials.default_material.id,
        GhostState::Scared => claflin_materials.scared_material.id,
        _ => claflin_materials.default_material.id
    };
}

fn samson_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &GhostStateComponent), With<Samson>>,
    samson_materials: Res<SamsonMaterials>
) {
    let (mut material_handle, ghost_state_component) = query.single_mut().unwrap();
    material_handle.id = match ghost_state_component.0 {
        GhostState::Default => samson_materials.default_material.id,
        GhostState::Scared => samson_materials.scared_material.id,
        _ => samson_materials.default_material.id
    };
}

fn score_system(
    mut query: Query<(&mut Text, &Score)>
) {
    let (mut text, score) = query.single_mut().unwrap();
    text.sections[0].value = format!("Score: {}", score.0);
}

fn update_ben_sprite(
    material_handle: &mut Handle<ColorMaterial>,
    direction: Direction,
    ben_materials: &BenMaterials
) {
    material_handle.id = match direction {
        Direction::Up => ben_materials.ben_up.id,
        Direction::Right => ben_materials.ben_right.id, 
        Direction::Down => ben_materials.ben_down.id,
        Direction::Left => ben_materials.ben_left.id,
    };
}

fn reset_power_up_sprite(
    material_handle: &mut Handle<ColorMaterial>,
    power_up_materials: &PowerUpMaterials,
) {
    material_handle.id = power_up_materials.material_1.id;
}