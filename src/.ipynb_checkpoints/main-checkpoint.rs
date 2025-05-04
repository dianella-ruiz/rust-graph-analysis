mod graph;
use graph::GraphAnalyzer;

fn main() {
    let path = "facebook_large/musae_facebook_edges.csv";

    let analyzer = match GraphAnalyzer::from_csv(path) {
        Ok(g) => {
            println!(
                "Graph loaded with {} nodes and {} edges",
                g.graph.node_count(),
                g.graph.edge_count()
            );
            g
        }
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
            return;
        }
    };

    let degree_counts = analyzer.degree_distribution();
    println!("Degree Distribution:");
    for (degree, count) in &degree_counts {
        println!("Degree {}: {} nodes", degree, count);
    }

    if let Err(e) = analyzer.save_distribution("degree_distribution.csv") {
        eprintln!("Failed to save degree distribution: {}", e);
    } else {
        println!("Saved degree distribution to file.");
    }

    let avg_deg = analyzer.average_degree();
    println!("Average node degree: {:.2}", avg_deg);

    let components = analyzer.count_connected_components();
    println!("The graph has {} connected components.", components);
    
}
