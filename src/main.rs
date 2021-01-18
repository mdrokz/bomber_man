use bevy::prelude::*;

struct Player {
    x: f32,
    y: f32
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_system(hello_world.system())
    .run();
}

fn hello_world() {
    println!("hello world");
}