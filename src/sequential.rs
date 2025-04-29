/// src/sequential.rs
///
/// # Author
/// Yifan Jiang, 2025
/// alvinjiang01@hotmail.com
/// GitHub: [AlvinJ404](https://github.com/AlvinJ404/chronograph)

/// This module implements a simple temporal graph structure
/// and provides basic functionalities to add nodes and edges.

use std::collections::{HashMap, HashSet};

type NodeId = usize; // Unique identifier for a node in the graph
type Timestamp = u64; // TImestamp represented as a non-negative integer


/// A sequential temporal graph data struture where edges are
/// timestamped to indicate when they are active.
///
/// Nodes must be added using [`add_node`] before adding edges.
/// Edges must be added using [`add_edge`] and with pre-exisitng
/// source and destination nodes.
///
/// Time-sensitive neighbors are queried using [`get_neighbors_at`].
#[derive(Debug, Clone)]
pub struct TemporalGraph {
    edges: HashMap<NodeId, Vec<(NodeId, Timestamp)>>,
    nodes: HashSet<NodeId>,
}

impl TemporalGraph {
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
        self.edges
            .entry(src)
            .or_insert_with(Vec::new)
            .push((dst, timestamp));
        Ok(())
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> Result<NodeId, bool> {
        if !self.nodes.remove(&node_id) {
            return Err(false);
        }

        self.edges.remove(&node_id);
        
        for neighbors in self.edges.values_mut() {
            neighbors.retain(|(dst, _)| *dst != node_id);
        }

        Ok(node_id)
    }

    pub fn remove_edge(&mut self, src: NodeId, dst: NodeId, timestamp: Timestamp) -> Result<(NodeId, NodeId, Timestamp), bool> {
        if let Some(neighbors) = self.edges.get_mut(&src) {
            let before = neighbors.len();
            neighbors.retain(|(n, ts)| !(*n == dst && *ts == timestamp));
            if neighbors.len() < before {
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
                    .iter()
                    .filter_map(|(neighbor, ts)| {
                        if *ts <= timestamp {
                            Some(*neighbor)
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