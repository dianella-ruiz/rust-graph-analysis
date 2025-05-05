use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::connected_components;
use petgraph::Undirected;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};


//Struct to encapsulate graph and analysis logic 
pub struct GraphAnalyzer {
    pub graph: Graph<(), (), Undirected>,
}

impl GraphAnalyzer {
    
    //Loads the graph from a CSV file of edges 
    //each line contains a source and targte node 
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let mut node_map: HashMap<usize, NodeIndex> = HashMap::new();
        
        //skip header
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if i == 0 {
                continue; //skip header
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 2 {
                continue;
            }

            let source: usize = parts[0].parse()?;
            let target: usize = parts[1].parse()?;

            let source_index = *node_map.entry(source).or_insert_with(|| graph.add_node(()));
            let target_index = *node_map.entry(target).or_insert_with(|| graph.add_node(()));

            graph.add_edge(source_index, target_index, ());
        }

        Ok(GraphAnalyzer { graph })
    }

    //computes degree distribution, how many nodes have each degree 
    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut degree_counts: HashMap<usize, usize> = HashMap::new();
        for node in self.graph.node_indices() {
            let degree = self.graph.neighbors(node).count();
            *degree_counts.entry(degree).or_insert(0) += 1;
        }
        degree_counts
    }

    //saves the degree distribution to a CSV file 
    pub fn save_distribution(&self, output_path: &str) -> Result<(), Box<dyn Error>> {
        let degree_counts = self.degree_distribution();
        let mut file = File::create(output_path)?;
        writeln!(file, "degree,count")?;
        for (degree, count) in degree_counts {
            writeln!(file, "{},{}", degree, count)?;
        }
        Ok(())
    }

    //computes average degree of all nodes 
    pub fn average_degree(&self) -> f64 {
        let degree_counts = self.degree_distribution();
        let total_nodes: usize = degree_counts.values().sum();
        let total_degrees: usize = degree_counts.iter().map(|(deg, count)| deg * count).sum();

        if total_nodes == 0 {
            0.0
        } else {
            total_degrees as f64 / total_nodes as f64
        }
    }

    //counts number of connected components in the graph 
    pub fn count_connected_components(&self) -> usize {
        connected_components(&self.graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    //verifies that the large facebook graph dataset is loaded properly from CSV input 
    #[test]
    fn test_graph_loading() {
        let result = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv");
        assert!(result.is_ok());
        let analyzer = result.unwrap();
        assert!(analyzer.graph.node_count() > 0);
    }

    //check if degree distribution is computed as expected (has to exist) 
    #[test]
    fn test_degree_distribution() {
        let analyzer = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv").unwrap();
        let dist = analyzer.degree_distribution();
        assert!(!dist.is_empty());
    }

    //checks average degree calculation is positive
    #[test]
    fn test_average_degree() {
        let analyzer = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv").unwrap();
        let avg = analyzer.average_degree();
        assert!(avg > 0.0);
    }

    //ensures connected_components returns at least 1 connected component 
    #[test]
    fn test_connected_components() {
        let analyzer = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv").unwrap();
        let comps = analyzer.count_connected_components();
        assert!(comps >= 1);
    }
}
