use bevy::{pbr::AmbientLight, prelude::*, render::mesh::skinning::SkinnedMesh};
use web_sys;

pub struct WindowResizePlugin;

impl Plugin for WindowResizePlugin {
    #[cfg(target_arch = "wasm32")]
    fn build(&self, app: &mut App) {
        app.add_system(handle_browser_resize);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn build(&self, _app: &mut App) {}
}

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(
    mut primary_query: bevy::ecs::system::Query<
        &mut bevy::window::Window,
        bevy::ecs::query::With<bevy::window::PrimaryWindow>,
    >,
) {
    for mut window in &mut primary_query {
        let wasm_window = web_sys::window().unwrap();
        let (target_width, target_height) = (
            wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
            wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
        );
        if window.resolution.width() != target_width || window.resolution.height() != target_height
        {
            window.resolution.set(target_width, target_height);
        }
    }
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WindowResizePlugin)
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .insert_resource(AmbientLight {
            brightness: 1.0,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, joint_animation)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/broccoli/scene.gltf#Scene0"),
        transform: Transform::from_xyz(-0.5, -0.7, 1.2),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/tomato/scene.gltf#Scene0"),
        transform: Transform::from_xyz(0.5, -0.7, 1.2)
            .with_scale(Vec3 {
                x: 0.6,
                y: 0.6,
                z: 0.6,
            })
            .with_rotation(Quat::from_rotation_x(2.0)),
        ..default()
    });
}
fn joint_animation(
    time: Res<Time>,
    _parent_query: Query<&Parent, With<SkinnedMesh>>,
    _children_query: Query<&Children>,
    mut transform_query: Query<&mut Transform>,
) {
    for mut transform in transform_query.iter_mut() {
        /*
        transform.scale = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0 + FRAC_PI_8 * time.elapsed_seconds().sin() * 0.1,
        };
        */
        let elapsed = time.elapsed().as_millis();
        let millis_y = elapsed % 1000;
        let millis_fun = elapsed % 8000;
        if millis_fun > 7500 {
            transform.rotation.y = (millis_fun as f32 - 7750.0) * 0.001;
        } else if millis_y < 100 {
            transform.rotation.y = ((millis_y as f32) * 0.0157).sin() * 0.02;
        } else if 500 <= millis_y && millis_y <= 600 {
            transform.rotation.y = (((millis_y - 500) as f32) * 0.0157).sin() * -0.02;
        }

        //println!("xyz: {:?}", transform.rotation.xyz());
        let millis_x = elapsed % 4000;
        if millis_x > 2000 && millis_x < 2100 {
            transform.rotation.x = -0.16 + (((millis_x - 2000) as f32) * 0.0157).sin() * 0.02;
        } else if millis_x < 100 {
            transform.rotation.x = -0.16 + ((millis_x as f32) * 0.0157).sin() * -0.02;
        }
    }
}
