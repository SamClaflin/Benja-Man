mod board;
mod ben;

use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, WindowOrigin},
};
use board::{Board, BoardTile, WallType};
use ben::{Ben, BenBundle, BenDirection};

struct MapMaterials {
    corner_bottom_left: Handle<ColorMaterial>,
    corner_bottom_left_solid: Handle<ColorMaterial>,
    corner_bottom_right: Handle<ColorMaterial>,
    corner_bottom_right_solid: Handle<ColorMaterial>,
    corner_top_left: Handle<ColorMaterial>,
    corner_top_left_solid: Handle<ColorMaterial>,
    corner_top_right: Handle<ColorMaterial>,
    corner_top_right_solid: Handle<ColorMaterial>,
    wall_bottom_hollow: Handle<ColorMaterial>,
    wall_bottom_solid: Handle<ColorMaterial>,
    wall_left_hollow: Handle<ColorMaterial>,
    wall_left_solid: Handle<ColorMaterial>,
    wall_right_hollow: Handle<ColorMaterial>,
    wall_right_solid: Handle<ColorMaterial>,
    wall_top_hollow: Handle<ColorMaterial>,
    wall_top_solid: Handle<ColorMaterial>,
    corner_bottom_left_enclosed: Handle<ColorMaterial>,
    corner_bottom_right_enclosed: Handle<ColorMaterial>,
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
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
    let map_materials = MapMaterials {
        corner_bottom_left: materials.add(asset_server.load("../assets/walls/corner_bottom_left.png").into()),
        corner_bottom_left_solid: materials.add(asset_server.load("../assets/walls/corner_bottom_left_solid.png").into()),
        corner_bottom_right: materials.add(asset_server.load("../assets/walls/corner_bottom_right.png").into()),
        corner_bottom_right_solid: materials.add(asset_server.load("../assets/walls/corner_bottom_right_solid.png").into()),
        corner_top_left: materials.add(asset_server.load("../assets/walls/corner_top_left.png").into()),
        corner_top_left_solid: materials.add(asset_server.load("../assets/walls/corner_top_left_solid.png").into()),
        corner_top_right: materials.add(asset_server.load("../assets/walls/corner_top_right.png").into()),
        corner_top_right_solid: materials.add(asset_server.load("../assets/walls/corner_top_right_solid.png").into()),
        wall_bottom_hollow: materials.add(asset_server.load("../assets/walls/wall_bottom_hollow.png").into()),
        wall_bottom_solid: materials.add(asset_server.load("../assets/walls/wall_bottom_solid.png").into()),
        wall_left_hollow: materials.add(asset_server.load("../assets/walls/wall_left_hollow.png").into()),
        wall_left_solid: materials.add(asset_server.load("../assets/walls/wall_left_solid.png").into()),
        wall_right_hollow: materials.add(asset_server.load("../assets/walls/wall_right_hollow.png").into()),
        wall_right_solid: materials.add(asset_server.load("../assets/walls/wall_right_solid.png").into()),
        wall_top_hollow: materials.add(asset_server.load("../assets/walls/wall_top_hollow.png").into()),
        wall_top_solid: materials.add(asset_server.load("../assets/walls/wall_top_solid.png").into()),
        corner_bottom_left_enclosed: materials.add(asset_server.load("../assets/walls/corner_bottom_left_enclosed.png").into()),
        corner_bottom_right_enclosed: materials.add(asset_server.load("../assets/walls/corner_bottom_right_enclosed.png").into()),
    };
    let dot_material_handle = materials.add(asset_server.load("../assets/cookie.png").into());
    for i in 0..board.height() {
        for j in 0..board.width() {
            let curr_tile = board.get_at(i, j).unwrap();
            let material: Handle<ColorMaterial>;
            let scale_factor: f32;
            match curr_tile {
                BoardTile::Empty => continue,
                BoardTile::Wall(wall_type) => {
                    material = get_material_from_wall_type(wall_type, &map_materials);
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
            let (x, y) = board.indeces_to_coordinates(i, j);
            commands.spawn_bundle(SpriteBundle {
                material,
                sprite: Sprite::new(Vec2::new(board.cell_size() as f32, board.cell_size() as f32)),
                transform: Transform {
                    translation: Vec3::new(
                        x * board.cell_size() as f32 + board.offset(),
                        y * board.cell_size() as f32 + board.offset(),
                        1.
                    ),
                    scale,
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
    commands.insert_resource(map_materials);

    // Ben
    // TODO: TextureAtlas
    commands.spawn_bundle(BenBundle {
        sprite_sheet_bundle: SpriteSheetBundle {
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Ben);
}

fn get_material_from_wall_type(wall_type: WallType, map_materials: &MapMaterials) -> Handle<ColorMaterial> {
    match wall_type {
        WallType::CornerBottomLeft => map_materials.corner_bottom_left.clone(),
        WallType::CornerBottomLeftSolid=> map_materials.corner_bottom_left_solid.clone(),
        WallType::CornerBottomRight => map_materials.corner_bottom_right.clone(),
        WallType::CornerBottomRightSolid => map_materials.corner_bottom_right_solid.clone(),
        WallType::CornerTopLeft => map_materials.corner_top_left.clone(),
        WallType::CornerTopLeftSolid => map_materials.corner_top_left_solid.clone(),
        WallType::CornerTopRight => map_materials.corner_top_right.clone(),
        WallType::CornerTopRightSolid => map_materials.corner_top_right_solid.clone(),
        WallType::WallBottomHollow => map_materials.wall_bottom_hollow.clone(),
        WallType::WallBottomSolid => map_materials.wall_bottom_solid.clone(),
        WallType::WallLeftHollow => map_materials.wall_left_hollow.clone(),
        WallType::WallLeftSolid => map_materials.wall_left_solid.clone(),
        WallType::WallRightHollow => map_materials.wall_right_hollow.clone(),
        WallType::WallRightSolid => map_materials.wall_right_solid.clone(),
        WallType::WallTopHollow => map_materials.wall_top_hollow.clone(),
        WallType::WallTopSolid => map_materials.wall_top_solid.clone(),
        WallType::CornerBottomLeftEnclosed => map_materials.corner_bottom_left_enclosed.clone(),
        WallType::CornerBottomRightEnclosed => map_materials.corner_bottom_right_enclosed.clone(),
    }
}
