use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};
use bevy_rapier2d::{geometry::Collider, pipeline::QueryFilter, plugin::RapierContext};
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
      .add_systems(Update, collide_check2)
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

fn collide_check2(
   mut q_collider : Query<&mut Collider>,
   q_window : Query<&Window, With<PrimaryWindow>>,
   q_camera : Query<(&Camera,&GlobalTransform)>,
){
   let window = q_window.single();
   let (camera, camera_transform) = q_camera.single();

   for mut collider in q_collider.iter_mut() {
      if collider.contains_local_point(Vec2::new(29., 29.)) {
         println!("contain");
      }
   }
}



fn collide_check(
   q_window : Query<&Window, With<PrimaryWindow>>,
   q_camera : Query<(&Camera,&GlobalTransform)>,
   rapier_context : Res<RapierContext>,
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
         println!("COLLIDE ENTITY : {:?}",entity);
         true
      });
   }
}