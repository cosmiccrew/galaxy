use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("hello world!");
}