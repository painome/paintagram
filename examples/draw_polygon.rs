use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::RoundedPolygon};

const PI:f32 = 3.1415;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands
){
    let points = [
        Vec2::new(0.,1.),
        Vec2::new((PI/10.).cos(),(PI/10.).sin()),
        Vec2::new((PI/5.).sin(),-(PI/5.).cos()),
        Vec2::new(-(PI/5.).sin(),-(PI/5.).cos()),
        Vec2::new(-(PI/10.).cos(),(PI/10.).sin()),
    ].map(|v| v * 300.);


    shapes::

    spawn_polygon(&mut commands);
}


fn spawn_polygon(commands:&mut Commands, let ){
    commands.spawn(Camera2dBundle::default());

    let points = [
        Vec2::new(0.,1.),
        Vec2::new((PI/10.).cos(),(PI/10.).sin()),
        Vec2::new((PI/5.).sin(),-(PI/5.).cos()),
        Vec2::new(-(PI/5.).sin(),-(PI/5.).cos()),
        Vec2::new(-(PI/10.).cos(),(PI/10.).sin()),
    ].map(|v| v * 300.);

    let shape = shapes::RoundedPolygon{
        points: points.into_iter().collect(),
        radius: 0.,
        closed: true,
    };

    let entity=commands.spawn((
        ShapeBundle{
            path: GeometryBuilder::build_as(&shape),
            ..default()
        },
        Fill::color(Color::ORANGE_RED),
    )).id();

}