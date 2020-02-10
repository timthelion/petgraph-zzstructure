use petgraph_evcxr::draw_dot;
use petgraph::*;
use petgraph::graph::Graph;

use std::fmt;

use crate::*;

pub fn zzstructure_to_dot<'a, W>(zzs: &'a Graph<W, Dimension, petgraph::Directed>, vertical: Dimension, horizontal: Dimension) -> String
where
    W: fmt::Display,
{
    let stacks = zzs.filter_map(
        |_, n| Some(n.clone()),
        |_, e| match e {
            _ if *e == horizontal => None,
            _ if *e == vertical => Some(vertical),
            _ => None,
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
            if *er.weight() == horizontal {
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
    dot
}

pub fn draw_zzstructure<'a, W>(zzs: &'a Graph<W, Dimension, petgraph::Directed>, vertical: Dimension, horizontal: Dimension)
where
    W: fmt::Display,
{
    draw_dot(zzstructure_to_dot(zzs, vertical, horizontal));
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::pmzzs::*;
    use petgraph_examples as examples;

    #[test]
    fn test_conversions() {
        let dwc = examples::directed_graph_with_cycle();
        let dwc_zz = to_pmzzs(&dwc, "".to_string(), 0, 1);
        let output = zzstructure_to_dot(&dwc_zz, 0, 1);
        assert!(output.contains("digraph G {\n    node [shape=plaintext]\n    rankdir=LR;\n    \t7 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">\n\t\t\t<TR><TD PORT=\"7\"></TD></TR>\n\t\t\t<TR><TD PORT=\"1\">b</TD></TR>\n\t\t\t<TR><TD PORT=\"10\"></TD></TR>\n\t\t\t<TR><TD PORT=\"12\"></TD></TR>\n\n\t\t</TABLE>>];\n\t9 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">\n\t\t\t<TR><TD PORT=\"9\"></TD></TR>\n\t\t\t<TR><TD PORT=\"2\">c</TD></TR>\n\n\t\t</TABLE>>];\n\t11 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">\n\t\t\t<TR><TD PORT=\"11\"></TD></TR>\n\t\t\t<TR><TD PORT=\"3\">d</TD></TR>\n\t\t\t<TR><TD PORT=\"14\"></TD></TR>\n\n\t\t</TABLE>>];\n\t13 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">\n\t\t\t<TR><TD PORT=\"13\"></TD></TR>\n\t\t\t<TR><TD PORT=\"4\">e</TD></TR>\n\t\t\t<TR><TD PORT=\"16\"></TD></TR>\n\n\t\t</TABLE>>];\n\t17 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">\n\t\t\t<TR><TD PORT=\"17\"></TD></TR>\n\t\t\t<TR><TD PORT=\"15\"></TD></TR>\n\t\t\t<TR><TD PORT=\"5\">f</TD></TR>\n\t\t\t<TR><TD PORT=\"18\"></TD></TR>\n\n\t\t</TABLE>>];\n\t19 [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\">\n\t\t\t<TR><TD PORT=\"19\"></TD></TR>\n\t\t\t<TR><TD PORT=\"0\">a</TD></TR>\n\t\t\t<TR><TD PORT=\"6\"></TD></TR>\n\t\t\t<TR><TD PORT=\"8\"></TD></TR>\n\n\t\t</TABLE>>];\n\n"));
        assert!(output.contains("\t7:10 -> 11:11"));
        assert!(output.contains("\t7:12 -> 13:13"));
        assert!(output.contains("\t17:18 -> 19:19"));
        assert!(output.contains("\t19:6 -> 7:7"));
        assert!(output.contains("\t19:8 -> 9:9"));
        assert!(output.contains("\t11:14 -> 17:15"));
        assert!(output.contains("\t13:16 -> 17:17"));
    }
}
