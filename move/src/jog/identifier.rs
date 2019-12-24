use std::fmt::{self, Display, Formatter};

const PREFIX: &str = "s";
const SPAWN: &str = "spawn";

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Identifier<'a> {
    Raw(&'a str),
    Prefixed(&'a str),
    Spawn(usize),
    Transition(usize, usize),
    AbstractTransition(&'a str, usize, usize),
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Raw(name) => name.fmt(f),
            Self::Prefixed(name) => write!(f, "{}_{}", PREFIX, name),
            Self::Spawn(id) => write!(f, "{}_{}", SPAWN, id),
            Self::Transition(from, to) => write!(f, "transition_{}_{}", from, to),
            Self::AbstractTransition(name, from, to) => {
                write!(f, "{}_{}_{}_{}", PREFIX, name, from, to)
            }
        }
    }
}