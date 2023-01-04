use crate::{
    atlas::AtlasDictionary,
    module::param::{QueryQueryIter, QueryQuerySimple},
    spawn::{BodyType, CommandsSpawn},
    *,
};

const TRACER_N: usize = 20;
#[derive(Deref, DerefMut, Resource)]
pub struct TracerEntities([Entity; TRACER_N]);

const TRACER_COLOR: Color = Color::Rgba {
    red: 0.25,
    green: 0.25,
    blue: 0.25,
    alpha: 0.5,
};

impl FromWorld for TracerEntities {
    fn from_world(world: &mut World) -> Self {
        let mut e: TracerEntities = unsafe { std::mem::zeroed() };

        for i in 0..TRACER_N - 1 {
            let (texture_atlas, index) = basic::tracer_tick.info();
            e[i] = world
                .spawn((
                    SpriteSheetBundle {
                        texture_atlas,
                        sprite: TextureAtlasSprite {
                            index,
                            color: TRACER_COLOR,
                            ..default()
                        },
                        visibility: Visibility::INVISIBLE,
                        ..default()
                    },
                    Name::new("tracer.sprite"),
                ))
                .id();
        }
        let (texture_atlas, index) = basic::target.info();
        e[TRACER_N - 1] = world
            .spawn((
                SpriteSheetBundle {
                    texture_atlas,
                    sprite: TextureAtlasSprite {
                        index,
                        color: TRACER_COLOR,
                        ..default()
                    },
                    visibility: Visibility::INVISIBLE,
                    ..default()
                },
                Name::new("tracer.sprite"),
            ))
            .id();

        world
            .spawn((
                Name::new("tracer.parent"),
                TransformBundle::from_transform(default()),
                VisibilityBundle::default(),
            ))
            .push_children(&*e);
        return e;
    }
}

/// identifies an entity as being a tracer object
#[derive(Component)]
pub struct Tracer;

pub fn tracer(
    // mut commands: Commands,
    rapier_config: Res<RapierConfiguration>,
    rapier_ctx: Res<RapierContext>,
    selected: Res<SelectedModules>,
    q_children: Query<&Children>,
    mut q_transform: ParamSet<(Query<&mut Transform>, Query<Changed<Transform>>)>,
    w_out: Query<Entity, With<marker::Output>>,
    w_sprite: Query<Entity, With<TextureAtlasSprite>>,
    mut q_visibility: Query<&mut Visibility>,
    has_body: Query<With<marker::ModuleBody>>,
    // q_name: Query<&Name>,
    tracers: Res<TracerEntities>,
) {
    // get the timestep factor
    let factor;
    match rapier_config.timestep_mode {
        TimestepMode::Fixed { dt, .. } => factor = dt,
        TimestepMode::Variable { max_dt, .. } => factor = max_dt,
        TimestepMode::Interpolated { dt, .. } => factor = dt,
    }

    let per_step = 4;

    let selected = selected.selected.unwrap();
    let p_pos = q_transform.p0().entity(selected).translation.truncate();

    for tracer in **tracers {
        *q_visibility.entity_mut(tracer) = Visibility::INVISIBLE;
    }

    for entity in q_children.entity(selected).iter().with(&w_out) {
        // gotta love vector math
        if !q_transform.p1().get(entity).is_ok() {
            return;
        }
        let mut q_transform = q_transform.p0();
        // get the transform of the actual output graphic thing itself
        let mut transform = *q_transform.entity(
            q_children
                .entity(entity)
                .iter()
                .with(&w_sprite)
                .next()
                .unwrap(),
        );
        let z = transform.translation.z;
        let rot = q_transform.entity(entity).rotation;
        transform.rotate_around(Vec3::Z * z, rot);

        // borrowed from marble_io::spawn_marbles, if you change that and this breaks thats why
        let mut shape_pos = transform.translation.truncate() + p_pos;
        let shape_rot = transform.rotation;
        let mut shape_vel = shape_rot.mul_vec3(Vec3::X).truncate() * marble_io::VELOCITY_FACTOR;

        // update it such that its out of the output module
        for _ in 0..per_step {
            shape_vel += rapier_config.gravity * factor;
            shape_pos += shape_vel * factor;
        }

        // step through until either we rapier scene query turn up bad or we do <x> steps
        let mut tracer_iter = tracers.iter();
        'tracer: while let Some(&tracer) = tracer_iter.next() {
            // update the tracers
            *q_visibility.entity_mut(tracer) = Visibility::VISIBLE;
            let mut shape_transform = q_transform.entity_mut(tracer);
            *shape_transform = Transform::from_translation(shape_pos.extend(2.0));

            let filter = QueryFilter::only_fixed().exclude_sensors();

            for _ in 0..per_step {
                shape_vel += rapier_config.gravity * factor;
                shape_pos += shape_vel * factor;
                // if the shape collides
                if let Some((_, _)) = rapier_ctx.cast_shape(
                    shape_pos,
                    0.0,
                    shape_vel * factor,
                    &Collider::ball(basic::marble_small.width() * 0.5),
                    1.0,
                    filter,
                ) {
                    let mut current = shape_vel * factor;
                    let goal = 1.0 / 256.0;
                    // shape_pos += shape_vel * factor;
                    // step it closer to the colliding shape
                    while current.length() > goal {
                        if rapier_ctx
                            .cast_shape(
                                shape_pos,
                                0.0,
                                Vec2::ZERO,
                                &Collider::ball(basic::marble_small.width() * 0.5),
                                1.0,
                                filter,
                            )
                            .is_some()
                        {
                            shape_pos -= current;
                        } else {
                            shape_pos += current;
                        }
                        current /= 2.0;
                    }

                    let tracer = tracers[TRACER_N - 1];
                    *q_visibility.entity_mut(tracer) = Visibility::VISIBLE;
                    let mut shape_transform = q_transform.entity_mut(tracer);
                    *shape_transform = Transform::from_translation(shape_pos.extend(2.0));

                    break 'tracer;
                }
            }
        }
    }
}
