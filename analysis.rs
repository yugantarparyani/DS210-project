use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, HashSet};
use petgraph::visit::EdgeRef;

pub fn calculate_community_density(
    graph: &DiGraph<String, i32>,
    communities: &HashMap<usize, usize>,
) -> Vec<(usize, f64)> {
    let mut community_edges = HashMap::new();
    let mut community_nodes = HashMap::new();

    for edge in graph.edge_references() {
        let (source, target) = (edge.source(), edge.target());
        let source_comm = communities[&source.index()];
        let target_comm = communities[&target.index()];

        if source_comm == target_comm {
            *community_edges.entry(source_comm).or_insert(0) += 1;
        }
    }

    for node in graph.node_indices() {
        let community = communities[&node.index()];
        *community_nodes.entry(community).or_insert(0) += 1;
    }

    let mut densities = community_edges
        .iter()
        .map(|(&comm, &edges)| {
            let nodes = community_nodes[&comm];
            (comm, edges as f64 / nodes as f64)
        })
        .collect::<Vec<_>>();

    densities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    densities
}

pub fn analyze_inter_community_links(
    graph: &DiGraph<String, i32>,
    communities: &HashMap<usize, usize>,
    community_names: &HashMap<usize, String>,
) {
    let mut inter_community_counts: HashMap<(usize, usize), usize> = HashMap::new();

    for edge in graph.edge_references() {
        let (source, target) = (edge.source(), edge.target());
        let source_comm = communities[&source.index()];
        let target_comm = communities[&target.index()];

        if source_comm != target_comm {
            let key = if source_comm < target_comm {
                (source_comm, target_comm)
            } else {
                (target_comm, source_comm)
            };
            *inter_community_counts.entry(key).or_insert(0) += 1;
        }
    }

    let mut inter_community_pairs: Vec<_> = inter_community_counts.into_iter().collect();
    inter_community_pairs.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top Inter-Community Connections:");
    let unknown_name = "Unknown".to_string();
    for ((comm1, comm2), count) in inter_community_pairs.iter().take(10) {
        let comm1_name = community_names.get(comm1).unwrap_or(&unknown_name);
        let comm2_name = community_names.get(comm2).unwrap_or(&unknown_name);
        println!(
            "Community {} ({}) ↔ Community {} ({}): {} edges",
            comm1, comm1_name, comm2, comm2_name, count
        );
    }
}

pub fn sentiment_analysis_intra_community(
    graph: &DiGraph<String, i32>,
    communities: &HashMap<usize, usize>,
    densities: &[(usize, f64)],
    community_names: &HashMap<usize, String>,
    top_n: usize,
) {
    println!(
        "Sentiment analysis for intra-community links in the top {} densest communities:",
        top_n
    );
    let top_communities: HashSet<usize> = densities.iter().take(top_n).map(|(comm, _)| *comm).collect();
    let unknown = "Unknown".to_string();

    for &community in &top_communities {
        let mut positive_count = 0;
        let mut negative_count = 0;

        for edge in graph.edge_references() {
            let (source, target) = (edge.source(), edge.target());
            let source_comm = communities[&source.index()];
            let target_comm = communities[&target.index()];

            if source_comm == community && target_comm == community {
                let sentiment = edge.weight();
                if *sentiment > 0 {
                    positive_count += 1;
                } else if *sentiment < 0 {
                    negative_count += 1;
                }
            }
        }

        let community_name = community_names.get(&community).unwrap_or(&unknown);
        println!(
            "Community {} ({}): Positive Sentiment Links: {}, Negative Sentiment Links: {}",
            community, community_name, positive_count, negative_count
        );
    }
}

pub fn analyze_top_densest_communities(
    graph: &DiGraph<String, i32>,
    communities: &HashMap<usize, usize>,
    densities: &[(usize, f64)],
    community_names: &HashMap<usize, String>,
    top_n: usize,
) {
    println!("Top {} Densest Communities:", top_n);
    for (community_id, density) in densities.iter().take(top_n) {
        let unknown_name = "Unknown".to_string();
        let community_name = community_names.get(community_id).unwrap_or(&unknown_name);
        println!("Community {} ({}): Density {:.2}", community_id, community_name, density);

        let mut community_nodes = vec![];
        for node in graph.node_indices() {
            if communities[&node.index()] == *community_id {
                community_nodes.push(graph[node].clone());
            }
        }

        println!(
            "Community {} ({}) contains {} nodes: {:?}",
            community_id,
            community_name,
            community_nodes.len(),
            community_nodes
        );
    }
}

pub fn identify_brokers_in_densest_communities(
    graph: &DiGraph<String, i32>,
    communities: &HashMap<usize, usize>,
    densities: &[(usize, f64)],
    top_n: usize,
) {
    println!("Identifying brokers in the top {} densest communities:", top_n);
    let top_communities: HashSet<usize> = densities.iter().take(top_n).map(|(comm, _)| *comm).collect();

    for &community in &top_communities {
        let mut brokers = HashSet::new();

        for edge in graph.edge_references() {
            let (source, target) = (edge.source(), edge.target());
            let source_comm = communities[&source.index()];
            let target_comm = communities[&target.index()];

            if source_comm == community && target_comm != community {
                brokers.insert(source.index());
            } else if target_comm == community && source_comm != community {
                brokers.insert(target.index());
            }
        }

        println!(
            "Community {}: {} brokers identified",
            community,
            brokers.len()
        );
        let broker_names: Vec<String> = brokers
            .iter()
            .map(|&broker| graph[NodeIndex::new(broker)].clone())
            .collect();
        println!("Brokers: {:?}", broker_names);
    }
}

pub fn sentiment_analysis(
    graph: &DiGraph<String, i32>,
    communities: &HashMap<usize, usize>,
    broker_node: usize,
) {
    let mut positive_count = 0;
    let mut negative_count = 0;

    for edge in graph.edges(NodeIndex::new(broker_node)) {
        let (source, target) = (edge.source(), edge.target());
        let source_comm = communities[&source.index()];
        let target_comm = communities[&target.index()];
        let sentiment = edge.weight();

        if source_comm != target_comm {
            if *sentiment > 0 {
                positive_count += 1;
            } else if *sentiment < 0 {
                negative_count += 1;
            }
        }
    }

    let broker_name = &graph[NodeIndex::new(broker_node)];
    println!(
        "Sentiment Analysis for Broker Subreddit: {} (Node: {})",
        broker_name, broker_node
    );
    println!("Positive Sentiment Links: {}", positive_count);
    println!("Negative Sentiment Links: {}", negative_count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::DiGraph;

    #[test]
    fn test_calculate_community_density() {
        let mut graph = DiGraph::<String, i32>::new();
        let node1 = graph.add_node("node1".to_string());
        let node2 = graph.add_node("node2".to_string());
        graph.add_edge(node1, node2, 1);

        let mut communities = HashMap::new();
        communities.insert(0, 1);
        communities.insert(1, 1);

        let densities = calculate_community_density(&graph, &communities);
        assert_eq!(densities.len(), 1, "There should be one community density calculated");
    }

    #[test]
    fn test_sentiment_analysis_intra_community() {
        let mut graph = DiGraph::<String, i32>::new();
        let node1 = graph.add_node("node1".to_string());
        let node2 = graph.add_node("node2".to_string());
        graph.add_edge(node1, node2, 1);

        let mut communities = HashMap::new();
        communities.insert(0, 1);
        communities.insert(1, 1);

        let densities = vec![(1, 0.5)];
        let community_names = HashMap::new();

        sentiment_analysis_intra_community(&graph, &communities, &densities, &community_names, 1);
    }
}

//hello world