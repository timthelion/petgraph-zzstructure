use petgraph::*;
use petgraph::graph::Graph;

use crate::*;

pub fn to_pmzzs<'a, G, W, E, N>(g: &'a G, blank: W, vertical: Dimension, horizontal: Dimension) -> Graph<W, Dimension, petgraph::Directed>
where
    W: Clone,
    &'a G: IntoEdgeReferences
        + IntoNodeReferences
        + Data<NodeWeight = W, EdgeWeight = W>
        + GraphBase<EdgeId = E, NodeId = N>,
    G: Build,
    N: Eq + Copy + std::hash::Hash,
    E: Eq + Copy,
    W: Clone,
{
    let mut last_outgoing_port = HashMap::new();
    let mut last_incomming_port = HashMap::new();
    let mut node_map = HashMap::new();
    let mut zzg: Graph<W, Dimension, petgraph::Directed> = Graph::new();
    for nr in g.node_references() {
        let new_id = zzg.add_node(nr.weight().clone());
        node_map.insert(nr.id(), new_id);
    }
    for er in g.edge_references() {
        let outgoing = zzg.add_node(er.weight().clone());
        let incomming = zzg.add_node(blank.clone());
        zzg.add_edge(outgoing, incomming, horizontal);
        // Connect outgoing port list
        match last_outgoing_port.get(&er.source()) {
            Some(prev) => {
                zzg.add_edge(*prev, outgoing, vertical);
            }
            None => {
                zzg.add_edge(*node_map.get(&er.source()).unwrap(), outgoing, vertical);
            }
        }
        last_outgoing_port.insert(er.source(), outgoing);
        // Connect incomming port list
        match last_incomming_port.get(node_map.get(&er.target()).unwrap()) {
            Some(prev) => {
                zzg.add_edge(incomming, *prev, vertical);
            }
            None => {
                zzg.add_edge(incomming, *node_map.get(&er.target()).unwrap(), vertical);
            }
        }
        last_incomming_port.insert(*node_map.get(&er.target()).unwrap(), outgoing);
    }
    zzg
}

pub fn from_pmzzs<'a, W>(zzg: &'a Graph<W, Dimension, petgraph::Directed>, vertical: Dimension, horizontal: Dimension) -> Graph<W, W, petgraph::Directed>
where
 W: PartialEq + Clone,
{
    #[derive(Debug, PartialEq, Clone)]
    enum NType {
        OutgoingEdge,
        IncomingEdge,
        Vertex,
    }
    let mut gfz: Graph<W, W, petgraph::Directed> = Graph::new();
    let mut node_map = HashMap::new();
    let mut incoming_edges: Vec<petgraph::prelude::NodeIndex> = Vec::new();
    let mut outgoing_edges: Vec<petgraph::prelude::NodeIndex> = Vec::new();
    for nr in zzg.node_references() {
        let mut ntype = NType::Vertex;
        for edge in zzg.edges(nr.id()) {
            if *edge.weight() == horizontal {
                ntype = NType::OutgoingEdge;
            }
        }
        for edge in zzg.edges_directed(nr.id(), Incoming) {
            if *edge.weight() == horizontal {
                ntype = NType::IncomingEdge;
            }
        }
        //println!("{:?}", ntype);
        match ntype {
            NType::OutgoingEdge => {
                outgoing_edges.push(nr.id());
            },
            NType::IncomingEdge => {
                incoming_edges.push(nr.id());
            },
            NType::Vertex => {
                node_map.insert(nr.id(), gfz.add_node(nr.weight().clone()));
            },
        }
    }
    let mut outgoing_edge_map = HashMap::new();
    for (edge, node) in outgoing_edges.iter().map(|edge| {
            let mut node = *edge;
            while !node_map.contains_key(&node) {
                for e in zzg.edges_directed(node, Incoming) {
                    if *e.weight() == vertical {
                        node = e.source();
                    }
                }
            }
            (edge, node)
        }
    ) {
        outgoing_edge_map.insert(edge, node);
    }
    let mut incoming_edge_map = HashMap::new();
    for (edge, node) in incoming_edges.iter().map(|edge| {
            let mut node = *edge;
            while !node_map.contains_key(&node) {
                for e in zzg.edges_directed(node, Outgoing) {
                    if *e.weight() == vertical {
                        node = e.target();
                    }
                }
            }
            (edge, node)
        }
    ) {
        incoming_edge_map.insert(edge, node);
    }
    for (edge, source) in outgoing_edge_map {
        for e in zzg.edges_directed(*edge, Outgoing) {
            if *e.weight() == horizontal {
                gfz.add_edge(
                    source,
                    *incoming_edge_map.get(&e.target()).unwrap(),
                    zzg.node_weight(*edge).unwrap().clone(),
                );
            }
        }
    }
    gfz
}
