#[cfg(feature = "evcxr")]

use petgraph_evcxr::draw_graph_with_attr_getters;
use petgraph::visit::*;

use crate::*;

impl<'a, G, E, N, EW, NW, Transition> StateMachine<'a, G, E, N, NW, EW, Transition>
where
    G: NodeIndexable
    + GraphProp
    + GraphBase<EdgeId = E, NodeId = N>
    + Data<NodeWeight = NW, EdgeWeight = EW>,
    E: Copy + PartialEq,
    N: Copy + PartialEq,
for<'b> &'b G: IntoNodeReferences
    + IntoEdgeReferences
    + IntoEdges
    + GraphBase<EdgeId = E, NodeId = N>
    + Data<NodeWeight = NW, EdgeWeight = EW>,
    EW: std::fmt::Display,
    NW: std::fmt::Display,
{
    pub fn draw_evcxr(&self) {
        draw_graph_with_attr_getters(
            &self.state_network,
            &[],
            &|_, _| "".to_string(),
            &|_, nr| (if nr.id() == self.state { "shape = circle style = filled fillcolor = red" } else { "shape = circle" }).to_string(),
        );
    }
}
