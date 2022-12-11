use std::{f32::consts::PI, sync::OnceLock};

use crate::{
    atlas::AtlasDictionary,
    module::{param::*, transform_from_offset_rotate, Module},
    select::CursorCoords,
    spawn::BodyType,
    *,
};

#[derive(Component)]
pub struct Interactive;

#[derive(Component)]
pub struct RotationWidget(f32);

const ROTATION_WIDGET_OFFSET: f32 = 8.0;

/// look at SelectedModules and if its modified run this function
pub fn spawn_despawn_interactive_components(
    mut commands: Commands,
    selected: Res<SelectedModules>,
    q_children: Query<&Children>,
    q_transform: Query<&Transform>,
    mut q_module: Query<&mut ModuleType>,
    has_interactive: Query<With<Interactive>>,
    w_input: Query<Entity, With<marker::Input>>,
    w_output: Query<Entity, With<marker::Output>>,
    mut before: Local<Option<Entity>>,
) {
    // only run when SelectedModules is changed but not when its been added
    if !selected.is_changed() || selected.is_added() || selected.place {
        return;
    }

    // spawn all the interactive components
    if let Some(module) = selected.selected {
        if before.is_some() {
            // remove all the interactive components
            for entity in q_children.iter_descendants(before.unwrap()) {
                if has_interactive.has(entity) {
                    commands.entity(entity).despawn()
                }
            }
        }

        *before = Some(module);

        let body = &q_module
            .entity_mut(module)
            .get_inner()
            .spawn_instructions()
            .body;

        let inputs = q_children.entity(module).iter().with(&w_input);
        let outputs = q_children.entity(module).iter().with(&w_output);
        for entity in inputs.chain(outputs) {
            let len = q_transform
                .entity(entity)
                .clone()
                .translation
                .truncate()
                .length();

            let (texture_atlas, index) = basic::marble_small.info();
            let child = commands
                .spawn((
                    Interactive,
                    RotationWidget(len),
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite { index, ..default() },
                        texture_atlas,
                        transform: Transform::from_translation(
                            Vec3::X * (ROTATION_WIDGET_OFFSET + body.offset()),
                        ),
                        ..default()
                    },
                    Collider::ball(basic::marble_small.width() / 2.0),
                    Sensor,
                ))
                .id();
            commands.entity(entity).add_child(child);
        }
    } else {
        if let Some(b) = *before {
            // remove all the interactive components
            for entity in q_children.iter_descendants(b) {
                if has_interactive.has(entity) {
                    commands.entity(entity).despawn()
                }
            }
            *before = None;
        }
    }
}

/// holds the interactive component currently selected
#[derive(Deref, DerefMut, Resource, Default, Debug)]
pub struct InteractiveSelected(Option<Entity>);

/// use the interactive widget thingies
pub fn use_widgets(
    interactive_selected: Res<InteractiveSelected>,
    mut q_transform: Query<&mut Transform>,
    q_rotation: Query<&RotationWidget>,
    q_parent: Query<&Parent>,
    mut active: Local<bool>,
    buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<CursorCoords>,
) {
    // uh just trust me this works
    // i kinda forgot the logic behind it like right after i wrote it
    let changed = interactive_selected.is_changed();
    if (!changed && !*active) || !buttons.pressed(MouseButton::Left) {
        *active = false;
        return;
    }
    let Some(entity) = **interactive_selected else { *active = false; return; };
    *active = true;

    if let Ok(RotationWidget(offset)) = q_rotation.get(entity) {
        // were the child of input which is the child of the module entity, so
        let parent_tf = q_transform.entity(q_parent.entity(q_parent.entity(entity).get()).get());
        // the relative mouse pos of our cursor and the module
        let relative_pos = parent_tf.translation.truncate() - **mouse_pos;
        let rotation = -relative_pos.angle_between(Vec2::X) + PI;

        let mut affecting_tf = q_transform.entity_mut(q_parent.entity(entity).get());
        *affecting_tf = transform_from_offset_rotate(*offset, rotation, affecting_tf.translation.z);
    }
}
