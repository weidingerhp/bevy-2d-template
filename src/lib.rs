use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use bevy_kira_audio::{AudioPlugin};

mod audio_channels;
pub use audio_channels::AudioChannels;

#[wasm_bindgen]
pub fn run() {
    let title = format!("{{window-title}} V{}", env!("CARGO_PKG_VERSION"));
    let mut app = App::build();
    app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.3)));
    app.insert_resource(WindowDescriptor {
        title: title,
        width: 640.,
        height: 480., 
        ..Default::default()
    });
    app.insert_resource(AudioChannels::new());
    app.add_plugins(DefaultPlugins);
    app.add_plugin(AudioPlugin);
 
    app.add_startup_system(setup.system());

    // could add more systems and stages here - we will just add a background ....
    app.add_startup_stage("background", SystemStage::single(spawn_background.system()));

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    
    app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_background(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>, 
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(asset_server.load("background.png").into()), 
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            scale: Vec3::new(0.7, 0.7, 1.), // scale and bring to y-pos 1 (meaning first after background) 
            ..Default::default()
        },
        ..Default::default()
    });
}
