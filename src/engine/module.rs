use std::time::Duration;

use bevy_egui::*;

use crate::*;

use crate::spawn::{BodyType, SpawnInstructions};
use atlas::AtlasDictionary;
use marble::Marble;
use marble_io::FireMarble;

use egui::*;

#[derive(Copy, Clone, Component)]
pub enum ModuleType {
    Basic(Basic),
}

impl Default for ModuleType {
    fn default() -> Self {
        Self::Basic(default())
    }
}

impl ModuleType {
    pub fn get_inner(&mut self) -> &mut impl Module {
        match self {
            Self::Basic(x) => x,
        }
    }
}

pub mod param {
    use std::intrinsics::type_name;

    use crate::*;
    use bevy::ecs::{
        query::{QueryIter, ROQueryItem, ReadOnlyWorldQuery, WorldQuery},
        system::SystemParam,
    };

    type QuerySimple<'w, 's, T> = Query<'w, 's, &'static mut T>;
    // type QueryWith<'w, 's, T, W> = Query<'w, 's, &'static mut T, bevy::prelude::With<W>>;
    type QueryEntity<'w, 's, W> = Query<'w, 's, bevy::prelude::Entity, bevy::prelude::With<W>>;

    pub trait QueryQuerySimple<'a, Q: WorldQuery + 'a>
    where
        Self: Sized,
    {
        fn get_self(&self) -> &Query<'_, '_, Q, ()>;

        /// get the thing that satisfies this query under this entity
        #[must_use]
        fn entity(&'a self, entity: Entity) -> ROQueryItem<'_, Q> {
            self.get_self().get(entity).unwrap_or_else(|_| {
                error!(
                    "Expected component {} to exist on the queried entity",
                    type_name::<Q>()
                );
                panic!()
            })
        }

        /// gets the thing that satisfies this query under this entity *mutably*
        /// shhhhhhhhhh ignore that unsafe block shhhhhhhh
        /// im pretty sure it isnt unsafe as it lives on within that mutable query
        #[must_use]
        fn entity_mut(&'a mut self, entity: Entity) -> Q::Item<'a> {
            unsafe {
                self.get_self().get_unchecked(entity).unwrap_or_else(|_| {
                    error!(
                        "Expected component {} to exist on the queried entity",
                        type_name::<Q>()
                    );
                    panic!()
                })
            }
        }

        /// does i has this???
        fn has(&'a self, entity: Entity) -> bool {
            self.get_self().get(entity).is_ok()
        }
    }

    impl<'a, Q: WorldQuery> QueryQuerySimple<'a, Q> for Query<'_, '_, Q, ()>
    where
        Q: 'a,
    {
        fn get_self(&self) -> &Query<'_, '_, Q, ()> {
            self
        }
    }

    /// the output from the query
    #[derive(Clone)]
    pub struct QueryOutput<T: Sized>(T);

    impl<'a, I: Iterator + 'a, T> Iterator for QueryOutput<I>
    where
        I: Iterator<Item = T>,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<T> QueryOutput<T> {
        pub fn new(t: T) -> Self {
            QueryOutput(t)
        }
    }

    pub trait QueryQueryIter<'w>
    where
        Self: Sized,
    {
        fn get_self(self) -> impl Iterator<Item = Entity>;

        /// queries this objects query for queries that match the other query.
        #[must_use]
        fn query<T: Component>(
            self,
            q: &'w QuerySimple<'_, '_, T>,
        ) -> QueryOutput<impl Iterator<Item = &'w T>> {
            QueryOutput::new(self.get_self().into_iter().filter_map(|x| q.get(x).ok()))
        }

        /// queries this objects query for queries that match the other query. But *mutably*
        #[must_use]
        fn query_mut<T: Component>(
            self,
            q: &'w QuerySimple<'_, '_, T>,
        ) -> QueryOutput<impl Iterator<Item = Mut<'w, T>>> {
            QueryOutput::new(
                self.get_self()
                    .into_iter()
                    .filter_map(|x| unsafe { q.get_unchecked(x) }.ok()),
            )
        }

        /// queries the items and then collects them into a vector
        #[must_use]
        fn query_collect<T: Component>(self, q: &'w QuerySimple<'_, '_, T>) -> Vec<&'w T> {
            self.get_self()
                .into_iter()
                .filter_map(|x| q.get(x).ok())
                .collect()
        }

        /// queries the items and then collects them into a vector but mut
        #[must_use]
        fn query_collect_mut<T: Component>(self, q: &'w QuerySimple<'_, '_, T>) -> Vec<Mut<'w, T>> {
            self.get_self()
                .into_iter()
                .filter_map(|x| unsafe { q.get_unchecked(x) }.ok())
                .collect()
        }

        /// Filters this objects queries for queries that match the query
        #[must_use]
        fn with<T: Component>(
            self,
            w: &'w QueryEntity<'w, 'w, T>,
        ) -> QueryOutput<impl Iterator<Item = Entity> + 'w>
        where
            Self: 'w,
        {
            QueryOutput::new(
                self.get_self()
                    .into_iter()
                    .filter_map(move |x| w.get(x).ok()),
            )
        }

        /// Filters this objects queries for queries that match the query and then collects it
        #[must_use]
        fn with_collect<T: Component>(self, w: &'w QueryEntity<'w, 'w, T>) -> Vec<Entity>
        where
            Self: 'w,
        {
            self.get_self()
                .into_iter()
                .filter_map(move |x| w.get(x).ok())
                .collect()
        }
    }

    impl<'w, 's, F: ReadOnlyWorldQuery> QueryQueryIter<'w> for QueryIter<'w, 's, Entity, F> {
        fn get_self(self) -> QueryIter<'w, 's, Entity, F> {
            self
        }
    }

    impl<'w> QueryQueryIter<'w> for std::slice::Iter<'w, Entity> {
        fn get_self(self) -> impl Iterator<Item = Entity> + 'w {
            self.copied()
        }
    }

