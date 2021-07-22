use crate::SCALE;
use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{ColliderBundle, ColliderPositionSync},
    prelude::*,
    render::ColliderDebugRender,
};
use std::collections::HashMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(boundaries.system())
            .add_startup_system(map_matrix.system());
    }
}

fn map_matrix(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    /*
    2560 / 10 (SCALE) / 12 = 21.333333
    1440 / 10 (SCALE) / 12 = 12

    1 unit width = 10.665
    1 unit length = 6
    */

    let window = windows.get_primary().unwrap();

    let map = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    ];

    let mut x_conversion = HashMap::new();
    x_conversion.insert(0, -6);
    x_conversion.insert(1, -5);
    x_conversion.insert(2, -4);
    x_conversion.insert(3, -3);
    x_conversion.insert(4, -2);
    x_conversion.insert(5, -1);
    x_conversion.insert(6, 1);
    x_conversion.insert(7, 2);
    x_conversion.insert(8, 3);
    x_conversion.insert(9, 4);
    x_conversion.insert(10, 5);
    x_conversion.insert(11, 6);
    // println!("{}", pos_conversion.get(&0).unwrap());

    let unit_width = window.width() / SCALE / 12.0;
    let unit_height = window.height() / SCALE / 12.0 / 2.0;

    // a closure
    let collider = |x: f32, y: f32, w: f32, h: f32| -> ColliderBundle {
        ColliderBundle {
            position: [x, y].into(),
            shape: ColliderShape::cuboid(w, h),
            material: ColliderMaterial {
                friction: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    };

    let height = 0;
    for (r, row) in map.iter().enumerate() {
        // collider width in units as defined above
        let mut units = 0;
        let mut starting_idx = 0;
        for (c, col) in row.iter().enumerate() {
            if col != &0 {
                units += 1;
            } else {
                starting_idx = col.clone();
            }

            if &units > &0 && map[r][c + 1] == 0 || &units > &0 && c + 1 > row.len() {
                let collider_width = units as f32 * unit_width;
                println!("width {}", collider_width);
                let collider_height = unit_height;
                let collider_x =
                    x_conversion.get(&starting_idx).unwrap().clone() as f32 * unit_width;
                // let collider_x = -((row.len() - c) as f32 * unit_width);
                let collider_y = -((map.len() - (map.len() - r)) as f32 * unit_height);

                let sprite = SpriteBundle {
                    material: materials.add(Color::rgb(0.08, 0.58, 0.0).into()),
                    // the sprite vector is directly proportionate to the collider size.
                    // eg 1: a new vec of 'x: 1.0, y: 1.0' is the same exact size as the collider
                    // eg 2: a vec of 'x: 2.0, y: 2.0' is twice as large as the collider
                    sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                    ..Default::default()
                };

                units = 0;
                commands
                    .spawn()
                    .insert_bundle(sprite)
                    .insert_bundle(collider(
                        collider_x,
                        collider_y,
                        collider_width,
                        collider_height,
                    ))
                    .insert(ColliderDebugRender::default())
                    .insert(ColliderPositionSync::Discrete);
            }

            // if col == &1 {
            // } else if col == &2 {
            // }
        }
    }
}

fn boundaries(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.get_primary().unwrap();

    // a closure to use for the left, right, bottom and top colliders of the map
    let collider = |x: f32, y: f32, w: f32, h: f32| -> ColliderBundle {
        ColliderBundle {
            position: [x, y].into(),
            shape: ColliderShape::cuboid(w, h),
            material: ColliderMaterial {
                friction: 0.0,
                ..Default::default()
            },
            ..Default::default()
        }
    };

    // width and height
    let width = 1.0;
    let height = window.height();

    let x = window.width() / 2.0 / SCALE + width;
    let y = 0.0;

    // right
    commands.spawn_bundle(collider(x, y, width, height));

    // left
    commands.spawn_bundle(collider(-x, y, width, height));

    // top
    let width = window.width() / 2.0;
    let height = 1.0;
    let x = 0.0;
    let y = window.height() / 2.0 / SCALE + height;

    commands.spawn_bundle(collider(x, y, width, height));

    // bottom
    // let height = 5.0;

    // let sprite = SpriteBundle {
    //     material: materials.add(Color::rgb(0.08, 0.58, 0.0).into()),
    //     // the sprite vector is directly proportionate to the collider size.
    //     // eg 1: a new vec of 'x: 1.0, y: 1.0' is the same exact size as the collider
    //     // eg 2: a vec of 'x: 2.0, y: 2.0' is twice as large as the collider
    //     sprite: Sprite::new(Vec2::new(1.0, 1.0)),
    //     ..Default::default()
    // };

    // commands
    //     .spawn()
    //     .insert_bundle(sprite)
    //     .insert_bundle(collider(x, -y, width, height))
    //     .insert(ColliderDebugRender::default())
    //     // syncs the collider position with the sprite position
    //     .insert(ColliderPositionSync::Discrete);
}
