use super::{
    super::{expression::Expression, variable::Variable},
    Action,
};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Scale<'a> {
    scalar: Expression<'a>,
}

impl<'a> Scale<'a> {
    pub fn new(scalar: Expression<'a>) -> Self {
        Scale { scalar }
    }
}

impl Action for Scale<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![]
    }
}

impl Display for Scale<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "*(&mut copy(context_ref).scale) = *(&copy(context_ref).scale) * {}",
            self.scalar
        )
    }
}
