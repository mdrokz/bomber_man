use bevy::prelude::*;

#[macro_use]
extern crate bevy_derive;

struct Player {
    x: f32,
    y: f32
}

#[bevy_derive::main(systems = "hello_world,test,test1")]
fn main() {
}

fn hello_world() {
    println!("hello world");
}

fn test() {
    println!("hello world");
}

fn test1() {
    println!("hello world");
}