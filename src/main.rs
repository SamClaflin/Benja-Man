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
mod constants;

use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, WindowOrigin}
};
use ghost::{
    Ghost, 
    GhostPath, 
    AttackState, 
    ReleaseState,
    GhostSpeed,
    GhostBundle,
    Caleb,
    CalebBundle, 
    CalebMaterials,
    Harris, 
    HarrisMaterials,
    HarrisBundle, 
    Claflin, 
    ClaflinMaterials,
    ClaflinBundle, 
    Samson, 
    SamsonMaterials,
    SamsonBundle,
    GhostScareTimer,
    GhostReleaseTimer,
    GhostChain
};
use board::{Board, BoardTile};
use ben::{Ben, BenBundle, BenAnimationTimer, BenSpeed, BenDirection, BenNextDirection, BenMaterials};
use enums::{Direction, GameState, Label, CollisionType};
use dot::{Dot, DotBundle};
use score::{Score, ScoreBundle, PointValues};
use events::{BenDirectionChangedEvent, PowerUpConsumedEvent};
use power_up::{PowerUp, PowerUpBundle, PowerUpMaterials, PowerUpAnimationTimer};
use path::Path;

struct SoundMaterials {
    background_sound: Handle<AudioSource>,
    slurp_sound: Handle<AudioSource>,
    death_sound: Handle<AudioSource>
}

struct BackgroundMusicTimer(Timer);

struct FontMaterial {
    handle: Handle<Font>
}

struct StartMessage;

