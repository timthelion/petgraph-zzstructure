extern crate petgraph;

#[cfg(feature = "evcxr")]
pub mod evcxr;

pub mod pmzzs;

use petgraph::data::*;
use petgraph::visit::*;

use std::collections::HashMap;

pub type Dimension = usize;
