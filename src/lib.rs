use std::marker::PhantomData;

use bevy_ecs::{
    archetype::Archetype,
    component::{Component, ComponentId, Components, Mutable, Tick},
    entity::{Entity, EntityHashMap, EntityHashSet},
    query::{FilteredAccess, QueryData, WorldQuery, WriteFetch},
    storage::{Table, TableRow},
    system::lifetimeless::Read,
    world::{Mut, World, unsafe_world_cell::UnsafeWorldCell},
};
use bevy_utils::Parallel;

// TODO:
// Functionality:
// - [ ] general shape of types
// - [ ] method impls
// - [ ] WorldQuery impl
// - [ ] QueryData impl
// Polish:
// - [ ] unit tests
// - [ ] setup CI
// - [ ] benchmarks
// Docs:
// - [ ] module docs
// - [ ] rewrite method docs?
// - [ ] usage examples
// - [ ] make a README

/// A view into a single entity and component in a world, which may either be vacant or occupied.
pub enum Entry<'w, 's, T: Component> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'w, 's, T>),
    /// A vacant entry.
    Vacant(VacantEntry<'w, 's, T>),
}

impl<'w, 's, T: Component> Entry<'w, 's, T> {
    /// Provides in-place mutable access to an occupied entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn(Comp(0));
    ///
    /// entity.entry::<Comp>().and_modify(|mut c| c.0 += 1);
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 1);
    /// ```
    #[inline]
    pub fn and_modify<F: FnOnce(Mut<'_, T>)>(self, f: F) -> Self {
        // match self {
        //     Entry::Occupied(mut entry) => {
        //         f(entry.get_mut());
        //         Entry::Occupied(entry)
        //     }
        //     Entry::Vacant(entry) => Entry::Vacant(entry),
        // }
        todo!()
    }
}

impl<'w, 's, T: Component> Entry<'w, 's, T> {
    /// Replaces the component of the entry, and returns an [`OccupiedEntry`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn_empty();
    ///
    /// let entry = entity.entry().insert_entry(Comp(4));
    /// assert_eq!(entry.get(), &Comp(4));
    ///
    /// let entry = entity.entry().insert_entry(Comp(2));
    /// assert_eq!(entry.get(), &Comp(2));
    /// ```
    #[inline]
    pub fn insert_entry(self, component: T) -> OccupiedEntry<'w, 's, T> {
        // match self {
        //     Entry::Occupied(mut entry) => {
        //         entry.insert(component);
        //         entry
        //     }
        //     Entry::Vacant(entry) => entry.insert(component),
        // }
        todo!()
    }

    /// Ensures the entry has this component by inserting the given default if empty, and
    /// returns a mutable reference to this component in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn_empty();
    ///
    /// entity.entry().or_insert(Comp(4));
    /// # let entity_id = entity.id();
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 4);
    ///
    /// # let mut entity = world.get_entity_mut(entity_id).unwrap();
    /// entity.entry().or_insert(Comp(15)).into_mut().0 *= 2;
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 8);
    /// ```
    #[inline]
    pub fn or_insert(self, default: T) -> OccupiedEntry<'w, 's, T> {
        // match self {
        //     Entry::Occupied(entry) => entry,
        //     Entry::Vacant(entry) => entry.insert(default),
        // }
        todo!()
    }

    /// Ensures the entry has this component by inserting the result of the default function if
    /// empty, and returns a mutable reference to this component in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn_empty();
    ///
    /// entity.entry().or_insert_with(|| Comp(4));
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 4);
    /// ```
    #[inline]
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> OccupiedEntry<'w, 's, T> {
        // match self {
        //     Entry::Occupied(entry) => entry,
        //     Entry::Vacant(entry) => entry.insert(default()),
        // }
        todo!()
    }
}

