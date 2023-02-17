use std::any::type_name;

use bevy::ecs::query::{QueryIter, ROQueryItem, ReadOnlyWorldQuery, WorldQuery};
// use bevy_trait_query::imports::{ReadOnlyWorldQuery, WorldQuery};

use crate::*;

#[allow(unused)]
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
        QueryOutput::new(self.get_self().filter_map(|x| q.get(x).ok()))
    }

    /// queries this objects query for queries that match the other query. But *mutably*
    #[must_use]
    fn query_mut<T: Component>(
        self,
        q: &'w QuerySimple<'_, '_, T>,
    ) -> QueryOutput<impl Iterator<Item = Mut<'w, T>>> {
        QueryOutput::new(
            self.get_self()
                .filter_map(|x| unsafe { q.get_unchecked(x) }.ok()),
        )
    }

    /// queries the items and then collects them into a vector
    #[must_use]
    fn query_collect<T: Component>(self, q: &'w QuerySimple<'_, '_, T>) -> Vec<&'w T> {
        self.get_self()
            .filter_map(|x| q.get(x).ok())
            .collect()
    }

    /// queries the items and then collects them into a vector but mut
    #[must_use]
    fn query_collect_mut<T: Component>(self, q: &'w QuerySimple<'_, '_, T>) -> Vec<Mut<'w, T>> {
        self.get_self()
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
