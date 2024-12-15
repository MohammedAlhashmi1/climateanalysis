mod graph;

use crate::graph::Graph;

fn main() {
    // Path 
    let dataset_path = "GlobalLandTemperaturesByCity.csv";
    println!("Loading dataset from: {}", dataset_path);

    // Creating the graph
    let mut graph = Graph::new();
    if let Err(err) = graph.load_from_csv(dataset_path) {
        eprintln!("Failed to load dataset: {}", err);
        return;
    }

    // Graph analysis
    println!("Dataset loaded successfully. Analyzing graph...");
    graph.analyze();

    println!("Graph analysis complete!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn test_graph_loading() {
        let mut graph = Graph::new();
        assert!(graph.load_from_csv("test_data.csv").is_ok());
        assert!(graph.node_count() > 0);
    }

    #[test]
    fn test_graph_analysis() {
        let mut graph = Graph::new();
        graph.load_from_csv("test_data.csv").unwrap();
        assert!(graph.degree_distribution().len() > 0);
        assert!(graph.centrality().len() > 0);
    }
}
