use std::f32::consts::PI;

use bevy_prototype_lyon::shapes::Circle;

use crate::{
    misc::marker::{Module, ModuleBody},
    query::QueryQuerySimple,
    *, ui::spawning::recreate_module,
};

use super::{
    interact::{Interactive, InteractiveSelected},
    select::CursorCoords,
};

#[derive(Default, Deref, DerefMut, Resource)]
pub struct HoveredEntities(Vec<Entity>);

pub fn get_hovered_entities(
    mouse_pos: Res<CursorCoords>,
    rapier_context: Res<RapierContext>,
    mut hovered_entities: ResMut<HoveredEntities>,
) {
    hovered_entities.clear();
    rapier_context.intersections_with_point(**mouse_pos, default(), |entity| {
        hovered_entities.push(entity);
        true
    });
}

#[derive(Deref, DerefMut)]
pub struct HoverEntity(Entity);

impl Default for HoverEntity {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

pub fn draw_selection_on_hovered(
    mut commands: Commands,
    selected: Res<SelectedModules>,
    interactive_selected: Res<InteractiveSelected>,
    hovered_entities: Res<HoveredEntities>,
    q_sprite: Query<(Entity, &TextureAtlasSprite, &Handle<TextureAtlas>), With<Interactive>>,
    time: Res<Time>,
    mut hover_entity: Local<Option<Entity>>,
    mut prev_entity: Local<Option<Entity>>,
    mut hover_sprite: Local<Option<Entity>>,
    atlases: Res<Assets<TextureAtlas>>,
) {
    *prev_entity = *hover_entity;
    let mut itr = vec![];

    if let Some(e) = **interactive_selected {
        itr.push(e);
    }
    itr.extend(hovered_entities.iter());

    let size;

    macro despawn() {
        if let Some(entity) = *hover_sprite {
            *hover_sprite = None;
            if let Some(e) = *hover_entity && let Some(mut commands) = commands.get_entity(e) {
                // dbg!(q_name.get(entity));
                commands.remove_children(&[entity]);
            }
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.despawn();
            }
            // info!("despawned");
        }
    }

    // get the size of the hovered entity
    let mut iter = itr
        .iter()
        .chain(hovered_entities.iter())
        .filter_map(|e| q_sprite.get(*e).ok());

    if let Some((entity, sprite, atlas)) = iter.next() {
        *hover_entity = Some(entity);
        let bounds = atlases.get(atlas).unwrap().textures[sprite.index].size();
        size = f32::max(bounds.x, bounds.y);
    } else {
        despawn!();
        *hover_entity = None;
        return;
    }

    let rot_per_sec = 0.5;
    let rot = PI * 2.0 * rot_per_sec * time.elapsed_seconds();
    let quat = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rot);

    // the hovered entity is the same as last time
    if *hover_entity == *prev_entity {
        // something
    } else {
        // if theyre not equal,
        // despawn the old hover sprite
        despawn!();
        // info!("despawned");
        // spawn a new hover sprite
        if let Some(entity) = *hover_entity {
            // info!("respawned");
            let mut stroke = StrokeMode::new(Color::rgba(1.0, 1.0, 1.0, 0.6), 0.5);
            stroke.options.tolerance = 0.01;

            let mut transform = Transform::default();
            transform.rotation = quat;
            transform.translation.z = ZOrder::HoverIndicator.f32();

            let e = commands
                .spawn(
                    GeometryBuilder::new()
                        .add(
                            &(Circle {
                                center: Vec2::ZERO,
                                radius: size / 2.0,
                                ..Default::default()
                            }),
                        )
                        .build(DrawMode::Stroke(stroke), transform),
                )
                .name("hover.sprite")
                .id();

            *hover_sprite = Some(e);
            commands.entity(entity).add_child(e);
        }
    }
}
