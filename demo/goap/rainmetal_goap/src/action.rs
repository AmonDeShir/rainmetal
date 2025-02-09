use crate::state::PlannerState;
use bevy::prelude::{Commands, Entity};
use std::sync::Arc;

#[derive(Clone)]
pub enum ActionState {
    INIT,
    PROGRESS,
    SUCCESS,
    FAILED,
}

impl Default for ActionState {
    fn default() -> Self {
        ActionState::INIT
    }
}


#[derive(Clone)]
pub struct Action<S: PlannerState> {
    pub key: String,
    pub cost: Arc<dyn Fn(&S) -> usize + Send + Sync>,
    pub effect: Arc<dyn Fn(S) -> S + Send + Sync>,
    pub preconditions: Arc<dyn Fn(&S) -> bool + Send + Sync>,
}

impl<S: PlannerState> Action<S> {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            cost: Arc::new(|_| 0),
            effect: Arc::new(|s| s),
            preconditions: Arc::new(|_| false),
        }
    }

    pub fn with_effect(mut self, effect: Arc<dyn Fn(S) -> S + Send + Sync>) -> Self {
        self.effect = effect;
        self
    }

    pub fn with_precondition(mut self, precondition: Arc<dyn Fn(&S) -> bool + Send + Sync>) -> Self {
        self.preconditions = precondition;
        self
    }

    pub fn with_cost(mut self, cost: Arc<dyn Fn(&S) -> usize + Send + Sync>) -> Self {
        self.cost = cost;
        self
    }

    pub fn with_static_cost(mut self, cost: usize) -> Self {
        self.cost = Arc::new(move |_| cost);
        self
    }
}

pub trait ActionComponent {
    fn new<S: PlannerState>() -> Action<S>;

    fn get_state(&self) -> &ActionState;
}

pub trait InsertableActionComponent {
    fn insert(&self, commands: &mut Commands, entity_to_insert_to: Entity);
    fn remove(&self, commands: &mut Commands, entity_to_remove_from: Entity);
}