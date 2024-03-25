use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};
use bevy_rapier2d::geometry::Collider;
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

fn main(){
   App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins((
         RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
         RapierDebugRenderPlugin::default(),
      ))
      .add_systems(Startup, setup)
      .run();
}

fn setup(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,
){
   //カメラを生成
   commands.spawn(Camera2dBundle::default());

   //黄色の円を生成
   commands.spawn(MaterialMesh2dBundle{
      mesh : Mesh2dHandle(meshes.add(Circle{radius: 50.})),
      material : materials.add(Color::GOLD),
      ..default()
   })
   .insert(Collider::ball(40.))
   ;
}