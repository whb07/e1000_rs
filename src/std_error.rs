use crate::lib::{Debug, Display};

pub trait Error: Debug + Display {
    /// The underlying cause of this error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
