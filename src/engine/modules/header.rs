use std::time::Duration;

use bevy::ecs::system::SystemParam;
use bevy_egui::egui::Ui;

use crate::{
    engine::marble_io::FireMarble,
    query::{QueryOutput, QueryQueryIter, QueryQuerySimple},
    *,
};

use super::{BodyType, ModuleType};

type QuerySimple<'w, 's, T> = Query<'w, 's, &'static mut T>;
// type QueryWith<'w, 's, T, W> = Query<'w, 's, &'static mut T, bevy::prelude::With<W>>;
type QueryEntity<'w, 's, W> = Query<'w, 's, bevy::prelude::Entity, bevy::prelude::With<W>>;

// information the modules get to mess around with
#[derive(SystemParam)]
pub struct ModuleResources<'w, 's> {
    pub commands: Commands<'w, 's>,
    // simple queries
    pub q_name: QuerySimple<'w, 's, Name>,
    pub q_module_type: QuerySimple<'w, 's, ModuleType>,
    pub q_input_state: QuerySimple<'w, 's, marble_io::InputState>,
    pub q_transform: QuerySimple<'w, 's, Transform>,
    pub q_children: QuerySimple<'w, 's, Children>,
    pub q_sprite: QuerySimple<'w, 's, Sprite>,
    // entity queries
    pub w_input: QueryEntity<'w, 's, marker::Input>,
    pub w_output: QueryEntity<'w, 's, marker::Output>,
    pub w_indicator: QueryEntity<'w, 's, marker::Indicator>,
    // events
    pub fire_marble: EventWriter<'w, 's, FireMarble>,
    // resources
    pub keyboard: Res<'w, Input<KeyCode>>,
}

impl<'w, 's> ModuleResources<'w, 's> {
    #[must_use]
    pub fn inputs(
        &'w self,
        module: Entity,
    ) -> QueryOutput<impl Iterator<Item = bevy::prelude::Entity> + 'w> {
        self.q_children.entity(module).iter().with(&self.w_input)
    }

    #[must_use]
    pub fn outputs(
        &'w self,
        module: Entity,
    ) -> QueryOutput<impl Iterator<Item = bevy::prelude::Entity> + 'w> {
        self.q_children.entity(module).iter().with(&self.w_output)
    }

    /// update the indicator lights for the inputs to show if theyre full or not
    pub fn update_input_indicators(&mut self, module: Entity) {
        let input_states = self.q_input_state.entity(module).iter();
        let inputs = self.inputs(module);

        for (input, input_state) in inputs.zip(input_states) {
            let indicator_sprite = &mut self
                .q_children
                .entity(input)
                .iter()
                .with(&self.w_indicator)
                .query_collect_mut(&self.q_sprite)[0];

            let color = &mut indicator_sprite.color;
            let hsla = color.as_hsla_f32();
            let hue = [117.0, 0.0][input_state.is_some() as usize];
            let new_color = Color::hsla(hue, hsla[1], hsla[2], hsla[3]);
            *color = new_color;
        }
    }
}

/// "i want to do something after x second(s) pls help"
#[derive(Deref, DerefMut, Component)]
pub struct ModuleCallbackTimer(Timer);

impl ModuleCallbackTimer {
    pub fn new(ticks: u32) -> Self {
        ModuleCallbackTimer(Timer::from_seconds(ticks as f32, TimerMode::Once))
    }
}

pub fn update_module_callbacks(
    mut set: ParamSet<(
        ModuleResources,
        Query<(&mut ModuleType, Entity, &mut ModuleCallbackTimer)>,
    )>,
) {
    // no mutability conflict as they conflict because the query gets ModuleType mutably
    // and im passing in ModuleResources into the ModuleType we get, so
    // shhhhhhh
    let res_module = &set.p0() as *const ModuleResources as *mut ModuleResources;
    let res_module = unsafe { &mut *res_module };
    for (mut module, entity, mut timer) in set.p1().iter_mut() {
        // tick once
        timer.tick(Duration::from_secs(1));

        if timer.finished() {
            module.get_inner_mut().callback_update(res_module, entity);
            res_module
                .commands
                .entity(entity)
                .remove::<ModuleCallbackTimer>();
        }
    }
}

/// tells this entity that they need to be updated (!!! (!!!)) (probably a module)
#[derive(Deref, DerefMut)]
pub struct UpdateModule(pub Entity);

/// run the update functions for the modules!!
#[allow(clippy::type_complexity)]
pub fn update_modules(
    mut set: ParamSet<(
        ModuleResources,
        Query<(&mut ModuleType, Entity), Changed<marble_io::InputState>>,
        // Query<(&mut ModuleType, Entity)>,
    )>,
) {
    // no mutability conflict as they conflict because the query gets ModuleType mutably
    // and im passing in ModuleResources into the ModuleType we get, so
    // shhhhhhh
    let res_module = &set.p0() as *const ModuleResources as *mut ModuleResources;
    for (mut module, entity) in set.p1().iter_mut() {
        module
            .get_inner_mut()
            .update(unsafe { &mut *res_module }, entity)
    }
}

pub trait Module {
    /// return instructions on spawning this module
    fn spawn_instructions(&self) -> &'static SpawnInstructions;
    /// function that runs to update this module
    fn update(&mut self, res: &mut ModuleResources, module: Entity);
    fn callback_update(&mut self, res: &mut ModuleResources, module: Entity);
    /// function to build the ui / interactive elements
    fn interactive(&mut self, res: &mut ModuleResources, ui: &mut Ui, entity: Entity);
    /// the name of the module
    fn get_name(&self) -> &'static str;
    /// the identifier of the module
    fn get_identifier(&self) -> &'static str;
}

/// basically, imagine offsetting some object by `offset` in the x-axis, then rotating it around the origin `rotation` radians.
///
/// this is what this function does.
pub fn transform_from_offset_rotate(offset: f32, rotation: f32, z: f32) -> Transform {
    let rotation = Quat::from_rotation_z(rotation);
    let translation = rotation.mul_vec3(Vec3::X * offset) + Vec3::Z * z;
    Transform {
        rotation,
        translation,
        scale: Vec3::ONE,
    }
}

#[derive(Default)]
pub struct SpawnInstructions {
    pub body: BodyType,
    pub input_transforms: Vec<Transform>,
    pub output_transforms: Vec<Transform>,
}

impl SpawnInstructions {
    pub fn from_body(body: BodyType) -> Self {
        Self { body, ..default() }
    }

    pub fn with_input_rotations<T: Iterator<Item = f32>>(mut self, input_transforms: T) -> Self {
        self.input_transforms = input_transforms
            .map(|r| {
                let rot = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, r.to_radians());
                let mut transform = Transform::from_xyz(
                    self.body.offset() - 4.0,
                    0.0,
                    ZOrder::InputComponent.f32(),
                );
                transform.rotate_around(Vec3::ZERO, rot);
                transform
            })
            .collect();
        self
    }

    pub fn with_output_rotations<T: Iterator<Item = f32>>(mut self, output_transforms: T) -> Self {
        self.output_transforms = output_transforms
            .map(|r| {
                let rot = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, r.to_radians());
                let mut transform = Transform::from_xyz(
                    self.body.offset() - 3.5,
                    0.0,
                    ZOrder::OutputComponent.f32(),
                );
                transform.rotate_around(Vec3::ZERO, rot);
                transform
            })
            .collect();
        self
    }
}
