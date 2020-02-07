extern crate petgraph;

#[cfg(feature = "evcxr")]
pub mod evcxr;

pub mod pmzzs;

use petgraph::data::*;
use petgraph::visit::*;

use std::collections::HashMap;

pub type Dimension = i64;

/*
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

*/
