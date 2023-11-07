use bevy::prelude::*;
pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        println!("running Box System")
    }
}
