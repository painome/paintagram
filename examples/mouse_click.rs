use bevy::{prelude::*, window::PrimaryWindow};

fn main(){
   let app= App::new()
      .add_plugins(DefaultPlugins)
      .add_systems(Startup,setup)
      .add_systems(Update,cursor_position)
      .run();
}

fn setup(mut commands:Commands){
   commands.spawn(Camera2dBundle::default());
}


fn cursor_position(
   window_query: Query<&Window, With<PrimaryWindow>>
){
   //ウィンドウ上の座標を返す
   if let Some(position) = window_query.get_single().unwrap().cursor_position(){
      println!("{:?}",position);
   }
}

