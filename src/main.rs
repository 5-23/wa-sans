use bevy::prelude::*;
mod sans;
#[derive(Component, Clone)]
struct Soul {
    hp: i32,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(sans::MainPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
