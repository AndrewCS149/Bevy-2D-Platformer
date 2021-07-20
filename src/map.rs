use crate::SCALE;
use bevy::prelude::*;
use bevy_rapier2d::{
    physics::{ColliderBundle, ColliderPositionSync},
    prelude::ColliderShape,
    render::ColliderDebugRender,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(boundaries.system());
    }
}

fn boundaries(
    mut commands: Commands,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.get_primary().unwrap();

    // width and height
    let width = 1.0;
    let height = window.height();

    let x = window.width() / 2.0 / SCALE + width;
    let y = 0.0;

    // a closure to use for the left, right, bottom and top colliders of the map
    let collider = |x: f32, y: f32, w: f32, h: f32| -> ColliderBundle {
        ColliderBundle {
            position: [x, y].into(),
            shape: ColliderShape::cuboid(w, h),
            ..Default::default()
        }
    };

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
    let height = 5.0;

    let sprite = SpriteBundle {
        material: materials.add(Color::rgb(0.08, 0.58, 0.0).into()),
        // the sprite vector is directly proportionate to the collider size.
        // eg 1: a new vec of 'x: 1.0, y: 1.0' is the same exact size as the collider
        // eg 2: a vec of 'x: 2.0, y: 2.0' is twice as large as the collider
        sprite: Sprite::new(Vec2::new(1.0, 1.0)),
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(sprite)
        .insert_bundle(collider(x, -y, width, height))
        .insert(ColliderDebugRender::default())
        // syncs the collider position with the sprite position
        .insert(ColliderPositionSync::Discrete);
}
