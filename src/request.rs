use libccanvas::{bindings::Discriminator, features::common::Direction};
use serde::Deserialize;

use crate::{Border, Constraint, Layout};

#[derive(Deserialize)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[serde(tag = "type")]
pub enum LayoutRequest {
    #[serde(rename = "add")]
    Add {
        at: Vec<Direction>,
        split: Direction,
        constraint_1: Constraint,
        constraint_2: Constraint,
        component: Option<Discriminator>,
        border: Option<Border>,
    },
    #[serde(rename = "remove")]
    Remove { at: Vec<Direction> },
    #[serde(rename = "setlayout")]
    SetLayout { at: Vec<Direction>, layout: Layout },
}
