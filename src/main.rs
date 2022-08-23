use bevy::prelude::*;

pub mod prelude {
    pub use bevy::prelude::*;

    pub use crate::{entities::*, movement::*};
}

mod bullet;
mod entities;
mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(entities::EntitiesPlugin)
        .add_startup_system(setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}