use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;


#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Clickable;

#[derive(Component)]
struct Cursor;

fn main(){
   App::new()
      .add_plugins((
         DefaultPlugins,
         RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
         RapierDebugRenderPlugin::default(),
      ))
      .add_systems(Startup, setup)
      .add_systems(Update,collide_check)
      .run();
}

fn setup(
   mut commands: Commands,
   asset_server: Res<AssetServer>,
){
   commands.spawn((Camera2dBundle::default(),MainCamera));

   commands.spawn((
      Vertex,
      SpriteBundle{
         texture: asset_server.load("circle.png"),
         transform: Transform::from_xyz(100., 100., 0.),
         ..default()
      },
      Collider::ball(30.),
   ));
}


fn collide_check(
   q_window: Query<&Window, With<PrimaryWindow>>,
   q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
   rapier_context: Res<RapierContext>,
){
   let window = q_window.single();
   let (camera, camera_transform) = q_camera.single();

   if let Some(world_position) = window.cursor_position()
      .and_then(|cursor_position| camera.viewport_to_world_2d(camera_transform, cursor_position)){

         let filter = QueryFilter::default();
         rapier_context.intersections_with_point(world_position, filter, |entity|{
            println!("intersection");
            true
         });
   }
   
}

fn mouse_collider(
   q_window: Query<&Window, With<PrimaryWindow>>,
   q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
   mut q_circle: Query<&mut Transform, With<Cursor>>,
){
   let (camera, camera_transform) = q_camera.single();
   let window = q_window.single();

   if let Some(world_position) = window.cursor_position()
      .and_then(|cursor_position| camera.viewport_to_world_2d(camera_transform, cursor_position))
      
    {
      println!("{:?}",world_position);

      for mut circle_transform in q_circle.iter_mut(){
         circle_transform.translation.x = world_position.x;
         circle_transform.translation.y = world_position.y;
      }
   }

}