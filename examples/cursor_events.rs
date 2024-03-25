use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;


#[derive(Component,Default)]
struct CursorInteraction{
   over : bool,
   pre_over : bool,
   enter: bool,
   leave: bool,
   pressed: bool,
   just_pressed: bool,
   just_released: bool,
}

fn main(){
   App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins((
         RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
         RapierDebugRenderPlugin::default(),
      ))
      .add_systems(Startup, setup)
      .add_systems(Update,(check_cursor_events,test))
      .run();
}

fn setup(
   mut commands: Commands,
   mut meshes : ResMut<Assets<Mesh>>,
   mut materials : ResMut<Assets<ColorMaterial>>,
){
   commands.spawn(Camera2dBundle::default());

   for i in -3..=3{
      commands.spawn(MaterialMesh2dBundle{
         mesh: Mesh2dHandle(meshes.add(Circle{radius: 50.})),
         material: materials.add(Color::GOLD),
         transform: Transform::from_xyz((i as f32)*150., 0., 0.),
         ..default()
      })
      .insert(Collider::ball(40.))
      .insert(CursorInteraction::default())
      ;

   }
}

fn check_cursor_events(
   mut q_interaction_entities : Query<(Entity, &mut CursorInteraction)>,
   q_window : Query<&Window,With<PrimaryWindow>>,
   q_camera : Query<(&Camera,&GlobalTransform)>,
   rapier_context :Res<RapierContext>,
   buttons: Res<ButtonInput<MouseButton>>,
){
   let window = q_window.single();
   let (camera, camera_transform) = q_camera.single();

   let op_cursor_position = window.cursor_position()
      .and_then(|viewport_position|{camera.viewport_to_world_2d(camera_transform, viewport_position)});
 
   // カーソルがエンティティ内にあるかどうか
   for (_, mut cursor_interaction) in q_interaction_entities.iter_mut(){
      cursor_interaction.pre_over = cursor_interaction.over;
      cursor_interaction.over = false;
   }
   if let Some(cursor_position) = op_cursor_position {
      let filter = QueryFilter::default();

      rapier_context.intersections_with_point(cursor_position,filter,|entity|{
         for (interaction_entity, mut cursor_interaction) in q_interaction_entities.iter_mut(){
            if interaction_entity == entity {
               cursor_interaction.over = true;
            }
         }
         true
      });
   }
   
   for (_, mut cursor_interaction) in q_interaction_entities.iter_mut(){
      //マウスがコライダーに入った瞬間と出た瞬間の判定
      cursor_interaction.enter = false;
      cursor_interaction.leave = false;
      if !cursor_interaction.pre_over && cursor_interaction.over{ cursor_interaction.enter = true;}
      else if cursor_interaction.pre_over && !cursor_interaction.over{ cursor_interaction.leave = true;}

      //マウスがコライダーをクリックしているかどうかの判定
      cursor_interaction.pressed = false;
      cursor_interaction.just_pressed = false;
      cursor_interaction.just_released = false;
      if cursor_interaction.over && buttons.pressed(MouseButton::Left) {cursor_interaction.pressed = true;}
      if cursor_interaction.over && buttons.just_pressed(MouseButton::Left) {cursor_interaction.just_pressed = true;}
      if cursor_interaction.over && buttons.just_released(MouseButton::Left) {cursor_interaction.just_released = true;}
   }

}


fn test(
   q_interaction_entities : Query<(Entity, &CursorInteraction)>
){
   for (interaction_entity, cursor_interaction) in q_interaction_entities.iter(){
      if cursor_interaction.just_pressed {
         println!("{:?}:just_pressed",interaction_entity);
      }
      if cursor_interaction.just_released {
         println!("{:?}:just_released",interaction_entity);
      }
      if cursor_interaction.enter {
         println!("{:?}:just_enter",interaction_entity);
      }
      if cursor_interaction.leave {
         println!("{:?}:just_leave",interaction_entity);
      }
   }
}