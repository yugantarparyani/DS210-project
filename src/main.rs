mod graph;
mod community;
mod analysis;

use graph::create_graph;
use community::{label_propagation, map_community_names, print_community_sizes};
use analysis::{
    analyze_inter_community_links, calculate_community_density,
    analyze_top_densest_communities, identify_brokers_in_densest_communities,
    sentiment_analysis_intra_community, sentiment_analysis,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "soc-redditHyperlinks-body.tsv";
    let max_edges = 50000;
    let remove_isolated = true;

    //Create the graph
    let (graph, _node_indices) = create_graph(file_path, max_edges, remove_isolated)?;
    println!(
        "Graph created with {} nodes and {} edges.",
        graph.node_count(),
        graph.edge_count()
    );

    //Perform community detection
    let max_iterations = 20;
    let communities = label_propagation(&graph, max_iterations);
    println!("Community detection completed.");

    //Map community names
    let community_names = map_community_names(&graph, &communities);

    //Print community sizes
    print_community_sizes(&communities);

    //Analyze inter-community links
    analyze_inter_community_links(&graph, &communities, &community_names);

    //Calculate community densities
    let densities = calculate_community_density(&graph, &communities);

    //Analyze top densest communities
    analyze_top_densest_communities(&graph, &communities, &densities, &community_names, 10);

    //Identify brokers in top densest communities
    identify_brokers_in_densest_communities(&graph, &communities, &densities, 10);

    //Perform sentiment analysis for intra-community links
    sentiment_analysis_intra_community(&graph, &communities, &densities, &community_names, 10);

    sentiment_analysis(&graph, &communities, 11);  // dogecoin
    sentiment_analysis(&graph, &communities, 224); // gaming
    sentiment_analysis(&graph, &communities, 122); // subredditdrama

    Ok(())
}
//hello world