impl<'w, 's, T: Component + Default> Entry<'w, 's, T> {
    /// Ensures the entry has this component by inserting the default value if empty, and
    /// returns a mutable reference to this component in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::prelude::*;
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn_empty();
    ///
    /// entity.entry::<Comp>().or_default();
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 0);
    /// ```
    #[inline]
    pub fn or_default(self) -> OccupiedEntry<'w, 's, T> {
        // match self {
        //     Entry::Occupied(entry) => entry,
        //     Entry::Vacant(entry) => entry.insert(Default::default()),
        // }
        todo!()
    }
}

/// A view into an occupied entry in a [`EntityWorldMut`]. It is part of the [`Entry`] enum.
///
/// The contained entity must have the component type parameter if we have this struct.
pub struct OccupiedEntry<'w, 's, T: Component> {
    // entity_world: &'a mut EntityWorldMut<'w>,
    _data: PhantomData<T>,
    _temp: PhantomData<&'w &'s ()>,
}

impl<'w, 's, T: Component> OccupiedEntry<'w, 's, T> {
    /// Gets a reference to the component in the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::{prelude::*, world::Entry};
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn(Comp(5));
    ///
    /// if let Entry::Occupied(o) = entity.entry::<Comp>() {
    ///     assert_eq!(o.get().0, 5);
    /// }
    /// ```
    #[inline]
    pub fn get(&self) -> &T {
        // This shouldn't panic because if we have an OccupiedEntry the component must exist.
        // self.entity_world.get::<T>().unwrap()
        todo!()
    }

    /// Replaces the component of the entry.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::{prelude::*, world::Entry};
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn(Comp(5));
    ///
    /// if let Entry::Occupied(mut o) = entity.entry::<Comp>() {
    ///     o.insert(Comp(10));
    /// }
    ///
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 10);
    /// ```
    #[inline]
    pub fn insert(&mut self, component: T) {
        // self.entity_world.insert(component);
        todo!()
    }

    /// Removes the component from the entry and returns it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::{prelude::*, world::Entry};
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn(Comp(5));
    ///
    /// if let Entry::Occupied(o) = entity.entry::<Comp>() {
    ///     assert_eq!(o.take(), Comp(5));
    /// }
    ///
    /// assert_eq!(world.query::<&Comp>().iter(&world).len(), 0);
    /// ```
    #[inline]
    pub fn take(self) -> T {
        // This shouldn't panic because if we have an OccupiedEntry the component must exist.
        // self.entity_world.take().unwrap()
        todo!()
    }
}

impl<'w, 'a, T: Component<Mutability = Mutable>> OccupiedEntry<'w, 'a, T> {
    /// Gets a mutable reference to the component in the entry.
    ///
    /// If you need a reference to the `OccupiedEntry` which may outlive the destruction of
    /// the `Entry` value, see [`into_mut`].
    ///
    /// [`into_mut`]: Self::into_mut
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::{prelude::*, world::Entry};
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn(Comp(5));
    ///
    /// if let Entry::Occupied(mut o) = entity.entry::<Comp>() {
    ///     o.get_mut().0 += 10;
    ///     assert_eq!(o.get().0, 15);
    ///
    ///     // We can use the same Entry multiple times.
    ///     o.get_mut().0 += 2
    /// }
    ///
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 17);
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> Mut<'_, T> {
        // This shouldn't panic because if we have an OccupiedEntry the component must exist.
        // self.entity_world.get_mut::<T>().unwrap()
        todo!()
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry with
    /// a lifetime bound to the `EntityWorldMut`.
    ///
    /// If you need multiple references to the `OccupiedEntry`, see [`get_mut`].
    ///
    /// [`get_mut`]: Self::get_mut
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::{prelude::*, world::Entry};
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn(Comp(5));
    ///
    /// if let Entry::Occupied(o) = entity.entry::<Comp>() {
    ///     o.into_mut().0 += 10;
    /// }
    ///
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 15);
    /// ```
    #[inline]
    pub fn into_mut(self) -> Mut<'a, T> {
        // This shouldn't panic because if we have an OccupiedEntry the component must exist.
        // self.entity_world.get_mut().unwrap()
        todo!()
    }
}

