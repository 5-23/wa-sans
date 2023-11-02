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
    let mut speed = 4.;
    if input.pressed(KeyCode::X) {
        speed = 2.;
    }
    if input.pressed(KeyCode::Up) {
        soul_transform.translation.y += speed;
    }
    if input.pressed(KeyCode::Down) {
        soul_transform.translation.y -= speed;
    }
    if input.pressed(KeyCode::Left) {
        soul_transform.translation.x -= speed;
    }
    if input.pressed(KeyCode::Right) {
        soul_transform.translation.x += speed;
    }
}
