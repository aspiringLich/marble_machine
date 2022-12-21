use std::{
    collections::hash_map::DefaultHasher,
    f32::consts::PI,
    hash::{Hash, Hasher},
};

use crate::{
    atlas::AtlasDictionary,
    module::{param::*, Module},
    select::CursorCoords,
    *,
};

#[derive(Component, Debug)]
pub enum Interactive {
    Rotation,
    IORotation,
    Delete,
}

#[derive(Component, Debug)]
pub struct InteractiveRotation {
    pub input_rot: Vec<f32>,
    pub output_rot: Vec<f32>,
    pub rot: f32,
}

impl InteractiveRotation {
    pub fn from<'a, T: Iterator<Item = &'a Transform>>(inputs: T, outputs: T) -> Self {
        Self {
            input_rot: inputs
                .map(|t| t.rotation.to_euler(EulerRot::XYZ).2)
                .collect(),
            output_rot: outputs
                .map(|t| t.rotation.to_euler(EulerRot::XYZ).2)
                .collect(),
            rot: 0.0,
        }
    }
}

const ROTATION_WIDGET_OFFSET: f32 = 8.0;

/// look at SelectedModules and if its modified run this function
pub fn spawn_despawn_interactive_components(
    mut commands: Commands,
    selected: Res<SelectedModules>,
    mut prev_selected: Local<u64>,
    q_children: Query<&Children>,
    q_parent: Query<&Parent>,
    q_transform: Query<&Transform>,
    mut q_module: Query<&mut ModuleType>,
    has_interactive: Query<With<Interactive>>,
    w_input: Query<Entity, With<marker::Input>>,
    w_output: Query<Entity, With<marker::Output>>,
    mut before: Local<Option<Entity>>,
) {
    // make sure its not the exact same value
    let mut hasher = DefaultHasher::new();
    selected.hash(&mut hasher);
    let hash = hasher.finish();
    if hash == *prev_selected {
        return;
    }
    *prev_selected = hash;

    // only run when SelectedModules is changed but not when its been added
    if !selected.is_changed() || selected.is_added() || selected.place {
        return;
    }

    // spawn all the interactive components
    if let Some(module) = selected.selected {
        if let Some(b) = *before {
            let to_be_removed: Vec<_> = q_children
                .iter_descendants(b)
                .into_iter()
                .filter(|e| has_interactive.has(*e))
                .collect();
            // remove all the interactive components
            to_be_removed.iter().for_each(|&e| {
                commands
                    .entity(q_parent.entity(e).get())
                    .remove_children(&[e]);
                commands.entity(e).despawn();
            });
            *before = None;
        }

        *before = Some(module);

        let body = &q_module
            .entity_mut(module)
            .get_inner()
            .spawn_instructions()
            .body;

        macro spawn_widget($translation:expr, $color:expr, $name:literal $($tail:tt)*) {
            commands
                .spawn((
                    SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: basic::marble_small.info().1,
                            color: $color,
                            ..default()
                        },
                        texture_atlas: basic::marble_small.info().0,
                        transform: Transform::from_translation($translation + Vec3::Z * 2.0).with_scale(Vec3::ONE * 1.01),
                        ..default()
                    },
                    Collider::ball(basic::marble_small.width() / 2.0),
                    Sensor,
                    Name::new($name)
                    $($tail)*
                ))
                .id()
        }

        // rotation widgets
        let children = q_children.entity(module);
        let inputs = children.iter().with_collect(&w_input);
        let outputs = children.iter().with_collect(&w_output);
        let color = Color::ORANGE;

        let mut children = vec![];

        for entity in inputs.iter().chain(outputs.iter()) {
            let child = spawn_widget!(
                Vec3::X * (ROTATION_WIDGET_OFFSET + body.offset()),
                color,
                "rotation.widget",
                Interactive::Rotation
            );
            commands.entity(*entity).add_child(child);
        }
        children.push(spawn_widget!(
            Vec3::new(body.offset() - 3.0, 0.0, 0.0),
            color,
            "io_rotation.widget",
            Interactive::IORotation
        ));
        children.push(spawn_widget!(
            Vec3::new(-body.offset(), body.offset(), 0.0),
            Color::RED,
            "delete.widget",
            Interactive::Delete
        ));

        commands.entity(module).push_children(&children);

        let get_transform = |e: &Entity| q_transform.get(*e).ok();
        commands.entity(module).insert(InteractiveRotation::from(
            inputs.iter().filter_map(get_transform),
            outputs.iter().filter_map(get_transform),
        ));
    } else {
        if let Some(b) = *before {
            if commands.get_entity(b).is_none() {
                return;
            }
            let to_be_removed: Vec<_> = q_children
                .iter_descendants(b)
                .into_iter()
                .filter(|e| has_interactive.has(*e))
                .collect();
            // remove all the interactive components
            to_be_removed.iter().for_each(|&e| {
                commands
                    .entity(q_parent.entity(e).get())
                    .remove_children(&[e]);
                commands.entity(e).despawn();
            });
            commands.entity(b).remove::<InteractiveRotation>();
            *before = None;
        }
    }
}

