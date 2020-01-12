extern crate petgraph;

#[cfg(feature = "evcxr")]
pub mod evcxr;

use petgraph::data::*;
use petgraph::visit::*;

pub struct StateMachine<'a, G, E, N, NW, EW, Action>
where
    G: GraphBase<EdgeId = E, NodeId = N> + Data<NodeWeight = NW, EdgeWeight = EW>,
    E: Copy + PartialEq,
    N: Copy + PartialEq,
{
    state_network: G,
    state: N,
    match_inputs: &'a dyn Fn(EW, EW) -> Option<Action>,
}

fn get_id_for_state<'a, G, NW, EW>(
    network: &'a G,
    state: NW,
) -> Option<<&'a G as GraphBase>::NodeId>
where
    &'a G: IntoNodeReferences + GraphBase + DataMap + Data<NodeWeight = NW, EdgeWeight = EW>,
    NW: PartialEq,
{
    for nr in network.node_references() {
        if *(network.node_weight(nr.id())).unwrap() == state {
            return Option::Some(nr.id());
        }
    }
    return None;
}

impl<'a, G, E, N, EW, NW, Action> StateMachine<'a, G, E, N, NW, EW, Action>
where
    G: Data<NodeWeight = NW, EdgeWeight = EW>
        + NodeIndexable
        + GraphProp
        + DataMap
        + GraphBase<EdgeId = E, NodeId = N>,
    E: Copy + PartialEq,
    N: Copy + PartialEq,
    for<'b> &'b G: IntoNodeReferences
        + IntoEdgeReferences
        + IntoEdges
        + Data<NodeWeight = NW, EdgeWeight = EW>
        + GraphBase<EdgeId = E, NodeId = N>,
    EW: PartialEq + Clone,
    NW: PartialEq + Clone,
{
    pub fn next<'c>(&'c mut self, input: EW) -> Option<(Action, NW)> {
        for edge in (&self.state_network).edges(self.state) {
            match (self.match_inputs)(edge.weight().clone(), input.clone()) {
                Some(matched_transition) => {
                    self.state = edge.target();
                    return match self.state_network.node_weight(self.state) {
                        Some(weight) => Some((matched_transition, weight.clone())),
                        None => None,
                    };
                }
                None => (),
            }
        }
        return None;
    }

    pub fn set_state<'c>(&'c mut self, state: NW) {
        get_id_for_state(&self.state_network, state).map(|id| self.state = id);
    }

    pub fn new(
        network: G,
        start: NW,
        match_inputs: &'a dyn Fn(EW, EW) -> Option<Action>,
    ) -> Option<
        StateMachine<'a, G, <G as GraphBase>::EdgeId, <G as GraphBase>::NodeId, NW, EW, Action>,
    > {
        get_id_for_state(&network, start).map(|id| StateMachine {
            state_network: network,
            state: id,
            match_inputs: match_inputs,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::*;

    #[test]
    fn test_transitions() {
        let mut sn: Graph<&str, u32, petgraph::Directed> = Graph::new();
        let sn_item1 = sn.add_node("a");
        let sn_item2 = sn.add_node("b");
        let sn_item3 = sn.add_node("c");
        let sn_item4 = sn.add_node("d");
        let sn_item5 = sn.add_node("e");
        sn.add_edge(sn_item1, sn_item2, 1);
        sn.add_edge(sn_item1, sn_item3, 2);
        sn.add_edge(sn_item2, sn_item4, 1);
        sn.add_edge(sn_item2, sn_item5, 2);
        sn.add_edge(sn_item5, sn_item1, 2);
        sn.add_edge(sn_item5, sn_item3, 1);
        let mut sm = StateMachine::new(sn, "a", &|ew1, ew2| {
            if ew1 == ew2 {
                Some(())
            } else {
                None
            }
        })
        .unwrap();
        assert_eq!(sm.next(1), Some(((), "b")));
        assert_eq!(sm.next(1), Some(((), "d")));
        sm.set_state("b");
        assert_eq!(sm.next(2), Some(((), "e")));
        assert_eq!(sm.next(2), Some(((), "a")));
        assert_eq!(sm.next(2), Some(((), "c")));
        assert_eq!(sm.next(2), None);
    }
}
