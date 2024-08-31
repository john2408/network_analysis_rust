use petgraph::visit::EdgeRef;
use serde::Deserialize;
use std::collections::{HashMap,HashSet};
use std::fs::File;
use csv::Reader;
use rustworkx_core::Result;
use rustworkx_core::petgraph;
use rustworkx_core::centrality::eigenvector_centrality;
use petgraph::graph::{UnGraph, NodeIndex};


#[derive(Debug, Deserialize)]
struct Movie {
    Director: String,
    Star1: String,
    Star2: Option<String>,
    Star3: Option<String>,
    Star4: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {


    //Parameters 
    let tolerance: f64 = 1e-6;
    let max_iter: usize = 1000;
    let input_path: &str = "input_data/imdb_top_1000.csv";

    // Read the CSV file
    let file = File::open(input_path)?;
    let mut rdr = Reader::from_reader(file);

    // Store pair counts
    let mut pair_counts: HashMap<(String, String), usize> = HashMap::new();

    // Iterate over each row in the CSV
    for result in rdr.deserialize() {
        let movie: Movie = result?;
        
        // Collect collaborators into a vector
        let mut collaborators = vec![movie.Director, movie.Star1];
        
        if let Some(star) = movie.Star2 {
            collaborators.push(star);
        }
        if let Some(star) = movie.Star3 {
            collaborators.push(star);
        }
        if let Some(star) = movie.Star4 {
            collaborators.push(star);
        }

        // Remove duplicates for Director and Start by converting to a HashSet and back to a Vec
        let collaborators: Vec<String> = HashSet::<String>::from_iter(collaborators.into_iter()).into_iter().collect();

        // Generate all unique tuples (excluding self-pairs)
        for i in 0..collaborators.len() {
            for j in i + 1..collaborators.len() {
                let pair = if collaborators[i] < collaborators[j] {
                    (collaborators[i].clone(), collaborators[j].clone())
                } else {
                    (collaborators[j].clone(), collaborators[i].clone())
                };
                *pair_counts.entry(pair).or_insert(0) += 1;
            }
        }
    }

    // Print the unique elements of pair_counts along with their values
    // for (pair, count) in pair_counts.iter() {
    //     println!("{:?} -> {}", pair, count);
    // }

    // // Convert HashMap to Vec of tuples (String, String, usize) = (PersonA, PersonB,)
    // let edge_list: Vec<(String, String, usize)> = pair_counts.iter()
    // .map(|((start, end), weight)| (start.clone(), end.clone(), *weight))
    // .collect();
    
    // // Create the graph using petgraph
    // // let g = UnGraph::<String, usize>::from_edges(edge_list.iter().map(|(start, end, weight)| (start.clone(), end.clone(), *weight)));
    // let g = petgraph::graph::UnGraph::<i32, ()>::new_undirected();
    // // Add nodes and edges to the graph
    // let mut node_indices = HashMap::new();

    // for ((start, end), weight) in pair_counts {
    //     let start_index = *node_indices.entry(start.clone()).or_insert_with(|| g.add_node(start));
    //     let end_index = *node_indices.entry(end.clone()).or_insert_with(|| g.add_node(end));
    //     g.add_edge(start_index, end_index, weight);
    // }
       
    // Create a graph
    let mut g = UnGraph::<String, usize>::new_undirected();
    
    // Add nodes and edges to the graph
    let mut node_indices = HashMap::new();
    
    for ((start, end), weight) in pair_counts {
        let start_index = *node_indices.entry(start.clone()).or_insert_with(|| g.add_node(start));
        let end_index = *node_indices.entry(end.clone()).or_insert_with(|| g.add_node(end));
        g.add_edge(start_index, end_index, weight);
    }
    
    //// Example of using the graph
    // for node in g.node_indices() {
    //     println!("Node {:?}: {:?}", node.index(), g[node]);
    // }
    
    // for edge in g.edge_indices() {
    //     let (source, target) = g.edge_endpoints(edge).unwrap();
    //     println!(
    //         "Edge from {:?} to {:?} with weight: {}",
    //         g[source],
    //         g[target],
    //         g[edge]
    //     );
    // }

    // Calculate the eigenvector centrality
    println!("Eigenvector Centrality with Self-made Function: ");

    // Map node indices to their names
    let index_to_name: HashMap<_, _> = g
    .node_indices()
    .map(|index| (index.index(), g[index].clone()))
    .collect();

    // Step 5: Calculate Eigenvector Centrality
    let eigenvector_centrality_values = calculate_eigenvector_centrality(&g, max_iter, tolerance);

    // Convert HashMap to a vector of (node, centrality) tuples and sort by centrality values
    let mut sorted_centrality: Vec<_> = eigenvector_centrality_values.iter().collect();
    sorted_centrality.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    // Print the top 5 values
    for (node, score) in sorted_centrality.iter().take(5) {
        let index_as_f64 = node.index() as usize;
        println!("Node {:?}: Centrality = {:.6}", index_to_name.get(&index_as_f64).unwrap(), score);
    }

    // -------------------------------------------------------------------------------
    // Implemenation with Rustworkx
    // REf: https://docs.rs/rustworkx-core/0.15.1/rustworkx_core/centrality/fn.eigenvector_centrality.html
    // Collect scores into a vector of formatted strings
    // -------------------------------------------------------------------------------

    
    let centrality_scores: Result<Option<Vec<f64>>> = eigenvector_centrality(&g, |edge_ref| Ok(*edge_ref.weight() as f64), Some(max_iter), Some(tolerance));

    println!("\nEigenvector Centrality with Rustworkx:");
    match centrality_scores {
        Ok(Some(scores)) => {
            // Create a mapping from NodeIndex to f64 centrality score
            let mut node_scores: HashMap<NodeIndex, f64> = HashMap::new();
            for (node, score) in g.node_indices().zip(scores.iter()) {
                node_scores.insert(node, *score);
            }

            // Sort the centrality scores in descending order
            let mut sorted_scores: Vec<_> = node_scores.into_iter().collect();
            sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Print the top 5 scores
            for (i, (node, score)) in sorted_scores.into_iter().take(5).enumerate() {
                println!("Rank {}: Node {:?} has centrality score {:.6}", i + 1, g[node], score);
            }
        },
        Ok(None) => {
            eprintln!("Centrality computation did not return any results.");
        },
        Err(e) => {
            eprintln!("Error computing centrality: {}", e);
        }
    }

    Ok(())

}



fn calculate_eigenvector_centrality(
    graph: &UnGraph<String, usize>,
    max_iter: usize,
    tolerance: f64
) -> HashMap<NodeIndex, f64> {

    let mut centrality = HashMap::new();
    let mut norm_factor = 0.0;

    // Initialize centrality scores to 1.0
    for node in graph.node_indices() {
        centrality.insert(node, 1.0);
    }

    // Iterate to approximate the eigenvector centrality
    for _ in 0..max_iter {
        let mut temp_centrality = centrality.clone();
        norm_factor = 0.0;

        for node in graph.node_indices() {
            let mut sum = 0.0;
            for edge in graph.edges(node) {
                let target_node = edge.target();
                sum += centrality[&target_node] as f64 * *edge.weight() as f64;
            }
            temp_centrality.insert(node, sum);
            norm_factor += sum.powi(2);
        }

        norm_factor = norm_factor.sqrt();
        if norm_factor != 0.0 {
            for (node, value) in temp_centrality.iter_mut() {
                *value /= norm_factor;
            }
        }

        // Check for convergence
        let mut max_diff = 0.0;
        for (node, value) in temp_centrality.iter() {
            let old_value = centrality[node];
            let diff = (old_value - value).abs();
            if diff > max_diff {
                max_diff = diff;
            }
        }

        centrality = temp_centrality;

        if max_diff < tolerance {
            break;
        }
    }

    centrality
}
