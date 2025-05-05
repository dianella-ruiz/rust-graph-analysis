//Description: Loads a large social network graph from CSV, computes degree distribution, average degree, and connected components 

mod graph;
use graph::GraphAnalyzer;

fn main() {
    //file path to dataset 
    let path = "facebook_large/musae_facebook_edges.csv";
//load the graph and print basic info 
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
    
    //compute and print degree distribution 
    let degree_counts = analyzer.degree_distribution();
    println!("Degree Distribution:");
    for (degree, count) in &degree_counts {
        println!("Degree {}: {} nodes", degree, count);
    }
    
    //Save distribution to file 
    if let Err(e) = analyzer.save_distribution("degree_distribution.csv") {
        eprintln!("Failed to save degree distribution: {}", e);
    } else {
        println!("Saved degree distribution to file.");
    }
    
    //Compute and display average degree
    let avg_deg = analyzer.average_degree();
    println!("Average node degree: {:.2}", avg_deg);

    //count and display number of connected components 
    let components = analyzer.count_connected_components();
    println!("The graph has {} connected components.", components);
    
}
