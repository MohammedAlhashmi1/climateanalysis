use std::collections::{HashMap, HashSet};
use csv::Reader;
use std::io;

pub struct Graph {
    adjacency_list: HashMap<String, HashSet<String>>, // Node connections
    weights: HashMap<(String, String), f64>,          // Edge weights
    nodes: HashSet<String>,                           // All nodes
}

impl Graph {
    // Create a new, empty graph
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            weights: HashMap::new(),
            nodes: HashSet::new(),
        }
    }

    // Load data from a CSV file and construct the graph
    pub fn load_from_csv(&mut self, file_path: &str) -> io::Result<()> {
        let mut reader = Reader::from_path(file_path)?;
        let mut city_data: HashMap<String, Vec<f64>> = HashMap::new();

        // Debug: Track record count
        let mut record_count = 0;

        // Read the CSV file row by row
        for result in reader.records() {
            record_count += 1;

            let record = result?;
            let city = record.get(3).unwrap_or("").to_string();
            let avg_temp: f64 = record
                .get(1)
                .unwrap_or("0")
                .parse()
                .unwrap_or(0.0);

            city_data.entry(city.clone()).or_insert_with(Vec::new).push(avg_temp);
            self.nodes.insert(city);

            // Debug output for every 1M rows
            if record_count % 1_000_000 == 0 {
                println!("Processed {} records...", record_count);
            }
        }

        println!("Finished processing {} records. (2-5 minutes, depending on hardware 😊)", record_count);

        // Build edges based on temperature similarity
        for (city_a, temps_a) in &city_data {
            for (city_b, temps_b) in &city_data {
                if city_a != city_b && Graph::are_in_same_country(city_a, city_b) {
                    let correlation = Self::calculate_similarity(temps_a, temps_b);
                    if correlation > 0.8 {
                        self.add_edge(city_a.clone(), city_b.clone(), correlation);
                    }
                }
            }
        }

        Ok(())
    }

    // Check if two cities are in the same country (example optimization)
    fn are_in_same_country(_city_a: &str, _city_b: &str) -> bool {
        // Placeholder logic; update when the dataset includes country info
        true
    }

    // Add an edge to the graph
    pub fn add_edge(&mut self, node_a: String, node_b: String, weight: f64) {
        self.adjacency_list
            .entry(node_a.clone())
            .or_insert_with(HashSet::new)
            .insert(node_b.clone());

        self.adjacency_list
            .entry(node_b.clone())
            .or_insert_with(HashSet::new)
            .insert(node_a.clone());

        self.weights.insert((node_a, node_b), weight);
    }

    // Analyze the graph
    pub fn analyze(&self) {
        println!("Analyzing graph...");

        // Degree distribution
        let degree_dist = self.degree_distribution();
        println!("Degree Distribution: {:?}", degree_dist);

        // Centrality measures
        let centrality = self.centrality();
        println!("Top 5 Central Nodes: {:?}", centrality);
    }

    // Calculate degree distribution
    pub fn degree_distribution(&self) -> Vec<usize> {
        self.adjacency_list.values().map(|neighbors| neighbors.len()).collect()
    }

    // Compute centrality (degree-based)
    pub fn centrality(&self) -> Vec<(String, usize)> {
        let mut centrality_scores: Vec<(String, usize)> = self
            .adjacency_list
            .iter()
            .map(|(node, neighbors)| (node.clone(), neighbors.len()))
            .collect();

        centrality_scores.sort_by(|a, b| b.1.cmp(&a.1));
        centrality_scores.into_iter().take(5).collect()
    }

    // Helper to calculate similarity between temperature vectors
    fn calculate_similarity(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> f64 {
        let mean_a: f64 = vec_a.iter().copied().sum::<f64>() / vec_a.len() as f64;
        let mean_b: f64 = vec_b.iter().copied().sum::<f64>() / vec_b.len() as f64;

        let numerator: f64 = vec_a
            .iter()
            .zip(vec_b.iter())
            .map(|(a, b)| (a - mean_a) * (b - mean_b))
            .sum();

        let denominator_a: f64 = vec_a.iter().map(|a| (a - mean_a).powi(2)).sum::<f64>().sqrt();
        let denominator_b: f64 = vec_b.iter().map(|b| (b - mean_b).powi(2)).sum::<f64>().sqrt();

        if denominator_a == 0.0 || denominator_b == 0.0 {
            return 0.0;
        }

        numerator / (denominator_a * denominator_b)
    }
}
