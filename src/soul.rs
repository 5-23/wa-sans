use bevy::prelude::*;

pub struct MainPlugin;

#[derive(Component, Clone)]
pub struct Soul {
    pub hp: f32,
}

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        println!("starting soul plugin");
        app.add_systems(Update, movement);
    }
}

fn movement(mut soul: Query<&mut Transform, With<Soul>>, input: Res<Input<KeyCode>>) {
    let mut soul_transform = soul.iter_mut().next().unwrap();
    if input.pressed(KeyCode::Up) {
        soul_transform.translation.y += 4.;
    }
}
