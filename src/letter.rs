use std::fmt::Debug;
use std::hash::Hash;
pub trait Letter: Clone + Debug + Eq + Hash + PartialEq { }