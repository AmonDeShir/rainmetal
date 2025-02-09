use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Instant;
use bevy::prelude::*;
use bevy::tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task};
use bevy::utils::HashMap;
use crate::action::{Action, InsertableActionComponent};
use crate::goal::{DistanceCalculator, Goal};
use crate::state::PlannerState;
use pathfinding;

#[derive(Component)]
pub struct Planner<S: PlannerState> {
    pub all_goals: Vec<Goal<S>>,
    pub all_actions: Vec<Action<S>>,
    pub actions_map: HashMap<String, (Action<S>, Arc<dyn InsertableActionComponent + Send + Sync>)>,
    pub action: Option<Action<S>>,
    pub goal: Option<Goal<S>>,
    pub plan: VecDeque<String>,
}

impl<S: PlannerState> Planner<S> {
    pub fn sort_goals(&mut self, state: &S) {
        self.all_goals.sort_by(|a, b| (b.priority)(&state).cmp(&(a.priority)(&state)));
    }

    pub fn is_current_action(&self, action: &Action<S>) -> bool {
        let Some(current)  = &self.action else {
            return false
        };

       current.key == action.key
    }

    pub fn new(goals: Vec<Goal<S>>, actions: Vec<(Arc<dyn InsertableActionComponent + Send + Sync>, Action<S>)>) -> Self {
        let mut all_actions = vec![];
        let mut actions_map = HashMap::new();

        for (component, action) in actions {
            all_actions.push(action.clone());
            actions_map.insert(action.key.clone(), (action, component));
        }

        Self {
            all_goals: goals,
            all_actions,
            actions_map,
            action: None,
            goal: None,
            plan: VecDeque::new(),
        }
    }
}

pub fn init_planner<S: PlannerState>(app: &mut App) {
    app.add_systems(Update, (create_planner_tasks::<S>, handle_planner_tasks::<S>).chain());
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Node<S: PlannerState> {
    Action(String, S),
    State(S),
}

impl<S: PlannerState> Node<S> {
    pub fn state(&self) -> &S {
        match self {
            Node::Action(_, state) => state,
            Node::State(state) => state,
        }
    }
}

fn successors<'s, S: PlannerState>(node: &'s Node<S>, actions: &'s Vec<Action<S>>) -> impl Iterator<Item = (Node<S>, usize)> + 's {
    let state = node.state();

    actions.iter().filter_map(move |action| {
        if (action.preconditions)(state) {
            let value = Node::Action(
                action.key.clone(),
                (action.effect)(state.clone())
            );

            Some((value, (action.cost)(state)))
        }
        else {
            None
        }
    })
}

fn heuristic<S: PlannerState>(node: &Node<S>, goal: &Goal<S>) -> usize {
    (goal.distance)(node.state(), DistanceCalculator::new()).value
}

fn is_goal<S: PlannerState>(node: &Node<S>, goal: &Goal<S>) -> bool {
    (goal.requirements)(node.state())
}

pub fn make_plan<S: PlannerState>(start: &S, actions: &Vec<Action<S>>, goal: &Goal<S>) -> Option<(Vec<Node<S>>, usize)> {
    let start = Node::State(start.clone());

    pathfinding::directed::astar::astar(
        &start,
        |node| successors(node, actions).collect::<Vec<_>>().into_iter(),
        |node| heuristic(node, goal),
        |node| is_goal(node, goal),
    )
}

#[derive(Component)]
pub struct ComputePlan<S: PlannerState>(Task<(Option<(Vec<Node<S>>, usize)>, Option<Goal<S>>)>);

#[derive(Component)]
pub struct IsPlanning;

pub fn create_planner_tasks<S: PlannerState>(mut commands: Commands, mut query: Query<(Entity, &mut Planner<S>, &S), Without<ComputePlan<S>>>) {
    let thread_pool = AsyncComputeTaskPool::get();


    for (entity, mut planner, state) in query.iter_mut() {
        planner.sort_goals(state);

        let state = state.clone();
        let actions = planner.all_actions.clone();
        let goals = planner.all_goals.clone();

        let task = thread_pool.spawn(async move {
            let mut plan = None;
            let mut current_goal = None;

            for goal in goals.iter() {
                let start = Instant::now();

                if (goal.requirements)(&state) {
                    continue;
                }

                plan = make_plan(&state, &actions, &goal);
                let duration = start.elapsed().as_millis();
                let steps = plan.iter().len();

                if duration > 10 {
                    warn!("Planning duration for Entity {entity} was {duration}ms for {steps} steps");
                }

                if plan.iter().len() > 0 {
                    current_goal = Some(goal.clone());
                    break;
                }
            }

            (plan, current_goal)
        });

        commands.entity(entity).insert((ComputePlan(task), IsPlanning));
    }
}

pub fn handle_planner_tasks<S: PlannerState>(mut commands: Commands, mut query: Query<(Entity, &mut ComputePlan<S>, &mut Planner<S>)>) {
    for (entity, mut task, mut planner) in query.iter_mut() {
        let (plan, goal) = match block_on(future::poll_once(&mut task.0)) {
            Some(result) => result,
            None => {
                continue
            }
        };

        planner.goal = goal;
        commands.entity(entity).remove::<ComputePlan<S>>();

        let Some(plan) = plan else {
            continue
        };

        let new_plan = get_action_names_from_plan(&plan.0);

        if planner.plan != new_plan {
            planner.plan = new_plan.clone();
        }

        let Some(action_key) = new_plan.front() else {
            continue
        };


        let Some((action, action_component)) = planner.actions_map.get(&action_key.clone()) else {
            panic!("Didn't find action {:?} registered in the Planner::actions_map", action_key)
        };

        let new_action = action.clone();

        if planner.action.is_some() && !planner.is_current_action(action) {
            // remove all components
            for (_, (_, component)) in planner.actions_map.iter() {
                component.remove(&mut commands, entity);
            }
        }

        action_component.insert(&mut commands, entity);
        planner.action = Some(new_action);
        commands.entity(entity).remove::<IsPlanning>();
    }
}

pub fn get_action_names_from_plan<S: PlannerState>(plan: &Vec<Node<S>>) -> VecDeque<String> {
    let mut nodes = VecDeque::new();

    for node in plan {
        match node {
            Node::Action(name, _) => nodes.push_back(name.clone()),
            Node::State(_s) => {}
        }
    }

    nodes
}