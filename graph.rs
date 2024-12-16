use csv::ReaderBuilder;
use petgraph::graph::{DiGraph, NodeIndex};
use std::error::Error;

pub fn create_graph(
    file_path: &str,
    max_edges: usize,
    remove_isolated: bool,
) -> Result<(DiGraph<String, i32>, std::collections::HashMap<String, usize>), Box<dyn Error>> {
    let mut graph = DiGraph::<String, i32>::new();
    let mut node_indices = std::collections::HashMap::new();
    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_path(file_path)?;

    let mut edge_count = 0;
    for result in reader.records() {
        if edge_count >= max_edges {
            break;
        }
        let record = result?;
        let source = record[0].to_string();
        let target = record[1].to_string();
        let label: i32 = record[4].parse()?;

        let source_index = *node_indices
            .entry(source.clone())
            .or_insert_with(|| graph.add_node(source.clone()).index());
        let target_index = *node_indices
            .entry(target.clone())
            .or_insert_with(|| graph.add_node(target.clone()).index());

        graph.add_edge(NodeIndex::new(source_index), NodeIndex::new(target_index), label);
        edge_count += 1;
    }

    if remove_isolated {
        remove_isolated_nodes(&mut graph);
    }

    Ok((graph, node_indices))
}

fn remove_isolated_nodes(graph: &mut DiGraph<String, i32>) {
    let nodes_to_keep: Vec<_> = graph
        .node_indices()
        .filter(|&node| graph.neighbors(node).count() > 0)
        .collect();

    graph.retain_nodes(|_, node| nodes_to_keep.contains(&node));
}
//hello world