/// A view into a vacant entry in a [`EntityWorldMut`]. It is part of the [`Entry`] enum.
pub struct VacantEntry<'w, 's, T: Component> {
    // entity_world: &'a mut EntityWorldMut<'w>,
    _data: PhantomData<fn(T)>,
    _temp: PhantomData<&'w &'s ()>,
}

impl<'w, 'a, T: Component> VacantEntry<'w, 'a, T> {
    /// Inserts the component into the `VacantEntry` and returns an `OccupiedEntry`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bevy_ecs::{prelude::*, world::Entry};
    /// #[derive(Component, Default, Clone, Copy, Debug, PartialEq)]
    /// struct Comp(u32);
    ///
    /// # let mut world = World::new();
    /// let mut entity = world.spawn_empty();
    ///
    /// if let Entry::Vacant(v) = entity.entry::<Comp>() {
    ///     v.insert(Comp(10));
    /// }
    ///
    /// assert_eq!(world.query::<&Comp>().single(&world).unwrap().0, 10);
    /// ```
    #[inline]
    pub fn insert(self, component: T) -> OccupiedEntry<'w, 'a, T> {
        // self.entity_world.insert(component);
        // OccupiedEntry {
        //     entity_world: self.entity_world,
        //     _marker: PhantomData,
        // }
        todo!()
    }
}

struct EntryFetch<'w, T: Component> {
    fetch: WriteFetch<'w, T>,
    state: &'w EntryState<T>,
}

impl<'w, T: Component> Clone for EntryFetch<'w, T> {
    fn clone(&self) -> Self {
        Self {
            fetch: self.fetch.clone(),
            state: self.state.clone(),
        }
    }
}

struct EntryState<T: Component> {
    component_id: ComponentId,
    record: Parallel<EntryRecord<T>>,
}

struct EntryRecord<T: Component> {
    inserts: EntityHashMap<T>,
    removes: EntityHashSet,
}

unsafe impl<'w, 's, T: Component> WorldQuery for Entry<'w, 's, T> {
    type Fetch<'a> = EntryFetch<'a, T>;

    type State = EntryState<T>;

    fn shrink_fetch<'wlong: 'wshort, 'wshort>(fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
        todo!()
    }

    unsafe fn init_fetch<'_w>(
        world: UnsafeWorldCell<'_w>,
        state: &Self::State,
        last_run: Tick,
        this_run: Tick,
    ) -> Self::Fetch<'_w> {
        todo!()
    }

    const IS_DENSE: bool = true;

    unsafe fn set_archetype<'_w>(
        fetch: &mut Self::Fetch<'_w>,
        state: &Self::State,
        archetype: &'_w Archetype,
        table: &'_w Table,
    ) {
        todo!()
    }

    unsafe fn set_table<'_w>(fetch: &mut Self::Fetch<'_w>, state: &Self::State, table: &'_w Table) {
        todo!()
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess<ComponentId>) {
        todo!()
    }

    fn init_state(world: &mut World) -> Self::State {
        todo!()
    }

    fn get_state(components: &Components) -> Option<Self::State> {
        todo!()
    }

    fn matches_component_set(
        state: &Self::State,
        set_contains_id: &impl Fn(ComponentId) -> bool,
    ) -> bool {
        todo!()
    }
}

unsafe impl<'w, 's, T: Component> QueryData for Entry<'w, 's, T> {
    const IS_READ_ONLY: bool = false;

    type ReadOnly = Option<Read<T>>; //TODO: need to add readonly entry variant that works with 

    type Item<'a> = Entry<'a, 'a, T>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }

    unsafe fn fetch<'_w>(
        fetch: &mut Self::Fetch<'_w>,
        entity: Entity,
        table_row: TableRow,
    ) -> Self::Item<'_w> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
