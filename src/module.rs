use bevy_egui::*;

use crate::atlas::AtlasDictionary;
use crate::marble::Marble;
use crate::marble_io::FireMarble;
use crate::spawn::SpawnInstruction;
use crate::ui::UiElements;
use crate::*;

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
    pub fn get_inner<'a>(&'a mut self) -> &'a mut impl Module {
        match self {
            Self::Basic(x) => x,
        }
    }
}

pub mod param {
    use crate::*;
    use bevy::ecs::{
        query::{QueryIter, ROQueryItem, ReadOnlyWorldQuery, WorldQuery},
        system::SystemParam,
    };

    type QuerySimple<'w, 's, T> = Query<'w, 's, &'static mut T>;
    type QueryWith<'w, 's, T, W> = Query<'w, 's, &'static mut T, bevy::prelude::With<W>>;
    type QueryEntity<'w, 's, W> = Query<'w, 's, bevy::prelude::Entity, bevy::prelude::With<W>>;

    /// information the modules get to mess around with
    #[derive(SystemParam)]
    pub struct ModuleResources<'w, 's> {
        pub commands: Commands<'w, 's>,
        // simple queries
        pub q_name: QuerySimple<'w, 's, Name>,
        pub q_module_type: QuerySimple<'w, 's, crate::ModuleType>,
        pub q_transform: QuerySimple<'w, 's, Transform>,
        pub q_children: QuerySimple<'w, 's, Children>,
        // entity queries
        pub w_input: QueryEntity<'w, 's, marker::Input>,
        pub w_output: QueryEntity<'w, 's, marker::Output>,
        pub w_indicator: QueryEntity<'w, 's, marker::Indicator>,
        // events
        pub spawn_marble: EventWriter<'w, 's, module::FireMarble>,
        // resources
        pub keyboard: Res<'w, Input<KeyCode>>,
    }

    pub trait QueryQuerySimple<'a, Q: WorldQuery + 'a>
    where
        Self: Sized,
    {
        fn get_self(&self) -> &Query<'_, '_, Q, ()>;

        /// get the thing that satisfies this query under this entity
        fn entity(&'a self, entity: Entity) -> ROQueryItem<'_, Q> {
            self.get_self().get(entity).unwrap()
        }

        /// gets the thing that satisfies this query under this entity *mutably*
        /// shhhhhhhhhh ignore that unsafe block shhhhhhhh
        /// im pretty sure it isnt unsafe as it lives on within that mutable query
        fn entity_mut(&'a mut self, entity: Entity) -> Q::Item<'a> {
            unsafe {
                self.get_self().get_unchecked(entity).expect(&format!(
                    "[{}{}] component was expected but was not found",
                    file!(),
                    line!(),
                ))
            }
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

    impl<T> std::ops::Deref for QueryOutput<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> std::ops::DerefMut for QueryOutput<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
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
        fn query<T: Component>(
            self,
            q: &'w QuerySimple<'w, '_, T>,
        ) -> QueryOutput<impl Iterator<Item = &'w T>> {
            QueryOutput::new(self.get_self().into_iter().filter_map(|x| q.get(x).ok()))
        }

        /// queries this objects query for queries that match the other query. But *mutably*
        fn query_mut<T: Component>(
            self,
            q: &'w QuerySimple<'w, 'w, T>,
        ) -> QueryOutput<impl Iterator<Item = Mut<'w, T>>> {
            QueryOutput::new(
                self.get_self()
                    .into_iter()
                    .filter_map(|x| unsafe { q.get_unchecked(x) }.ok()),
            )
        }

        /// Filters this objects queries for queries that match the query but returns the entity not the query.
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
    }

    impl<'w, 's, F: ReadOnlyWorldQuery> QueryQueryIter<'w> for QueryIter<'w, 's, Entity, F> {
        fn get_self(self) -> QueryIter<'w, 's, Entity, F> {
            self
        }
    }

    impl<'w, 's> QueryQueryIter<'w> for std::slice::Iter<'w, Entity> {
        fn get_self(self) -> impl Iterator<Item = Entity> + 'w {
            self.map(|x| *x)
        }
    }

    impl<'w, 's, I: Iterator<Item = Entity> + 'w> QueryQueryIter<'w> for QueryOutput<I> {
        fn get_self(self) -> impl Iterator<Item = Entity> + 'w {
            self
        }
    }
}

use param::{ModuleResources, QueryQueryIter, QueryQuerySimple};

pub trait Module {
    /// return instructions on spawning this module
    fn spawn_instructions(&self) -> Vec<SpawnInstruction>;
    /// function to build the gui
    fn gui(&mut self, res: &mut ModuleResources, ui: &mut Ui, entity: Entity);
    /// the name of the module
    const NAME: &'static str;
    fn get_name(&self) -> &'static str {
        Self::NAME
    }
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

/// returns a transform that equates to a valid i/o position around a `body_small`.
pub fn body_small_transform(rotation: f32) -> Transform {
    transform_from_offset_rotate(basic::body_small.width() * 0.5 + 1.0, rotation, 0.25)
}

#[derive(Copy, Clone)]
pub struct Basic {
    pub input_rot: f32,
    pub output_rot: f32,
}

impl Default for Basic {
    fn default() -> Self {
        Basic {
            input_rot: f32::to_radians(270.0),
            output_rot: f32::to_radians(90.0),
        }
    }
}

impl Module for Basic {
    fn spawn_instructions(&self) -> Vec<SpawnInstruction> {
        use SpawnInstruction::*;

        vec![BodySmall(vec![self.input_rot], vec![self.output_rot])]
    }

    fn gui(&mut self, res: &mut ModuleResources, ui: &mut Ui, module: Entity) {
        let ModuleResources {
            commands,
            spawn_marble,
            q_children,
            q_transform,
            w_input,
            w_output,
            ..
        } = &mut *res;
        let grid_lyt = |ui: &mut Ui| {
            ui.angle_slider("Input", &mut self.input_rot);
            ui.angle_slider("Output", &mut self.output_rot);
        };
        ui::Layout::new().with_grid(grid_lyt).build(ui);

        // get inputs and outputs
        let children = q_children.entity(module);
        let inputs: Vec<_> = children.iter().with(&w_input).collect();
        let outputs: Vec<_> = children.iter().with(&w_output).collect();
        let mut input_tfs: Vec<_> = inputs.iter().query_mut(&q_transform).collect();
        let mut output_tfs: Vec<_> = outputs.iter().query_mut(&q_transform).collect();

        // set input and output transforms
        *input_tfs[0] = body_small_transform(self.input_rot);
        *output_tfs[0] = body_small_transform(self.output_rot);

        // cool epic le hacker debug button
        if ui.button("Fire Marble!").clicked() {
            spawn_marble.send(FireMarble {
                marble: Marble::Bit { value: true },
                from: outputs[0],
                power: 1.0,
            })
        }
    }

    const NAME: &'static str = "Basic Module";
}
