use super::Expression;
use std::rc::Rc;

#[derive(Default, Debug, Eq, PartialEq)]
pub struct State {
    transitions: Vec<Transition>,
}

impl State {
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    pub fn add_transition(&mut self, transition: Transition) -> &mut Self {
        self.transitions.push(transition);
        self
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Transition {
    conditions: Vec<Rc<Expression>>,
    effects: Vec<Effect>,
    next: Option<Rc<Expression>>,
}

impl Transition {
    pub fn conditions(&self) -> &[Rc<Expression>] {
        &self.conditions
    }

    pub fn add_condition(&mut self, condition: Rc<Expression>) -> &mut Self {
        self.conditions.push(condition);
        self
    }

    pub fn effects(&self) -> Vec<&Effect> {
        self.effects.iter().rev().collect()
    }

    pub fn add_effect(&mut self, effect: Effect) -> &mut Self {
        self.effects.push(effect);
        self
    }

    pub fn next(&self) -> Option<Rc<Expression>> {
        self.next.clone()
    }

    pub fn set_next(&mut self, next: Rc<Expression>) -> &mut Self {
        self.next = Some(next);
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Effect {
    Flip,
    Scale(Rc<Expression>),
    Spawn(Rc<State>),
    Withdraw,
}
