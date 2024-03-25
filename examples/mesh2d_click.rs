use bevy::{input::ButtonState, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};
use bevy_rapier2d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier2d::render::RapierDebugRenderPlugin;

struct CirclePressedEvent{
   
}



fn main(){
   App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins((
         RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
         RapierDebugRenderPlugin::default(),
      ))
      .add_systems(Startup, setup)
      .add_systems(Update, collide_check)
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

fn collide_check(
   q_window : Query<&Window, With<PrimaryWindow>>,
   q_camera : Query<(&Camera,&GlobalTransform)>,
   rapier_context : Res<RapierContext>,
   buttons: Res<ButtonInput<MouseButton>>,
){

   let window = q_window.single();
   let (camera, camera_transform) = q_camera.single();

   //マウスのワールド座標取得(Option型)
   let q_mouse_position = window.cursor_position()
      .and_then(|window_mouse_position|{
         camera.viewport_to_world_2d(camera_transform,window_mouse_position)
      });

   //コライダーとマウスの衝突判定
   if let Some(mouse_position) = q_mouse_position {
      let filter = QueryFilter::default();
      rapier_context.intersections_with_point(mouse_position, filter, |entity|{
         if buttons.pressed(MouseButton::Left) {
            println!("COLLIDE ENTITY : {:?}",entity);
         }
         if buttons.released(MouseButton::Left){
            println!();
         }
         true

         MouseMotion::
      });
   }
}

fn circle_pressed(){

}