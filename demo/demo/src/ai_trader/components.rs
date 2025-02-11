use crate::memory::Memory;
use bevy::prelude::*;
use rainmetal_goap::prelude::{OrderedFloat, OrderedVec3};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::hash::Hash;

#[derive(Component, Default)]
#[require(TradingPlans, TradingPlanByValue)]
pub struct AiTrader;

#[derive(Clone, Debug)]
pub struct TradingPlan {
    pub from: Entity,
    pub to: Entity,
    pub item: String,
    pub distance: f64,
    pub start_position: Vec3,
    pub profit: f64,
    pub count: u64,
    pub capital_required: u64,
    pub buy_price: u64,
    pub sell_price: u64,
    pub last_updated: f32,
}

impl Hash for TradingPlan {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
        self.item.hash(state);
        OrderedFloat(self.distance).hash(state);
        OrderedVec3(self.start_position).hash(state);
        OrderedFloat(self.profit).hash(state);
        self.count.hash(state);
        self.capital_required.hash(state);
        self.sell_price.hash(state);
        self.buy_price.hash(state);

    }
}

impl PartialEq for TradingPlan {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from &&
        self.to == other.to &&
        self.item == other.item &&
        OrderedFloat(self.distance) == OrderedFloat(other.distance) &&
        OrderedVec3(self.start_position) == OrderedVec3(other.start_position) &&
        OrderedFloat(self.profit) == OrderedFloat(other.profit) &&
        self.count == other.count &&
        self.capital_required == other.capital_required &&
        self.sell_price == other.sell_price &&
        self.buy_price == other.buy_price
    }
}

impl  Eq for TradingPlan {}

impl Ord for TradingPlan {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().partial_cmp(&other.value()).expect("TradingPlan.value returned NaN, which is not supported by Ord::cmp!")
    }
}

impl PartialOrd for TradingPlan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TradingPlan {
    pub fn value(&self) -> f64 {
        self.profit / self.distance
    }
}

#[derive(Component)]
#[require(Memory)]
pub struct TradingPlans(pub BinaryHeap<TradingPlan>);

#[derive(Component, Default)]
#[require(TradingPlans)]
pub struct TradingPlanByValue {
    pub small: Option<TradingPlan>,
    pub medium: Option<TradingPlan>,
    pub big: Option<TradingPlan>,
    pub huge: Option<TradingPlan>,
}

impl TradingPlanByValue {
    pub fn all_set(&self) -> bool {
        vec![&self.small, &self.medium, &self.big, &self.huge]
            .iter()
            .all(|plan| plan.is_some())
    }

    pub fn all_none(&self) -> bool {
        vec![&self.small, &self.medium, &self.big, &self.huge]
            .iter()
            .all(|plan| plan.is_none())
    }

    pub fn set_all_to_none(&mut self) {
        self.small = None;
        self.medium = None;
        self.big = None;
        self.huge = None;
    }
}

impl Default for TradingPlans {
    fn default() -> Self {
        Self(BinaryHeap::new())
    }
}