use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::connected_components;
use petgraph::Undirected;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub struct GraphAnalyzer {
    pub graph: Graph<(), (), Undirected>,
}

impl GraphAnalyzer {
    pub fn from_csv(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let mut node_map: HashMap<usize, NodeIndex> = HashMap::new();

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            if i == 0 {
                continue; 
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

    pub fn degree_distribution(&self) -> HashMap<usize, usize> {
        let mut degree_counts: HashMap<usize, usize> = HashMap::new();
        for node in self.graph.node_indices() {
            let degree = self.graph.neighbors(node).count();
            *degree_counts.entry(degree).or_insert(0) += 1;
        }
        degree_counts
    }

    pub fn save_distribution(&self, output_path: &str) -> Result<(), Box<dyn Error>> {
        let degree_counts = self.degree_distribution();
        let mut file = File::create(output_path)?;
        writeln!(file, "degree,count")?;
        for (degree, count) in degree_counts {
            writeln!(file, "{},{}", degree, count)?;
        }
        Ok(())
    }

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

    pub fn count_connected_components(&self) -> usize {
        connected_components(&self.graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_loading() {
        let result = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv");
        assert!(result.is_ok());
        let analyzer = result.unwrap();
        assert!(analyzer.graph.node_count() > 0);
    }

    #[test]
    fn test_degree_distribution() {
        let analyzer = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv").unwrap();
        let dist = analyzer.degree_distribution();
        assert!(!dist.is_empty());
    }

    #[test]
    fn test_average_degree() {
        let analyzer = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv").unwrap();
        let avg = analyzer.average_degree();
        assert!(avg > 0.0);
    }

    #[test]
    fn test_connected_components() {
        let analyzer = GraphAnalyzer::from_csv("facebook_large/musae_facebook_edges.csv").unwrap();
        let comps = analyzer.count_connected_components();
        assert!(comps >= 1);
    }
}
