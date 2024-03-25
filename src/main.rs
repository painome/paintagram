//! A minimal 2d example.

mod cursor_interaction;

use std::f32::consts::E;

use bevy::{input::mouse::MouseButtonInput, prelude::*, scene::ron::de, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, transform, window::PrimaryWindow};
use bevy_prototype_lyon::{draw::{Fill, Stroke}, entity::{Path, ShapeBundle}, geometry::GeometryBuilder, plugin::ShapePlugin, shapes};
use bevy_rapier2d::{geometry::Collider, pipeline::QueryFilter, plugin::{NoUserData, RapierContext, RapierPhysicsPlugin}, rapier::dynamics::RigidBody, render::RapierDebugRenderPlugin};
use cursor_interaction::*;

const PI:f32 = 3.141592;

#[derive(Component)]
struct Paintagram;

#[derive(Component)]
struct Vertex;

#[derive(Component)]
struct Polyline{
    points:Vec<Vec2>,
}

#[derive(Event)]
struct UpdatedVertices{
    added: Vec<Vec2>,
    deleted: Vec<Vec2>,
}


#[derive(Component,Default)]
struct Palette{
    current_color:Option<Color>
}
#[derive(Component,Default)]
struct Paint{
    color:Color,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            RapierDebugRenderPlugin::default(),
            CursorInteractionPlugin,
        ))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update,(select_paint))
        .add_systems(Update,update_polyline)
        .add_event::<UpdatedVertices>()
        .run();
}

/// Set up a simple 2D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // カメラ生成
    commands.spawn(Camera2dBundle::default());


    commands.spawn((
        Vertex,
        MaterialMesh2dBundle {
            transform : Transform::from_xyz(-100.0, 0., 0.),
            mesh: Mesh2dHandle(meshes.add(Circle{radius:10.})),
            material: materials.add(Color::BLUE),
            ..default()
        },
        Collider::ball(10.),
        CursorInteraction::default(),
    ));

    // パレット作成
    let mut palette = commands.spawn((
        Palette::default(),
        
        SpriteBundle{
            transform: Transform::from_xyz(100., -100., 0.),
            ..default()
        },
        
    ));
    // 絵の具作成
    let colors = vec![
        Color::RED,
        Color::YELLOW,
        Color::BLUE,
        Color::GREEN
    ];
    palette.with_children(|palette|{
        for (index,color) in colors.iter().enumerate() {
            let shape = shapes::Circle{
                radius : 10.,
                center : Vec2::new(index as f32 * 100., 0.),
            };
            palette.spawn((
                Paint{color: *color},
                MaterialMesh2dBundle{
                    transform : Transform::from_xyz(index as f32 * 100., 0., 0.),
                    mesh: Mesh2dHandle(meshes.add(Circle{radius:10.})),
                    material : materials.add(*color),
                    ..default()
                }, 
                
                /*
                ShapeBundle{
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(*color),
                */
                CursorInteraction::default(),
            ))
            .insert(Collider::ball(10.))
            ;
        }
        
    });

    // ペインタグラム生成
    let mut paintagram = commands.spawn((
        Paintagram,
        SpriteBundle{
            transform: Transform::from_xyz(-100., 10., 0.).with_scale(Vec3::new(1.,1.,1.)),
            ..default()
        },
    ));
    
    // 頂点生成
    let initial_vertices = [
        Vec2::new(0.,1.),
        Vec2::new((PI/10.).cos(),(PI/10.).sin()),
        Vec2::new((PI/5.).sin(),-(PI/5.).cos()),
        Vec2::new(-(PI/5.).sin(),-(PI/5.).cos()),
        Vec2::new(-(PI/10.).cos(),(PI/10.).sin()),
    ].map(|v| v * 300.);

    for vertex in initial_vertices.iter() {
        paintagram.with_children(|parent|{
            parent.spawn((
                Vertex,
                MaterialMesh2dBundle {
                    transform : Transform::from_xyz(vertex.x, vertex.y, 0.),
                    mesh: Mesh2dHandle(meshes.add(Circle{radius:10.})),
                    material: materials.add(Color::GOLD),
                    ..default()
                },
                Collider::ball(10.),
                CursorInteraction::default(),
            ));
        });
    }
    
    // ポリライン生成
    let points = Vec::<Vec2>::new();
    let shape = shapes::Polygon{
        points: points.clone(),
        closed: false,
    };
    paintagram.with_children(|parent|{
        parent.spawn((
            Polyline{
                points,
            },
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Stroke::color(Color::RED),
        ));
    });
    
}


fn select_paint(
    mut q_palette : Query<&mut Palette>,
    mut q_paints : Query<(&CursorInteraction, &mut Transform, &Paint)>,
){
    let mut palette = q_palette.single_mut();
    for (cursor,mut transform, paint) in q_paints.iter_mut() {
        if cursor.over() {
            transform.scale = Vec3::new(1.5,1.5,1.);
        }else {
            transform.scale = Vec3::ONE;
        }

        if cursor.just_pressed(){
            palette.current_color = Some(paint.color);
            println!("current color is {:?}",palette.current_color);
        }
    }
}


fn select_vertex(
    mut q_palette: Query<&mut Palette>,
    //mut q_vertices: Query<(&CursorInteraction, &mut Transform, &mut Material),With<Vertex>>,
){
    let mut palette = q_palette.single_mut();

    //for (cursor, mut transform,)

}



fn update_polyline(
    q_vertices : Query<(&CursorInteraction,&Transform),With<Vertex>>,
    mut q_polyline : Query<(&mut Polyline, &mut Path)>,
){
    let (mut polyline, mut path) = q_polyline.single_mut();
    
    for (cursor_interaction, vertex) in q_vertices.iter() {  
        if cursor_interaction.just_pressed()  {
            polyline.points.push(vertex.translation.truncate());
            let shape = shapes::Polygon{
                points: polyline.points.clone(),
                closed: true,
            };
            *path = GeometryBuilder::build_as(&shape);

        }
    }
}