/// holds the interactive component currently selected
#[derive(Deref, DerefMut, Resource, Default, Debug)]
pub struct InteractiveSelected(Option<Entity>);

/// TODO: implement this better, somehow i dont think this has bugs but you cant be suuure
/// use the interactive widget thingies
pub fn use_widgets(
    mut commands: Commands,
    mut selected: ResMut<SelectedModules>,
    interactive_selected: Res<InteractiveSelected>,
    q_transform: Query<&Transform>,
    q_parent: Query<&Parent>,
    q_interactive: Query<&Interactive>,
    mut q_interactive_rot: Query<&mut InteractiveRotation>,
    q_in: Query<&marker::Input>,
    q_out: Query<&marker::Output>,
    mut active: Local<bool>,
    buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<CursorCoords>,
    mut diff: Local<Option<f32>>,
    keyboard: Res<Input<KeyCode>>,
) {
    // uh just trust me this works
    // i kinda forgot the logic behind it like right after i wrote it
    let changed = interactive_selected.is_changed();
    if (!changed && !*active) || !buttons.pressed(MouseButton::Left) {
        *active = false;
        *diff = None;
        return;
    }
    let Some(entity) = **interactive_selected else { 
        *active = false; 
        *diff = None;
        return; 
    };
    *active = true;
    

    let rel_angle = |t: &Transform| {
        let relative_pos = t.translation.truncate() - **mouse_pos;
        if relative_pos == Vec2::ZERO {
            return None;
        }
        Some(-relative_pos.angle_between(Vec2::X) + PI)
    };
    
    let step = PI / 12.0;
    
    use Interactive::*;
    match q_interactive.entity(entity) {
        Rotation => {
            let io_port = q_parent.entity(entity).get();
            let module = q_parent.entity(io_port).get();
            
            let mut i_rot = q_interactive_rot.entity_mut(module);
            let rot = i_rot.rot;
            // dbg!(&i_rot);
            let io_rot = if let Ok(marker::Input(n)) = q_in.get(io_port) {
                &mut i_rot.input_rot[*n]
            } else if let Ok(marker::Output(n)) = q_out.get(io_port) {
                &mut i_rot.output_rot[*n]
            } else {
                error!("Expected Input or Output component on module entity; Did not find either.");
                return;
            };
            
            let root = q_transform.entity(module);
            let Some(angle) = rel_angle(root) else { return; };
            let Some(diff) = *diff else {
                *diff = Some(angle - *io_rot - rot);
                return;
            };
            
            if keyboard.pressed(KeyCode::LShift) {
                let rounded = ((angle - diff) / step).round() * step;
                *io_rot = rounded - rot;
            } else {
                *io_rot = angle - rot - diff;
            }
        }
        IORotation => {
            let module = q_parent.entity(entity).get();
            let mut i_rot = q_interactive_rot.entity_mut(module);
            
            let root = q_transform.entity(module);
            let Some(angle) = rel_angle(root) else { return; };
            let Some(diff) = *diff else {
                *diff = Some(angle - i_rot.rot);
                return;
            };
            
            if keyboard.pressed(KeyCode::LShift) {
                i_rot.rot = ((angle - diff) / step).round() * step;
            } else {
                i_rot.rot = angle - diff;
            }
        },
        Delete => {
            let parent = q_parent.entity(entity).get();
            commands.entity(parent).despawn_recursive();
            *active = false;
            selected.clear_selected();
        },
    }
}

pub fn do_interactive_rotation(
    w_interactive_rot: Query<Entity, Changed<InteractiveRotation>>,
    q_interactive_rot: Query<&InteractiveRotation>,
    q_interactive: Query<&Interactive>,
    mut q_transform: Query<&mut Transform>,
    w_i: Query<Entity, With<marker::Input>>,
    w_o: Query<Entity, With<marker::Output>>,
    q_children: Query<&Children>,
) {
    let Ok(entity) = w_interactive_rot.get_single() else { return; }; 
    let Ok(i_rot) = q_interactive_rot.get(entity) else { return; };
    let children = q_children.entity(entity);
    
    let interactive = children.iter().filter_map(|e| q_interactive.get(*e).ok().map(|i| (e, i)));
    for (e, i) in interactive {
        use Interactive::*;
        match i {
            IORotation => {
                let mut transform = q_transform.entity_mut(*e);
                let z = transform.translation.z;
                let rot = transform.rotation.to_euler(EulerRot::XYZ).2;
                
                transform.rotate_around(Vec2::ZERO.extend(z), Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, i_rot.rot - rot));
                break;
            },
            _ => continue,
        }
    }

    for (i, input) in children.iter().with(&w_i).enumerate() {
        let mut transform = q_transform.entity_mut(input);
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, i_rot.input_rot[i] + i_rot.rot);
    }
    for (i, output) in children.iter().with(&w_o).enumerate() {
        let mut transform = q_transform.entity_mut(output);
        transform.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, i_rot.output_rot[i] + i_rot.rot);
    }
}
