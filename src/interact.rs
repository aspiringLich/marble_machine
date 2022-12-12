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

#[derive(Component)]
pub struct Interactive;

/// marks a components such that a different mouse cursor will appear when hovering over it
#[derive(Component)]
pub struct InteractiveClickable;

#[derive(Component)]
pub struct RotationWidget;

#[derive(Component)]
pub struct IORotationWidget;

#[derive(Component)]
pub struct DeleteWidget;

const ROTATION_WIDGET_OFFSET: f32 = 8.0;

/// look at SelectedModules and if its modified run this function
pub fn spawn_despawn_interactive_components(
    mut commands: Commands,
    selected: Res<SelectedModules>,
    mut prev_selected: Local<u64>,
    q_children: Query<&Children>,
    q_parent: Query<&Parent>,
    mut q_module: Query<&mut ModuleType>,
    has_interactive: Query<Or<(With<Interactive>, With<InteractiveClickable>)>>,
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
            let to_be_removed: Vec<_> = q_children.iter_descendants(b).into_iter().filter(|e| has_interactive.has(*e)).collect();
            // remove all the interactive components
            to_be_removed.iter().for_each(|&e| {
                commands.entity(q_parent.entity(e).get()).remove_children(&[e]);
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
        let inputs = children.iter().with(&w_input);
        let outputs = children.iter().with(&w_output);
        let color = Color::ORANGE;

        let mut children = vec![];
        for entity in inputs.chain(outputs) {
            let child = spawn_widget!(
                Vec3::X * (ROTATION_WIDGET_OFFSET + body.offset()),
                color,
                "rotation.widget",
                RotationWidget,
                Interactive
            );
            commands.entity(entity).add_child(child);
        }
        children.push(spawn_widget!(
            Vec3::new(body.offset() - 3.0, 0.0, 0.0),
            color,
            "io_rotation.widget",
            IORotationWidget,
            Interactive
        ));
        children.push(spawn_widget!(
            Vec3::new(-body.offset(), body.offset(), 0.0),
            Color::RED,
            "delete.widget",
            DeleteWidget,
            InteractiveClickable
        ));
        commands.entity(module).push_children(&children);
    } else {
        if let Some(b) = *before {
            let to_be_removed: Vec<_> = q_children.iter_descendants(b).into_iter().filter(|e| has_interactive.has(*e)).collect();
            // remove all the interactive components
            to_be_removed.iter().for_each(|&e| {
                commands.entity(q_parent.entity(e).get()).remove_children(&[e]);
                commands.entity(e).despawn();
            });
            *before = None;
        }
    }
}

/// holds the interactive component currently selected
#[derive(Deref, DerefMut, Resource, Default, Debug)]
pub struct InteractiveSelected(Option<Entity>);

/// use the interactive widget thingies
pub fn use_widgets(
    mut commands: Commands,
    mut selected: ResMut<SelectedModules>,
    interactive_selected: Res<InteractiveSelected>,
    mut q_transform: Query<&mut Transform>,
    q_rotation: Query<&RotationWidget>,
    q_io_rotation: Query<&IORotationWidget>,
    q_delete: Query<&DeleteWidget>,
    q_parent: Query<&Parent>,
    q_children: Query<&Children>,
    w_body: Query<Entity, With<marker::ModuleBody>>,
    has_io: Query<Or<(With<marker::Input>, With<marker::Output>)>>,
    mut active: Local<bool>,
    buttons: Res<Input<MouseButton>>,
    mouse_pos: Res<CursorCoords>,
    // the previous global rotation
    mut prev_rot: Local<Option<f32>>,
    mut prev_io_rot: Local<Option<Entity>>,
) {
    // uh just trust me this works
    // i kinda forgot the logic behind it like right after i wrote it
    let changed = interactive_selected.is_changed();
    if (!changed && !*active) || !buttons.pressed(MouseButton::Left) {
        *active = false;
        *prev_rot = None;
        return;
    }
    let Some(entity) = **interactive_selected else { 
        *active = false; 
        *prev_rot = None;
        return; 
    };
    *active = true;

    // horrible terrible mess i hope to never lay eyes upon again
    if q_rotation.get(entity).is_ok() {
        let parent = q_parent.entity(q_parent.entity(entity).get()).get();
        // were the child of input which is the child of the module entity, so
        let parent_tf = q_transform.entity(parent);
        // the relative mouse pos of our cursor and the module
        let relative_pos = parent_tf.translation.truncate() - **mouse_pos;
        if relative_pos == Vec2::ZERO {
            return;
        }

        let rotation = -relative_pos.angle_between(Vec2::X) + PI;

        let mut affecting_tf = q_transform.entity_mut(q_parent.entity(entity).get());
        affecting_tf.rotation = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation);
    }
    // i should really implement this better this is just the easiest way
    else if q_io_rotation.get(entity).is_ok() {
        *prev_io_rot = Some(entity);
        // rotate the parent, but also rotate the body in the opposite direction
        let parent = q_parent.entity(entity).get();

        // the relative mouse pos of our cursor and the module
        let parent_pos = q_transform.entity_mut(parent).translation.truncate();
        let relative_pos = parent_pos - **mouse_pos;
        if relative_pos == Vec2::ZERO {
            return;
        }

        let rotation = -relative_pos.angle_between(Vec2::X) + PI;
        let diff = rotation - prev_rot.unwrap_or(rotation);
        *prev_rot = Some(rotation);

        // rotate all the io things
        for &io_entity in q_children
            .entity(parent)
            .iter()
            .filter(|&&entity| has_io.has(entity))
        {
            q_transform.entity_mut(io_entity).rotate_z(diff);
        }
        // rotate muhself
        let mut tf = q_transform.entity_mut(entity);
        let z = tf.translation.z;
        tf.rotate_around(Vec3::Z * z, Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, diff));
    } else if q_delete.get(entity).is_ok() {
        let parent = q_parent.entity(entity).get();
        commands.entity(parent).despawn_recursive();
        *active = false;
        selected.clear_selected();
    }
}
