use bevy::prelude::*;

use crate::boxsys::BarType;
pub struct MainPlugin;

#[derive(Component)]
pub struct Bone {}

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement);
        println!("hi")
    }
}

fn movement(mut Bone: Query<(&mut Transform, &Bone), With<Bone>>) {}