fn main() {
    let board = Board::new(32., 16.);

    App::build()
        // Resources
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
        .init_resource::<GhostReleaseTimer>()
        .init_resource::<GhostChain>()

        // Events
        .add_event::<BenDirectionChangedEvent>()
        .add_event::<PowerUpConsumedEvent>()

        // State
        .add_state(GameState::InitialLoad)

        // Startup
        .add_startup_system(setup.system())

        // Game start
        .add_system_set(
            SystemSet::on_update(GameState::InitialLoad)
                .with_system(wait_for_asset_load_system.system())
        )

        // Mainloop
        .add_system_set(
            SystemSet::on_update(GameState::Default)
                .with_system(ben_controller_system.system().label(Label::BenControllerSystem))
                .with_system(ben_movement_system.system().label(Label::BenMovementSystem).after(Label::BenControllerSystem))
                .with_system(ben_dot_collision_system.system().after(Label::BenMovementSystem))
                .with_system(ben_power_up_collision_system.system().after(Label::BenMovementSystem)) 
                .with_system(ben_ghost_collision_system.system().label(Label::BenGhostCollisionSystem).after(Label::BenMovementSystem))
                .with_system(ben_animation_system.system())
                .with_system(scare_ghosts_system.system())
                .with_system(ghost_movement_system.system())
                .with_system(caleb_animation_system.system())
                .with_system(harris_animation_system.system())
                .with_system(claflin_animation_system.system())
                .with_system(samson_animation_system.system())
                .with_system(win_system.system())
                .with_system(ghost_release_system.system())
                .with_system(ghost_respawn_system.system().after(Label::BenGhostCollisionSystem))
        )

        // Miscellaneous
        .add_system(power_up_animation_system.system())
        .add_system(score_system.system())
        .add_system(background_music_system.system())
        
        // Plugins
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
            release_state: ReleaseState::Released,
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

    // Score and start message
    let font_material = FontMaterial {
        handle: asset_server.load("../assets/font.ttf")
    };
    let font = font_material.handle.clone();
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
            text: Text::with_section("", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(board.width() as f32 * board.cell_size() / 2., board.height() as f32 * board.cell_size(), 10.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Press space to start", text_style, text_alignment),
        transform: Transform {
            translation: Vec3::new(board.width() as f32 * board.cell_size() / 2., board.height() as f32 * board.cell_size() / 2. + 256., 15.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(StartMessage);
    commands.insert_resource(font_material);

    // Sounds
    commands.insert_resource(SoundMaterials {
        background_sound: asset_server.load("../assets/sounds/guts_theme.mp3"),
        slurp_sound: asset_server.load("../assets/sounds/slurp.mp3"),
        death_sound: asset_server.load("../assets/sounds/cringe.mp3")
    });

    // Background music timer
    commands.insert_resource(BackgroundMusicTimer(Timer::from_seconds(215., false)));
}

fn wait_for_asset_load_system(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    query: Query<Entity, With<StartMessage>>,
    keys: Res<Input<KeyCode>>
) {
    let start_message_entity = query.single().unwrap();
    if keys.just_pressed(KeyCode::Space) {
        commands.entity(start_message_entity).despawn();
        game_state.set(GameState::Default).unwrap();
    }
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
    let (target_x, target_y) = board.get_coordinates(transform.translation.x, transform.translation.y, direction, speed);
    if can_move_up && direction == Direction::Up
    || can_move_right && direction == Direction::Right
    || can_move_down && direction == Direction::Down
    || can_move_left && direction == Direction::Left {
        transform.translation.x = target_x;
        transform.translation.y = target_y;
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
        *material_handle = ben_materials.ben_default.clone();
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
        Query<(Entity, &Transform), With<PowerUp>>,
        Query<&mut Score>
    )>,
    mut power_up_consumed_event: EventWriter<PowerUpConsumedEvent>,
    mut ghost_chain: ResMut<GhostChain>,
    board: Res<Board>,
    point_values: Res<PointValues>,
    sound_materials: Res<SoundMaterials>,
    audio: Res<Audio>
) {
    let ben_transform = query_set.q0().single().unwrap().clone();
    if utils::is_centered_horizontally(&ben_transform, &board) && utils::is_centered_vertically(&ben_transform, &board) {
        for (power_up_entity, power_up_transform) in query_set.q1_mut().iter_mut() {
            if power_up_transform.translation.x == ben_transform.translation.x && power_up_transform.translation.y == ben_transform.translation.y {
                commands.entity(power_up_entity).despawn();
                query_set.q2_mut().single_mut().unwrap().0 += point_values.power_up;
                ghost_chain.0 = 0;
                power_up_consumed_event.send(PowerUpConsumedEvent);
                audio.play(sound_materials.slurp_sound.clone());
                break;
            }
        }
    }
}

fn ben_ghost_collision_system(
    mut game_state: ResMut<State<GameState>>,
    mut query_set: QuerySet<(
        Query<&Transform, With<Ben>>,
        Query<(&Transform, &AttackState, &mut ReleaseState, &mut GhostPath), With<Ghost>>,
        Query<&mut Score>
    )>,
    mut ghost_chain: ResMut<GhostChain>,
    board: Res<Board>,
    point_values: Res<PointValues>,
    sound_materials: Res<SoundMaterials>,
    audio: Res<Audio>
) {
    let ben_transform = query_set.q0().single().unwrap().clone();
    let mut points = 0;
    for (ghost_transform, attack_state, mut release_state, mut ghost_path) in query_set.q1_mut().iter_mut() {
        if utils::did_collide(ghost_transform, &ben_transform, &board, CollisionType::Approximate) {
            match attack_state {
                AttackState::Attacking => {
                    game_state.set(GameState::Lose).unwrap();
                    audio.play(sound_materials.death_sound.clone())
                },
                AttackState::Scared => {
                    if *release_state == ReleaseState::Respawning {
                        continue;
                    }

                    *release_state = ReleaseState::Respawning;
                    points += match ghost_chain.0 {
                        0 => point_values.first_ghost,
                        1 => point_values.second_ghost,
                        2 => point_values.third_ghost,
                        3 => point_values.fourth_ghost,
                        _ => point_values.fourth_ghost
                    };
                    ghost_chain.0 += 1;
                    ghost_path.0.clear();
                }
            }
        }
    }

    let score = &mut query_set.q2_mut().single_mut().unwrap();
    score.0 += points;
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

        *material_handle = if material_handle.id == power_up_materials.material_1.id {
            power_up_materials.material_2.clone()
        } else {
            power_up_materials.material_1.clone()
        }
    }
}

fn scare_ghosts_system(
    mut query: Query<&mut AttackState, With<Ghost>>,
    mut power_up_consumed_event: EventReader<PowerUpConsumedEvent>,
    mut ghost_scare_timer: ResMut<GhostScareTimer>,
    time: Res<Time>
) {
    let timer = &mut ghost_scare_timer.0;

    for _ in power_up_consumed_event.iter() {
        timer.reset();
        for mut attack_state in query.iter_mut() {
            if *attack_state == AttackState::Attacking {
                *attack_state = AttackState::Scared;
            }
        }
    }

    let mut scared = false;
    for attack_state in query.iter_mut() {
        if *attack_state == AttackState::Scared {
            scared = true;
            break;
        }
    }

    if scared {
        timer.tick(time.delta());
        if timer.finished() {
            for mut attack_state in query.iter_mut() {
                if *attack_state == AttackState::Scared {
                    *attack_state = AttackState::Attacking; 
                }
            }

            timer.reset();
        }
    }
}

fn ghost_movement_system(
    mut query_set: QuerySet<(
        Query<(&mut Transform, &mut GhostPath, &GhostSpeed, &ReleaseState), With<Ghost>>,
        Query<&Transform, With<Ben>>
    )>,
    board: Res<Board>,
) {
    let ben_transform = query_set.q1().single().unwrap().clone(); 
    for (mut caleb_transform, mut ghost_path, ghost_speed, release_state) in query_set.q0_mut().iter_mut() {
        if *release_state != ReleaseState::Released {
            continue; 
        }

        if let Some((x, y)) = ghost_path.0.pop_front() {
            caleb_transform.translation.x = x;
            caleb_transform.translation.y = y;
        } else {
            ghost_path.0 = Path::shortest_to_transform(
                &caleb_transform, 
                &ben_transform, 
                &board, 
                ghost_speed.0, 
                CollisionType::Approximate
            );
        }
    }
}

fn caleb_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &AttackState), With<Caleb>>,
    caleb_materials: Res<CalebMaterials>
) {
    let (mut material_handle, attack_state) = query.single_mut().unwrap();
    *material_handle = match attack_state {
        AttackState::Attacking => caleb_materials.default_material.clone(),
        AttackState::Scared => caleb_materials.scared_material.clone(),
    };
}

fn harris_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &AttackState), With<Harris>>,
    harris_materials: Res<HarrisMaterials>
) {
    let (mut material_handle, attack_state) = query.single_mut().unwrap();
    *material_handle = match attack_state {
        AttackState::Attacking => harris_materials.default_material.clone(),
        AttackState::Scared => harris_materials.scared_material.clone(),
    };
}