    impl<'w, I: Iterator<Item = Entity> + 'w> QueryQueryIter<'w> for QueryOutput<I> {
        fn get_self(self) -> impl Iterator<Item = Entity> + 'w {
            self
        }
    }

    impl<'w> QueryQueryIter<'w> for &'w Vec<Entity> {
        fn get_self(self) -> impl Iterator<Item = Entity> + 'w {
            self.iter().copied()
        }
    }

    /// information the modules get to mess around with
    #[derive(SystemParam)]
    pub struct ModuleResources<'w, 's> {
        pub commands: Commands<'w, 's>,
        // simple queries
        pub q_name: QuerySimple<'w, 's, Name>,
        pub q_module_type: QuerySimple<'w, 's, module::ModuleType>,
        pub q_input_state: QuerySimple<'w, 's, marble_io::InputState>,
        pub q_transform: QuerySimple<'w, 's, Transform>,
        pub q_children: QuerySimple<'w, 's, Children>,
        pub q_sprite: QuerySimple<'w, 's, Sprite>,
        // entity queries
        pub w_input: QueryEntity<'w, 's, marker::Input>,
        pub w_output: QueryEntity<'w, 's, marker::Output>,
        pub w_indicator: QueryEntity<'w, 's, marker::Indicator>,
        // events
        pub fire_marble: EventWriter<'w, 's, module::FireMarble>,
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
}

use param::{ModuleResources, QueryQueryIter, QueryQuerySimple};

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
            module.get_inner().callback_update(res_module, entity);
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
            .get_inner()
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

// /// returns a transform that equates to a valid i/o position around a `body_small`.
// /// !!! IF THE OFFSET CHANGES REMEMBER TO ALSO CHANGE IT IN INTERACTIVE
// pub fn body_small_transform(rotation: f32) -> Transform {
//     transform_from_offset_rotate(basic::body_small.width() * 0.5 + 1.0, rotation, 0.25)
// }

// /// goes over a slice also it takes in degrees instead of radians
// ///
// pub fn body_small_transforms(rotations: &'static [f32]) -> impl Iterator<Item = Transform> {
//     rotations
//         .into_iter()
//         .map(|x| body_small_transform(x.to_radians()))
// }

#[derive(Copy, Clone)]
pub struct Basic;

impl Default for Basic {
    fn default() -> Self {
        Basic
    }
}

static BASIC_INSTRUCTIONS: Lazy<SpawnInstructions> = Lazy::new(|| {
    SpawnInstructions::from_body(BodyType::Small)
        .with_input_rotations([270.].into_iter())
        .with_output_rotations([90.].into_iter())
});

impl Module for Basic {
    fn spawn_instructions(&self) -> &'static SpawnInstructions {
        &BASIC_INSTRUCTIONS
    }

    fn update(&mut self, res: &mut ModuleResources, module: Entity) {
        // update indicators
        res.update_input_indicators(module);

        let ModuleResources {
            commands,
            q_input_state,
            ..
        } = res;

        let input_state = q_input_state.entity(module);
        // if we have a marble then activate the callback
        if input_state[0].is_some() {
            commands.entity(module).insert(ModuleCallbackTimer::new(10));
        }
    }

    fn callback_update(&mut self, res: &mut ModuleResources, module: Entity) {
        let mut input_state = res.q_input_state.entity_mut(module);

        // if theres a marble in there (there should be)
        if let Some(marble) = input_state[0] {
            let ModuleResources {
                fire_marble,
                w_output,
                q_children,
                ..
            } = res;
            let outputs = q_children.entity(module).iter().with_collect(w_output);

            // fire it outta the input and mark that the input is empty
            fire_marble.send(FireMarble::new(marble, outputs[0], 1.0));
            input_state[0] = None;
        } else {
            warn!(
                "callback_update on {}: expected marble in input state",
                self.get_name()
            )
        }
        res.update_input_indicators(module);
    }

    fn interactive(&mut self, res: &mut ModuleResources, ui: &mut Ui, module: Entity) {
        // let inputs: Vec<_> = res.inputs(module).collect();
        let outputs: Vec<_> = res.outputs(module).collect();

        let ModuleResources {
            fire_marble: spawn_marble,
            // q_transform,
            // keyboard,
            ..
        } = &mut *res;
        // let input_tfs = inputs.query_collect_mut(q_transform);
        // let output_tfs = outputs.query_collect_mut(q_transform);

        // ui::Layout::new()
        //     .default_rotation_sliders(input_tfs, output_tfs, &body_small_transform)
        //     .build(ui);

        // cool epic le hacker debug button
        if ui.button("Fire Marble!").clicked() {
            spawn_marble.send(FireMarble {
                marble: Marble::Bit { value: true },
                from: outputs[0],
                power: 1.0,
            })
        }
    }

    fn get_name(&self) -> &'static str {
        "Basic Module"
    }

    fn get_identifier(&self) -> &'static str {
        "basic.module"
    }
}
