use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;


#[derive(Component,Default,Clone, Copy)]
pub struct CursorInteraction{
   over : bool,
   pre_over : bool,
   enter: bool,
   leave: bool,
   pressed: bool,
   just_pressed: bool,
   just_released: bool,
}

#[allow(dead_code)]
impl CursorInteraction {
   pub fn over(self)->bool{self.over}
   pub fn pre_over(self)->bool{self.pre_over}
   pub fn enter(self)->bool{self.enter}
   pub fn leave(self)->bool{self.leave}
   pub fn pressed(self)->bool{self.pressed}
   pub fn just_pressed(self)->bool{self.just_pressed}
   pub fn just_released(self)->bool{self.just_released}
}

pub struct CursorInteractionPlugin;
impl Plugin for CursorInteractionPlugin{
   fn build(&self, app: &mut App) {
       app.add_systems(Update, check_cursor_events);
   }
}

fn check_cursor_events(
   mut q_interaction_entities : Query<(Entity, &mut CursorInteraction)>,
   q_window : Query<&Window,With<PrimaryWindow>>,
   q_camera : Query<(&Camera,&GlobalTransform)>,
   rapier_context :Res<RapierContext>,
   buttons: Res<ButtonInput<MouseButton>>,
){
   // カーソル座標の取得
   let window = q_window.single();
   let (camera, camera_transform) = q_camera.single();
   let op_cursor_position = window.cursor_position()
      .and_then(|viewport_position|{camera.viewport_to_world_2d(camera_transform, viewport_position)});
 
   // カーソルがエンティティ内にあるかどうかの判定
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
      //カーソルがコライダーに入った瞬間と出た瞬間の判定
      cursor_interaction.enter = false;
      cursor_interaction.leave = false;
      if !cursor_interaction.pre_over && cursor_interaction.over{ cursor_interaction.enter = true;}
      else if cursor_interaction.pre_over && !cursor_interaction.over{ cursor_interaction.leave = true;}

      //カーソルがコライダーを押したかどうかの判定
      cursor_interaction.pressed = false;
      cursor_interaction.just_pressed = false;
      cursor_interaction.just_released = false;
      if cursor_interaction.over && buttons.pressed(MouseButton::Left) {cursor_interaction.pressed = true;}
      if cursor_interaction.over && buttons.just_pressed(MouseButton::Left) {cursor_interaction.just_pressed = true;}
      if cursor_interaction.over && buttons.just_released(MouseButton::Left) {cursor_interaction.just_released = true;}
   }

}