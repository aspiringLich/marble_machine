use std::default;
use std::f32::consts;
use std::iter::FilterMap;
use std::marker::PhantomData;

use bevy::ecs::query::{QueryItem, QueryIter, ROQueryItem, ReadOnlyWorldQuery, WorldQuery};
use bevy::ecs::system::SystemParam;
use bevy::input::keyboard::KeyboardInput;
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

mod param {
    use crate::*;
    use bevy::ecs::system::SystemParam;

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
        // events
        pub spawn_marble: EventWriter<'w, 's, module::FireMarble>,
        // resources
        pub keyboard: Res<'w, Input<KeyCode>>,
    }
}

// #[derive(Debug, Clone)]
// pub struct QueryQueryIter<
//     'a,
//     Q: WorldQuery,
//     R: ReadOnlyWorldQuery,
//     I: Iterator<Item = Entity>,
//     F = &'static dyn Fn(Entity) -> Option<<Q as WorldQuery>::Item<'a>>,
// > {
//     iter: I,
//     func: F,
//     phantom1: PhantomData<&'a Q>,
//     phantom2: PhantomData<&'a R>,
// }

#[derive(Debug, Clone)]
pub struct QueryQueryIter<I, Q> {
    q: Q,
    iter: I,
}

impl<I: Iterator, Q: WorldQuery, R: ReadOnlyWorldQuery> QueryQueryIter<I, Q> {
    pub fn new<'a>(iter: I, q: Query<Q, R>) -> Self {
        Self { q, iter }
    }
}

impl<B, I: Iterator<Item = Entity>, F> Iterator for QueryQueryIter<I, F>
where
    F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {}
        todo!()
    }
}

pub trait QueryQuery<'w> {
    type I: Iterator<Item = Entity>;

    fn get(self) -> Self::I;

    /// queries all entities
    fn query<Q: ReadOnlyWorldQuery, R: ReadOnlyWorldQuery, F>(
        self,
        other: &'w Query<'w, '_, Q, R>,
    ) -> FilterMap<Self::I, F> {
        let cls = |entity| other.get(entity).ok();
        self.get().filter_map(cls)
    }

    /// queries all entities mutably
    fn query_mut<Q: WorldQuery, R: ReadOnlyWorldQuery, I: Iterator, F>(
        self,
        other: &'w Query<'w, '_, Q, R>,
    ) -> FilterMap<I, F>
    where
        F: FnMut(I::Item) -> Option<Q>,
        Self: Sized,
    {
        QueryQueryIter {
            iter: self.get(),
            func: Box::new(|entity| unsafe { other.get_unchecked(entity) }.ok()),
            phantom: PhantomData,
        }
    }
}

// impl<'w, 's, F: ReadOnlyWorldQuery> QueryQuery<'w> for Query<'w, 'w, Entity, F> {
//     fn get(self) -> &'s mut QueryIter<'w, 's, Entity, F> {
//         &mut self.iter()
//     }
// }

// impl<'w, 's, Q: WoReadOnlyWorldQueryrldQuery, T = QueryQueryIter<'w, Q, dyn Iterator<Item = Entity>>>
//     QueryQuery<'w, Q, T> for T
// {
//     fn get(self) -> T {
//         self
//     }
// }

impl<'w> QueryQuery<'w> for std::slice::Iter<'w, Entity> {
    type I = impl Iterator<Item = Entity> + 'w;

    fn get(self) -> Self::I {
        self.map(|entity| *entity)
    }
}

pub use param::ModuleResources;

// impl<'w, 's> ModuleResources<'w, 's> {
//     /// get the children of an entity
//     pub fn get_children_of(&self, entity: Entity) -> &Children {
//         self.q_children.get(entity).unwrap()
//     }

//     /// query every single child of an entity
//     pub fn query_children<Q: WorldQuery, F: ReadOnlyWorldQuery>(
//         &'w self,
//         entity: Entity,
//         query: &'w Query<'w, 's, Q, F>,
//     ) -> impl Iterator<Item = ROQueryItem<'_, Q>> {
//         self.get_children_of(entity)
//             .iter()
//             .filter_map(|child| query.get(*child).ok())
//     }

//     /// query every single child of an entity mutable
//     ///
//     /// NOTICE: this is very unsafe as i use `get_unchecked` to get around the fact the closure becomes a `FnMut`
//     ///
//     /// the function is not marked as unsafe because i am doing this for ergonomics anyway
//     pub fn query_children_mut<Q: WorldQuery, F: ReadOnlyWorldQuery>(
//         &'w self,
//         entity: Entity,
//         query: &'w Query<'w, 's, Q, F>,
//     ) -> impl Iterator<Item = QueryItem<'_, Q>> {
//         self.get_children_of(entity)
//             .iter()
//             .filter_map(|child| unsafe { query.get_unchecked(*child) }.ok())
//     }
// }

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

        let children = q_children.get(module).unwrap();
        let inputs: Vec<_> = children.iter().query(w_input).collect();
        let outputs: Vec<_> = children.iter().query(w_output).collect();

        let mut input_tfs: Vec<_> = inputs.iter().query_mut(q_transform).collect();
        let mut output_tfs: Vec<_> = inputs.iter().query_mut(q_transform).collect();

        *input_tfs[0] = body_small_transform(self.input_rot);
        *output_tfs[0] = body_small_transform(self.output_rot);

        if ui.button("Fire Marble!").clicked() {
            let mut from = q_children.get(module).unwrap();
            spawn_marble.send(FireMarble {
                marble: Marble::Bit { value: true },
                from: from.iter().query(w_output).next().unwrap(),
                power: 1.0,
            })
        }
    }

    const NAME: &'static str = "Basic Module";
}
