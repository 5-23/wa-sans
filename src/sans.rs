pub const SIZE: f32 = 5.;
static mut START_TIME: f32 = 0.0;
use bevy::prelude::*;
pub struct MainPlugin;

#[derive(Component)]
pub struct Sans;

#[derive(Component)]
pub struct Clock;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        println!("starting sans plugin");
        app.add_systems(Update, body_animation);
        app.add_systems(Update, clock_animation);
    }
}

fn body_animation(mut commands: Commands, mut sans: Query<(&mut Transform, &Sprite), With<Sans>>) {
    unsafe { START_TIME += 0.1 }
    let mut sans = sans.iter_mut();
    let (mut leg_transform, leg_sprite) = sans.next().unwrap();
    let (mut body_transform, body_sprite) = sans.next().unwrap();
    let (mut head_transform, head_sprite) = sans.next().unwrap();
    leg_transform.translation.z = 0.;

    body_transform.translation.z = 0.1;
    body_transform.rotate_z(f32::sin(unsafe { START_TIME / 2. }) / 800.);
    body_transform.translation.x =
        leg_transform.translation.x + f32::sin(unsafe { START_TIME } / 2.) * 2. - SIZE / 2.;
    body_transform.translation.y = leg_transform.translation.y
        + leg_sprite.custom_size.unwrap()[1]
        + f32::sin(unsafe { START_TIME }) * 2.;

    head_transform.translation.z = 0.2;
    head_transform.rotate_z(f32::sin(unsafe { START_TIME / 2. }) / 800.);
    head_transform.translation.x =
        body_transform.translation.x + SIZE + f32::sin(unsafe { START_TIME } / 2.) / 2. * 2.
            - SIZE / 2.;
    head_transform.translation.y =
        body_transform.translation.y + body_sprite.custom_size.unwrap()[1] - 9. * SIZE
            + f32::sin(unsafe { START_TIME }) / 2. * 2.;
}

fn clock_animation(mut clock: Query<(&mut Transform, &Sprite), With<Clock>>) {
    let mut clock = clock.iter_mut();
    let (mut body_transform, body_sprite) = clock.next().unwrap();
    let (mut sec_transform, sec_sprite) = clock.next().unwrap();
    body_transform.translation.z = -0.2;
    body_transform.translation.y = 120.;

    sec_transform.translation.x =
        body_transform.translation.y + f32::cos(unsafe { START_TIME }) * 50. - 100.;

    sec_transform.rotate_z(-0.1);

    sec_transform.translation.y =
        body_transform.translation.x + f32::sin(unsafe { START_TIME }) * 50. + 100.;
}
