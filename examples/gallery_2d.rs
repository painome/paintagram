// Demonstrates rendering the same gallery as gallery_3d but with a 2d camera

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

mod gallery_3d;
use gallery_3d::gallery;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Shape2dPlugin::default())
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(Msaa::Off)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gallery)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw_gallery(time: Res<Time>, mut painter: ShapePainter) {
    painter.scale(Vec3::ONE * 34.0);
    gallery(painter, time.elapsed_seconds(), 0..15);
}