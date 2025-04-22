/// src/chronograph.rs
///
/// # Author
/// Yifan Jiang, 2025
/// alvinjiang01@hotmail.com
/// GitHub: [AlvinJ404](https://github.com/AlvinJ404/chronograph)

use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

type NodeId = usize;
type Timestamp = u64;

#[derive(Debug, Clone)]
pub struct ChronoGraph {
    edges: HashMap<NodeId, Vec<(NodeId, Timestamp)>>,
    nodes: HashSet<NodeId>,
}

impl ChronoGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashSet::new(),
        }
    }

    pub fn get_nodes(&self) -> &HashSet<NodeId> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &HashMap<NodeId, Vec<(NodeId, Timestamp)>> {
        &self.edges
    }

    pub fn add_node(&mut self, node_id: NodeId) {
        self.nodes.insert(node_id);
    }

    pub fn add_edge(&mut self, src: NodeId, dst: NodeId, timestamp: Timestamp) -> Result<(), String> {
        if !self.nodes.contains(&src) || !self.nodes.contains(&dst) {
            return Err("Source or destination node does not exist.".into());
        }
        self.edges.entry(src).or_insert_with(Vec::new).push((dst, timestamp));
        Ok(())
    }

    pub fn get_neighbors_at(&self, node: NodeId, timestamp: Timestamp) -> Vec<NodeId> {
        self.edges
            .get(&node)
            .map(|neighbors| {
                neighbors
                    .par_iter() // rayon parallel iterator
                    .filter_map(|(dst, ts)| {
                        if *ts <= timestamp {
                            Some(*dst)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn print(&self) {
        for node in &self.nodes {
            println!("Node {}:", node);
            if let Some(edges) = self.edges.get(node) {
                for (dst, ts) in edges {
                    println!("  -> {} @ {}", dst, ts);
                }
            } else {
                println!("  (no outgoing edges)");
            }
        }
    }
}