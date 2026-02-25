pub use crate::SimpleQueryData;
use bevy::ecs::query::QueryData;

pub trait SimpleQueryData<const MUT: bool> {
    type Fetch: QueryData;
    type Item<'w>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort>;

    fn fetch<'w, 's>(fetch: <Self::Fetch as QueryData>::Item<'w, 's>) -> Self::Item<'w>;
}

#[macro_export]
macro_rules! SimpleQueryData {
    derive() ($(#[$_:meta])* $__:vis struct $name:ident <$($(const $const_generic_ident:ident:)? $generic_type:ty),*> $($___:tt)*) => {
        $crate::query_data::SimpleQueryData!(|1: Process Generics|
            remaining_generics_to_check($($(const $const_generic_ident:)? $generic_type,)*),
            self_type($name<),
            impl_generics(<$($(const $const_generic_ident:)? $generic_type,)*>),
        );
    };
    derive() ($(#[$_:meta])* $__:vis struct $name:ident $($___:tt)*) => {
        $crate::query_data::SimpleQueryData!(|1: Process Generics|
            remaining_generics_to_check(),
            self_type($name<),
            impl_generics(),
        );
    };

    (|helper: If|
        if true {
            $($when_true:tt)*
        } else {
            $($when_false:tt)*
        }
    ) => {
        $($when_true)*
    };
    (|helper: If|
        if false {
            $($when_true:tt)*
        } else {
            $($when_false:tt)*
        }
    ) => {
        $($when_false)*
    };

    (|1: Process Generics|
        remaining_generics_to_check(const $generic_ident:ident: $generic_type:ty, $($remaining_generics_to_check:tt)*),
        self_type($($self_type:tt)*),
        impl_generics($($impl_generics:tt)*),
    ) => {
        $crate::query_data::SimpleQueryData!(|1: Process Generics|
            remaining_generics_to_check($($remaining_generics_to_check)*),
            self_type($($self_type)* $generic_ident,),
            impl_generics($($impl_generics)*),
        );
    };

    (|1: Process Generics|
        remaining_generics_to_check(),
        self_type($($self_type:tt)*),
        impl_generics($($impl_generics:tt)*),
    ) => {
        $crate::query_data::SimpleQueryData!(|2: Implementation|
            self_type($($self_type)*>),
            is_mut(false),
            impl_generics($($impl_generics)*),
        );

        $crate::query_data::SimpleQueryData!(|2: Implementation|
            self_type($($self_type)*>),
            is_mut(true),
            impl_generics($($impl_generics)*),
        );
    };

    (|2: Implementation|
        self_type($self_type:ty),
        is_mut($is_mut:tt),
        impl_generics($($impl_generics:tt)*),
    ) => {
        #[allow(unused_parens)]
        const _: () = {
            use bevy::ecs::query::{WorldQuery, QueryData};
            use bevy::prelude::*;

            $crate::query_data::SimpleQueryData! {
                |helper: If|
                if $is_mut {

                } else {
                    unsafe impl$($impl_generics)* bevy::ecs::query::ReadOnlyQueryData for $crate::query_data::SimpleQueryData! {
                        |helper: If|
                        if $is_mut {
                            &mut $self_type
                        } else {
                            &$self_type
                        }
                    } {}
                }
            }

            unsafe impl$($impl_generics)* WorldQuery for $crate::query_data::SimpleQueryData! {
                |helper: If|
                if $is_mut {
                    &mut $self_type
                } else {
                    &$self_type
                }
            } {
                const IS_DENSE: bool = <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::IS_DENSE;

                type Fetch<'w> = <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::Fetch<'w>;
                type State = <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::State;

                fn shrink_fetch<'wlong: 'wshort, 'wshort>(fetch: Self::Fetch<'wlong>) -> Self::Fetch<'wshort> {
                    fetch
                }

                unsafe fn init_fetch<'w>(
                        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                        state: &Self::State,
                        last_run: bevy::ecs::change_detection::Tick,
                        this_run: bevy::ecs::change_detection::Tick,
                    ) -> Self::Fetch<'w> {
                        unsafe { <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::init_fetch(world, state, last_run, this_run) }
                }

                unsafe fn set_archetype<'w>(
                    fetch: &mut Self::Fetch<'w>,
                    state: &Self::State,
                    archetype: &'w bevy::ecs::archetype::Archetype,
                    table: &'w bevy::ecs::storage::Table,
                    ) {
                    unsafe { <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::set_archetype(fetch, state, archetype, table) }
                }

                unsafe fn set_table<'w>(
                    fetch: &mut Self::Fetch<'w>,
                    state: &Self::State,
                    table: &'w bevy::ecs::storage::Table,
                ) {
                    unsafe { <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::set_table(fetch, state, table) }
                }

                fn update_component_access(state: &Self::State, access: &mut bevy::ecs::query::FilteredAccess) {
                    <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::update_component_access(state, access)
                }

                fn init_state(world: &mut World) -> Self::State {
                    <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::init_state(world)
                }

                fn get_state(components: &bevy::ecs::component::Components) -> Option<Self::State> {
                    <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::get_state(components)
                }

                fn matches_component_set(
                        state: &Self::State,
                        set_contains_id: &impl Fn(bevy::ecs::component::ComponentId) -> bool,
                    ) -> bool {
                    <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as WorldQuery>::matches_component_set(state, set_contains_id)
                }
            }

            unsafe impl$($impl_generics)* QueryData for $crate::query_data::SimpleQueryData! {
                |helper: If|
                if $is_mut {
                    &mut $self_type
                } else {
                    &$self_type
                }
            } {
                const IS_READ_ONLY: bool = $is_mut;
                const IS_ARCHETYPAL: bool = <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as QueryData>::IS_ARCHETYPAL;

                type ReadOnly = $crate::query_data::SimpleQueryData! {
                    |helper: If|
                    if $is_mut {
                        &'static $self_type
                    } else {
                        Self
                    }
                };
                type Item<'w, 's> = <$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Item<'w>;

                fn shrink<'wlong: 'wshort, 'wshort, 's>(
                    item: Self::Item<'wlong, 's>,
                ) -> Self::Item<'wshort, 's> {
                    <$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::shrink(item)
                }

                unsafe fn fetch<'w, 's>(
                    state: &'s Self::State,
                    fetch: &mut Self::Fetch<'w>,
                    entity: Entity,
                    table_row: bevy::ecs::storage::TableRow,
                ) -> Option<Self::Item<'w, 's>> {
                    let fetch = unsafe { <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as QueryData>::fetch(state, fetch, entity, table_row) };
                    fetch.map(|fetch| <$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::fetch(fetch))
                }

                fn iter_access(state: &Self::State) -> impl Iterator<Item = bevy::ecs::query::EcsAccessType<'_>> {
                    <<$self_type as $crate::query_data::SimpleQueryData<$is_mut>>::Fetch as QueryData>::iter_access(state)
                }
            }
        };
    };
}