fn claflin_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &AttackState), With<Claflin>>,
    claflin_materials: Res<ClaflinMaterials>
) {
    let (mut material_handle, attack_state) = query.single_mut().unwrap();
    *material_handle = match attack_state {
        AttackState::Attacking => claflin_materials.default_material.clone(),
        AttackState::Scared => claflin_materials.scared_material.clone(),
    };
}

fn samson_animation_system(
    mut query: Query<(&mut Handle<ColorMaterial>, &AttackState), With<Samson>>,
    samson_materials: Res<SamsonMaterials>
) {
    let (mut material_handle, attack_state) = query.single_mut().unwrap();
    *material_handle = match attack_state {
        AttackState::Attacking => samson_materials.default_material.clone(),
        AttackState::Scared => samson_materials.scared_material.clone(),
    };
}

fn score_system(
    mut query: Query<(&mut Text, &Score)>
) {
    let (mut text, score) = query.single_mut().unwrap();
    text.sections[0].value = format!("Score: {}", score.0);
}

fn win_system(
    mut commands: Commands,
    mut game_state: ResMut<State<GameState>>,
    query: Query<&Dot>,
    board: Res<Board>,
    font_material: Res<FontMaterial>
) {
    let mut did_win = true;
    for _ in query.iter() {
        did_win = false;
        break;
    }

    if did_win {
        game_state.set(GameState::Win).unwrap();
        let text_style = TextStyle {
            font: font_material.handle.clone(),
            font_size: 35.,
            color: Color::WHITE
        };
        let text_alignment = TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center
        };
        commands.spawn_bundle(Text2dBundle {
            text: Text::with_section("Based\nAND\nRed-Pilled", text_style, text_alignment),
            transform: Transform {
                translation: Vec3::new(board.width() as f32 * board.cell_size() / 2., board.height() as f32 * board.cell_size() / 2., 15.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub fn ghost_release_system(
    mut ghost_release_timer: ResMut<GhostReleaseTimer>,
    mut query: Query<(&mut ReleaseState, &mut Transform, &GhostSpeed), With<Ghost>>,
    board: Res<Board>,
    time: Res<Time>
) {
    // First pass: determine if any ghosts are currently being released
    let mut any_caged = false;
    let mut currently_releasing = false;
    for (release_state, _, _) in query.iter_mut() {
        match *release_state {
            ReleaseState::Caged => any_caged = true,
            ReleaseState::Releasing => currently_releasing = true,
            _ => { }
        }
    }

    // If no ghosts are currently caged or being released, no work needs to be done 
    if !any_caged && !currently_releasing {
        return;
    }

    // Second pass: perform necessary operations
    for (mut release_state, mut ghost_transform, ghost_speed) in query.iter_mut() {
        match *release_state {
            ReleaseState::Caged => {
                // Ghosts that are currently being released must be prioritized
                if currently_releasing {
                    continue;
                }

                let timer = &mut ghost_release_timer.0;
                timer.tick(time.delta());
                if !timer.finished() {
                    return;
                }

                *release_state = ReleaseState::Releasing;
                timer.reset();
                return;
            }, 
            ReleaseState::Releasing => {
                // Step 1: Get centered within the cage
                let x_target = board.width() as f32 * board.cell_size() / 2.;
                if ghost_transform.translation.x < x_target {
                    ghost_transform.translation.x += ghost_speed.0;
                    return;
                } else if ghost_transform.translation.x > x_target {
                    ghost_transform.translation.x -= ghost_speed.0;
                    return;
                }

                // Step 2: Move upward 
                let (_, y_target) = board.indeces_to_coordinates(11, 0);
                if ghost_transform.translation.y < y_target {
                    ghost_transform.translation.y += ghost_speed.0;
                    return;
                } else {
                    *release_state = ReleaseState::Released;
                    return;
                }
            },
            _ => continue
        }
    }
}

fn ghost_respawn_system(
    mut query: Query<(&mut Transform, &mut ReleaseState, &mut GhostPath, &mut AttackState), With<Ghost>>,
    board: Res<Board>
) {
    for (mut transform, mut release_state, mut ghost_path, mut attack_state) in query.iter_mut() {
        if *release_state != ReleaseState::Respawning {
            continue;
        }

        if let Some((x, y)) = ghost_path.0.pop_front() {
            let (target_x, target_y) = utils::get_ghost_spawn_coordinates(&board);
            transform.translation.x = x;
            transform.translation.y = y;

            if transform.translation.x == target_x && transform.translation.y == target_y {
                *release_state = ReleaseState::Caged;
                *attack_state = AttackState::Attacking;
            }
        }
        else {
            // Hack to ensure that the ghost is centered on a tile before searching for a path to the spawn point.
            // This is required to prevent the ghost from never finding a path.
            if !utils::is_centered_horizontally(&transform, &board) {
                transform.translation.x -= transform.translation.x % board.cell_size();
            } 
            if !utils::is_centered_vertically(&transform, &board) {
                transform.translation.y -= transform.translation.y % board.cell_size();
            }

            ghost_path.0 = Path::shortest_to_ghost_spawn(&transform, &board, constants::GHOST_SPEED_RESPAWNING);
        }
    }
}

fn background_music_system(
    mut background_music_timer: ResMut<BackgroundMusicTimer>,
    sound_materials: Res<SoundMaterials>,
    audio: Res<Audio>,
    time: Res<Time>
) {
    let timer = &mut background_music_timer.0;
    if timer.elapsed_secs() == 0. {
        audio.play(sound_materials.background_sound.clone());
    }

    timer.tick(time.delta());
    if timer.finished() {
        timer.reset()
    }
}

fn update_ben_sprite(
    material_handle: &mut Handle<ColorMaterial>,
    direction: Direction,
    ben_materials: &BenMaterials
) {
    *material_handle = match direction {
        Direction::Up => ben_materials.ben_up.clone(),
        Direction::Right => ben_materials.ben_right.clone(), 
        Direction::Down => ben_materials.ben_down.clone(),
        Direction::Left => ben_materials.ben_left.clone(),
    };
}
