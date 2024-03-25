use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::plugin::RapierContext;

fn main(){
   App::new()
      .add_plugins(DefaultPlugins)
      .add_systems(Startup, setup)
      .add_systems(Update, print_mouse_position)
      .run();
}

fn setup(mut commands : Commands){
   commands.spawn(Camera2dBundle::default());
}

fn print_mouse_position(
   q_window : Query<&Window, With<PrimaryWindow>>,
   q_camera : Query<(&Camera,&GlobalTransform)>,
   rapier_context : Res<RapierContext>,
){
   let window = q_window.single();
   let (camera, camera_transform) = q_camera.single();
   let op_world_cursor_position = window.cursor_position()
      .and_then(|viewport_position|{
         camera.viewport_to_world_2d(camera_transform, viewport_position)
      });

   if let Some(world_cursor_position) = op_world_cursor_position{
      println!("{}",world_cursor_position);
   }
}