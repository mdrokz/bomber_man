use bevy::{input::system, prelude::*};

#[macro_use]
extern crate bevy_derive;

struct Player {
    x: f32,
    y: f32
}


#[bevy_derive::main(system = "hello_world,animate_sprite_system",startup_system = "setup")]
fn main() {

}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta_seconds());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("resources/sprites/walk_right.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 24.0), 6, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..Default::default()
        })
        .with(Timer::from_seconds(0.1, true));
}

fn hello_world() {
    // println!("hello world");
}
