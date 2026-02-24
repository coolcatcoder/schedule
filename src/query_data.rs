#[macro_export]
macro_rules! query_data {
    (if_mut mut {$($mut:tt)*} else {$($not_mut:tt)*}) => {
        $($mut)*
    };

    (if_mut {$($mut:tt)*} else {$($not_mut:tt)*}) => {
        $($not_mut)*
    };

    ($name:ident, &mut, ($($field_type:ty),*)) => {
        $crate::query_data!(|internal| $name, &mut, <($(&'static mut $field_type),*) as QueryData>::Item<'w, 's>, ($(&'static mut $field_type),*));
    };
    ($name:ident, &, ($($field_type:ty),*)) => {
        $crate::query_data!(|internal| $name, &, <($(&'static $field_type),*) as QueryData>::Item<'w, 's>, ($(&'static $field_type),*));
    };

    (|internal| $name:ident, &$($ref:ident)?, $item:ty, $tuple:ty) => {
        #[allow(unused_parens)]
        const _: () = {
            use bevy::ecs::query::{WorldQuery, QueryData};
            use bevy::prelude::*;

            $crate::query_data! {
                if_mut $($ref)? {

                } else {
                    unsafe impl bevy::ecs::query::ReadOnlyQueryData for &$name {}
                }
            }

            unsafe impl WorldQuery for &$($ref)?$name {
                const IS_DENSE: bool = <$tuple as WorldQuery>::IS_DENSE;

                type Fetch<'w> = <$tuple as WorldQuery>::Fetch<'w>;
                type State = <$tuple as WorldQuery>::State;

                fn shrink_fetch<'wlong: 'wshort, 'wshort>(fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
                    fetch
                }

                unsafe fn init_fetch<'w>(
                        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                        state: &Self::State,
                        last_run: bevy::ecs::change_detection::Tick,
                        this_run: bevy::ecs::change_detection::Tick,
                    ) -> Self::Fetch<'w> {
                        unsafe { <$tuple as WorldQuery>::init_fetch(world, state, last_run, this_run) }
                }

                unsafe fn set_archetype<'w>(
                    fetch: &mut Self::Fetch<'w>,
                    state: &Self::State,
                    archetype: &'w bevy::ecs::archetype::Archetype,
                    table: &'w bevy::ecs::storage::Table,
                    ) {
                    unsafe { <$tuple as WorldQuery>::set_archetype(fetch, state, archetype, table) }
                }

                unsafe fn set_table<'w>(
                    fetch: &mut Self::Fetch<'w>,
                    state: &Self::State,
                    table: &'w bevy::ecs::storage::Table,
                ) {
                    unsafe { <$tuple as WorldQuery>::set_table(fetch, state, table) }
                }

                fn update_component_access(state: &Self::State, access: &mut bevy::ecs::query::FilteredAccess) {
                    <$tuple as WorldQuery>::update_component_access(state, access)
                }

                fn init_state(world: &mut World) -> Self::State {
                    <$tuple as WorldQuery>::init_state(world)
                }

                fn get_state(components: &bevy::ecs::component::Components) -> Option<Self::State> {
                    <$tuple as WorldQuery>::get_state(components)
                }

                fn matches_component_set(
                        state: &Self::State,
                        set_contains_id: &impl Fn(bevy::ecs::component::ComponentId) -> bool,
                    ) -> bool {
                    <$tuple as WorldQuery>::matches_component_set(state, set_contains_id)
                }
            }

            unsafe impl QueryData for &$($ref)?$name {
                const IS_READ_ONLY: bool = $crate::query_data! {
                    if_mut $($ref)? {
                        false
                    } else {
                        true
                    }
                };
                const IS_ARCHETYPAL: bool = <$tuple as QueryData>::IS_ARCHETYPAL;

                type ReadOnly = $crate::query_data! {
                    if_mut $($ref)? {
                        &'static $name
                    } else {
                        Self
                    }
                };
                type Item<'w, 's> = $item;

                fn shrink<'wlong: 'wshort, 'wshort, 's>(
                    item: Self::Item<'wlong, 's>,
                ) -> Self::Item<'wshort, 's> {
                    Self::Item::<'wshort, 's>::from(item)
                }

                unsafe fn fetch<'w, 's>(
                    state: &'s Self::State,
                    fetch: &mut Self::Fetch<'w>,
                    entity: Entity,
                    table_row: bevy::ecs::storage::TableRow,
                ) -> Option<Self::Item<'w, 's>> {
                    let fetch = unsafe { <$tuple as QueryData>::fetch(state, fetch, entity, table_row) };
                    fetch.map(|fetch| <$item as From<_>>::from(fetch))
                }

                fn iter_access(state: &Self::State) -> impl Iterator<Item = bevy::ecs::query::EcsAccessType<'_>> {
                    <$tuple as QueryData>::iter_access(state)
                }
            }
        };
    };

    (mut, ($($field_type:ty),*)) => {
        $(
            &'static mut $field_type
        ),*
    };

    (, ($($field_type:ty),*)) => {
        $(
            &'static $field_type
        ),*
    };
}
