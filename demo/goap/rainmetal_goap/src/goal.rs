use std::sync::Arc;
use crate::distance::Distance;
use crate::state::PlannerState;

#[derive(Clone)]
pub struct Goal<S: PlannerState> {
    pub key: String,
    pub priority: Arc<dyn Fn(&S) -> usize + Send + Sync>,
    pub distance: Arc<dyn Fn(&S, DistanceCalculator) -> DistanceCalculator + Send + Sync>,
    pub requirements: Arc<dyn Fn(&S) -> bool + Send + Sync>,
}

impl<S: PlannerState> Goal<S> {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            priority: Arc::new(|_| 0),
            distance: Arc::new(|_, d| d),
            requirements: Arc::new(|_| true),
        }
    }

    pub fn with_requirement(mut self, requirement: Arc<dyn Fn(&S) -> bool + Send + Sync>) -> Self {
        self.requirements = requirement;
        self
    }

    pub fn with_distance(mut self, distance: Arc<dyn Fn(&S, DistanceCalculator) -> DistanceCalculator + Send + Sync>) -> Self {
        self.distance = distance;
        self
    }

    pub fn with_priority(mut self, priority: Arc<dyn Fn(&S) -> usize + Send + Sync>) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_static_priority(mut self, priority: usize) -> Self {
        self.priority = Arc::new(move |_| priority);
        self
    }
}

pub struct DistanceCalculator {
    pub value: usize
}

impl DistanceCalculator {
    pub fn new() -> DistanceCalculator {
        DistanceCalculator { value: 0 }
    }

    pub fn add<T: Distance>(mut self, a: &T , b: &T) -> Self {
        self.value += a.distance(b);
        self
    }

    pub fn add_eq<T: PartialEq>(mut self, a: &T, b: &T) -> Self {
        if a != b {
            self.value += 1
        }

        self
    }
}