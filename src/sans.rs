pub const SIZE: f32 = 2.;
static mut START_TIME: f32 = 0.0;
use bevy::prelude::*;
pub struct MainPlugin;

#[derive(Component)]
pub struct Sans;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        println!("starting sans plugin");
        app.add_systems(Update, body_animation);
    }
}

fn body_animation(mut commands: Commands, mut body: Query<(&mut Transform, &Sprite), With<Sans>>) {
    unsafe { START_TIME += 0.1 }
    let mut sans = body.iter_mut();
    let (leg_transform, leg_sprite) = sans.next().unwrap();
    let (mut body_transform, body_sprite) = sans.next().unwrap();
    let (mut head_transform, head_sprite) = sans.next().unwrap();
    body_transform.translation.x =
        leg_transform.translation.x + f32::sin(unsafe { START_TIME } / 2.) * 2.;
    body_transform.translation.y = leg_transform.translation.y + leg_sprite.custom_size.unwrap()[1]
        - 15. * SIZE
        + f32::sin(unsafe { START_TIME }) * 2.;

    head_transform.translation.x = body_transform.translation.x;
    head_transform.translation.y =
        body_transform.translation.y + body_sprite.custom_size.unwrap()[1] - 15. * SIZE;
}
