use std::fmt::Debug;
use std::hash::Hash;
use bevy::prelude::Component;

pub trait PlannerState: Clone + Eq + Hash + Default + Component + Debug {}