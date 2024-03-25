use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

fn main(){
   App::new()
      .add_plugins(DefaultPlugins)
      .add_systems(Startup, setup)
      .run();
}

fn setup(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,
){
   // カメラを生成
   commands.spawn(Camera2dBundle::default());

   // 黄色の円を生成
   commands.spawn(MaterialMesh2dBundle{
      mesh : Mesh2dHandle(meshes.add(Circle{radius: 50.})),
      material : materials.add(Color::GOLD),
      ..default()
   });
}