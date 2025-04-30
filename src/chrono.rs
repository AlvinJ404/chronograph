/// src/chrono.rs
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
    num_threads: usize,
}

impl ChronoGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashSet::new(),
            num_threads: 1,
        }
    }

    pub fn new_with_threads(num_threads: usize) -> Self {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build_global()
            .expect("Failed to build thread pool");

        Self {
            edges: HashMap::new(),
            nodes: HashSet::new(),
            num_threads,
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

    pub fn remove_node(&mut self, node_id: NodeId) -> Result<NodeId, bool> {
        if !self.nodes.remove(&node_id) {
            return Err(false);
        }
    
        self.edges.remove(&node_id);
    
        for edges in self.edges.values_mut() {
            edges.retain(|(dst, _)| *dst != node_id);
        }
    
        Ok(node_id)
    }
    
    pub fn remove_edge(&mut self, src: NodeId, dst: NodeId, timestamp: Timestamp) -> Result<(NodeId, NodeId, Timestamp), bool> {
        if let Some(edge_list) = self.edges.get_mut(&src) {
            let before_len = edge_list.len();
            edge_list.retain(|(d, t)| !(*d == dst && *t == timestamp));
            if edge_list.len() < before_len {
                return Ok((src, dst, timestamp));
            }
        }
        Err(false)
    }    

    pub fn get_neighbors_at(&self, node: NodeId, timestamp: Timestamp) -> Vec<NodeId> {
        self.edges
            .get(&node)
            .map(|neighbors| {
                neighbors
                    .par_iter()
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