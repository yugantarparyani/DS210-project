use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub fn label_propagation(
    graph: &DiGraph<String, i32>,
    max_iterations: usize,
) -> HashMap<usize, usize> {
    let mut communities: HashMap<usize, usize> = HashMap::new();

    for node in graph.node_indices() {
        communities.insert(node.index(), node.index());
    }

    for _ in 0..max_iterations {
        let mut updated = false;
        for node in graph.node_indices() {
            let mut neighbor_labels = HashMap::new();

            for neighbor in graph.neighbors(node) {
                let label = communities[&neighbor.index()];
                *neighbor_labels.entry(label).or_insert(0) += 1;
            }

            if let Some((&most_common_label, _)) =
                neighbor_labels.iter().max_by_key(|(_, &count)| count)
            {
                if communities[&node.index()] != most_common_label {
                    communities.insert(node.index(), most_common_label);
                    updated = true;
                }
            }
        }

        if !updated {
            break;
        }
    }

    communities
}

pub fn map_community_names(
    graph: &DiGraph<String, i32>,
    communities: &HashMap<usize, usize>,
) -> HashMap<usize, String> {
    let mut community_names = HashMap::new();

    for (node_index, &community) in communities.iter() {
        let node_name = graph[NodeIndex::new(*node_index)].clone();
        community_names
            .entry(community)
            .and_modify(|name: &mut String| {
                if node_name.len() > name.len() {
                    *name = node_name.clone();
                }
            })
            .or_insert(node_name);
    }

    community_names
}

pub fn print_community_sizes(communities: &HashMap<usize, usize>) {
    let mut community_sizes: HashMap<usize, usize> = HashMap::new();

    for &label in communities.values() {
        *community_sizes.entry(label).or_insert(0) += 1;
    }

    println!("Community sizes:");
    for (community, size) in community_sizes.iter().filter(|(_, &size)| size > 1) {
        println!("Community {}: {} nodes", community, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    #[test]
    fn test_label_propagation_small_graph() {
        let mut graph = DiGraph::<String, i32>::new();
        let node1 = graph.add_node("node1".to_string());
        let node2 = graph.add_node("node2".to_string());
        graph.add_edge(node1, node2, 1);

        let communities = label_propagation(&graph, 10);
        assert_eq!(communities.len(), 2, "Each node should form its own community");
    }

    #[test]
    fn test_map_community_names() {
        let mut graph = DiGraph::<String, i32>::new();
        let node1 = graph.add_node("node1".to_string());
        let node2 = graph.add_node("node2".to_string());
        graph.add_edge(node1, node2, 1);
    
        let mut communities = HashMap::new();
        communities.insert(node1.index(), 1); 
        communities.insert(node2.index(), 1);
    
        let names = map_community_names(&graph, &communities);
    
        println!("Community Names: {:?}", names);
        assert_eq!(
            names[&1],
            "node2",
            "Community name should be the longest node name"
        );
    }
}
//hello world