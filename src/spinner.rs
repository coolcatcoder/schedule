use bevy::prelude::*;

use crate::query_data;

struct Spinner;

// I am using these to test that my my macro still works when used normally.
query_data!(Spinner, &, (BackgroundGradient));
query_data!(Spinner, &mut, (BackgroundGradient));

fn weird(spinner: Single<&Spinner>) {
    //let bad = &*query;
}
