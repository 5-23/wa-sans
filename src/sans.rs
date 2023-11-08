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
        app.add_systems(Update, timer);
        app.add_systems(Update, body_animation);
        app.add_systems(Update, clock_animation);
    }
}

fn timer(time: Res<Time>) {
    unsafe { START_TIME += time.delta_seconds() * 10. }
}

fn body_animation(mut sans: Query<(&mut Transform, &Sprite), With<Sans>>) {
    let mut sans = sans.iter_mut();
    let (mut leg_transform, leg_sprite) = sans.next().unwrap();
    let (mut body_transform, body_sprite) = sans.next().unwrap();
    let (mut head_transform, _) = sans.next().unwrap();
    leg_transform.translation.z = 0.;
    leg_transform.translation.y = 80.;

    body_transform.translation.z = 0.1;
    body_transform.rotate_z(f32::sin(unsafe { START_TIME / 1.5 }) / 800.);
    body_transform.translation.x =
        leg_transform.translation.x + f32::sin(unsafe { START_TIME } / 1.5) * 2. - SIZE / 2.;
    body_transform.translation.y = leg_transform.translation.y
        + leg_sprite.custom_size.unwrap()[1]
        + f32::sin(unsafe { START_TIME * 2. / 1.5 }) * 2.;

    head_transform.translation.z = 0.2;
    head_transform.rotate_z(f32::sin(unsafe { START_TIME / 1.5 }) / 800.);
    head_transform.translation.x =
        body_transform.translation.x + SIZE + f32::sin(unsafe { START_TIME } / 1.5) / 4. * 2.
            - SIZE / 2.;
    head_transform.translation.y =
        body_transform.translation.y + body_sprite.custom_size.unwrap()[1] - 9. * SIZE
            + f32::sin(unsafe { START_TIME * 2. / 1.5 }) / 4. * 2.;
}

fn clock_animation(mut clock: Query<&mut Transform, With<Clock>>) {
    let mut clock = clock.iter_mut();
    let mut body_transform = clock.next().unwrap();
    let mut sec_transform = clock.next().unwrap();
    body_transform.translation.z = -0.2;
    body_transform.translation.y = 180.;

    sec_transform.translation.z = -0.19;
    sec_transform.translation.y = 180.;

    if unsafe { START_TIME % 9. } as usize == 0 {
        sec_transform.rotate_z(f32::to_radians(-1.));
    }
}
