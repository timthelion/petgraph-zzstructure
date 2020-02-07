use petgraph_evcxr::draw_dot;
use petgraph::*;
use petgraph::graph::Graph;

use crate::*;

pub fn draw_zzstructure<'a, W>(zzs: &'a Graph<W, Dimension, petgraph::Directed>)
where
    W: fmt::Display,
{
    let stacks = zzs.filter_map(
        |_, n| Some(n.clone()),
        |_, e| match e {
            Dimension::EW => None,
            Dimension::NS => Some(Dimension::NS),
        },
    );
    let mut dot_stacks: Vec<String> = vec![];
    let mut dot_edges: Vec<String> = vec![];
    let mut stack_heads: HashMap<petgraph::graph::NodeIndex, petgraph::graph::NodeIndex> =
        HashMap::new();

    for nr in stacks.node_references() {
        if !stacks
            .edges_directed(nr.id(), Direction::Incoming)
            .next()
            .is_some()
        {
            let mut stack_items: Vec<String> = vec![];
            let mut cur = nr.id();
            loop {
                stack_items.push(format!(
                    "\t\t\t<TR><TD PORT=\"{}\">{}</TD></TR>\n",
                    cur.index(),
                    stacks.node_weight(cur).unwrap(),
                ));
                stack_heads.insert(cur, nr.id());
                if let Some(er) = stacks.edges_directed(cur, Direction::Outgoing).next() {
                    cur = er.target();
                } else {
                    break;
                }
            }
            let dot_stack: String = stack_items.iter().map(|x| x.clone()).collect();
            dot_stacks.push(
            format!(
                "\t{} [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">\n{}\n\t\t</TABLE>>];\n",
                nr.id().index(),
                dot_stack,
            )
        );
        }
    }

    for (k, v) in stack_heads.clone().into_iter() {
        for er in zzs.edges(k) {
            if *er.weight() == Dimension::EW {
                dot_edges.push(format!(
                    "\t{}:{} -> {}:{}\n",
                    v.index(),
                    k.index(),
                    stack_heads.get(&er.target()).unwrap().index(),
                    er.target().index(),
                ));
            }
        }
    }

    let dot_stacks_string: String = dot_stacks.iter().map(|x| x.clone()).collect();
    let dot_edges_string: String = dot_edges.iter().map(|x| x.clone()).collect();
    let dot = format!(
        "digraph G {{
    node [shape=plaintext]
    rankdir=LR;
    {}
    {}
}}",
        dot_stacks_string, dot_edges_string,
    );
    draw_dot(dot);
}
