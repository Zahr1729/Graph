use std::fmt;

use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Weight(pub usize);

impl fmt::Debug for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

/// Use this trait if the node/edge should have be weighted.
pub trait Weighted {
    fn get_weight(&self) -> &Weight;
    fn set_weight(&mut self, weight: Weight);
}