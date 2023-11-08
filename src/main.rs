use bevy::{math::quat, prelude::*};
mod boxsys;
mod sans;
mod soul;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(sans::MainPlugin)
        .add_plugins(soul::MainPlugin)
        .add_plugins(boxsys::MainPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("spawn");
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/sans.mp3"),
        ..Default::default()
    });
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(70. * sans::SIZE, 70. * sans::SIZE)),
                ..Default::default()
            },
            texture: asset_server.load("clock-main.png"),
            ..Default::default()
        })
        .insert(sans::Clock);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30. * 2. * sans::SIZE, 4. * sans::SIZE)),
                ..Default::default()
            },

            transform: Transform {
                rotation: Quat::from_rotation_z(f32::to_radians(-130.)),
                ..Default::default()
            },
            texture: asset_server.load("clock-sec.png"),
            ..Default::default()
        })
        .insert(sans::Clock);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(26. * sans::SIZE, 17. * sans::SIZE)),
                ..Default::default()
            },
            texture: asset_server.load("leg.png"),
            ..Default::default()
        })
        .insert(sans::Sans);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(27. * sans::SIZE, 22. * sans::SIZE)),
                ..Default::default()
            },
            texture: asset_server.load("body.png"),
            ..Default::default()
        })
        .insert(sans::Sans);
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16. * sans::SIZE, 16. * sans::SIZE)),
                ..Default::default()
            },
            texture: asset_server.load("head.png"),
            ..Default::default()
        })
        .insert(sans::Sans);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20., 20.)),
                ..Default::default()
            },
            texture: asset_server.load("soul.png"),
            ..Default::default()
        })
        .insert(soul::Soul { hp: 100. });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(boxsys::Bar {
            t: boxsys::BarType::Down,
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(boxsys::Bar {
            t: boxsys::BarType::Left,
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(boxsys::Bar {
            t: boxsys::BarType::Right,
        });

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(boxsys::Bar {
            t: boxsys::BarType::Up,
        });
}
