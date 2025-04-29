/// src/lib.rs
///
/// # Author
/// Yifan Jiang, 2025
/// alvinjiang01@hotmail.com
/// GitHub: [AlvinJ404](https://github.com/AlvinJ404/chronograph)

pub mod sequential;
pub mod chrono;

#[cfg(test)]
mod sequential_unit_tests {
    use super::sequential::*;
    use std::time::Instant;

    #[test]
    fn test_add_node() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        assert!(tg.get_nodes().contains(&1));

        eprintln!("\nTG for test_add_node:");
        tg.print();
    }

    #[test]
    fn test_add_edge_success() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        tg.add_node(2);
        let result = tg.add_edge(1, 2, 5);
        assert!(result.is_ok());
        assert_eq!(tg.get_edges().get(&1).unwrap(), &vec![(2, 5)]);

        eprintln!("\nTG for test_add_edge_success:");
        tg.print();
    }

    #[test]
    fn test_add_edge_missing_node() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        let result = tg.add_edge(1, 2, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_neighbors_at() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        tg.add_node(2);
        tg.add_node(3);
        tg.add_edge(1, 2, 5).unwrap();
        tg.add_edge(1, 3, 15).unwrap();
        let neighbors_at_10 = tg.get_neighbors_at(1, 10);
        assert_eq!(neighbors_at_10, vec![2]);
        let neighbors_at_20 = tg.get_neighbors_at(1, 20);
        assert_eq!(neighbors_at_20.len(), 2);
        assert!(neighbors_at_20.contains(&2));
        assert!(neighbors_at_20.contains(&3));

        eprintln!("\nTG for test_get_neighbors_at:");
        tg.print();
    }

    #[test]
    fn test_get_neighbors_no_edge() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        let neighbors = tg.get_neighbors_at(1, 10);
        assert!(neighbors.is_empty());

        eprintln!("\nTG for test_get_neighbors_no_edge:");
        tg.print();
    }
    
    #[test]
    fn test_get_neighbors_nonexistent_node() {
        let tg = TemporalGraph::new();
        let neighbors = tg.get_neighbors_at(999, 100);
        assert!(neighbors.is_empty());
    }

    #[test]
    fn test_remove_node() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        tg.add_node(2);
        tg.add_node(3);
        tg.add_edge(1, 2, 5).unwrap();
        tg.add_edge(3, 2, 10).unwrap();
        assert_eq!(tg.remove_node(2), Ok(2));
        assert!(!tg.get_nodes().contains(&2));
        assert!(tg.get_edges().get(&1).unwrap().is_empty());
        assert!(tg.get_edges().get(&3).unwrap().is_empty());

        eprintln!("\nTG for test_remove_node:");
        tg.print();
    }

    #[test]
    fn test_remove_node_nonexistent() {
        let mut tg = TemporalGraph::new();
        assert_eq!(tg.remove_node(42), Err(false));
    }

    #[test]
    fn test_remove_edge() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        tg.add_node(2);
        tg.add_edge(1, 2, 5).unwrap();

        assert_eq!(tg.remove_edge(1, 2, 5), Ok((1, 2, 5)));
        assert!(tg.get_edges().get(&1).unwrap().is_empty());

        eprintln!("\nTG for test_remove_edge:");
        tg.print();
    }

    #[test]
    fn test_remove_edge_nonexistent() {
        let mut tg = TemporalGraph::new();
        tg.add_node(1);
        tg.add_node(2);
        tg.add_edge(1, 2, 5).unwrap();

        assert_eq!(tg.remove_edge(1, 2, 99), Err(false));
        assert_eq!(tg.remove_edge(2, 1, 5), Err(false));
        assert_eq!(tg.remove_edge(1, 3, 5), Err(false));

        eprintln!("\nTG for test_remove_edge_nonexistent:");
        tg.print();
    }
}

mod chrono_unit_tests {
    use super::chrono::*;
    use std::time::Instant;

    #[test]
    fn test_add_node() {
        let mut cg = ChronoGraph::new();
        cg.add_node(1);
        assert!(cg.get_nodes().contains(&1));

        eprintln!("\nCG for test_add_node:");
        cg.print();
    }

    #[test]
    fn test_add_edge_success() {
        let mut cg = ChronoGraph::new();
        cg.add_node(1);
        cg.add_node(2);
        let result = cg.add_edge(1, 2, 5);
        assert!(result.is_ok());
        assert_eq!(cg.get_edges().get(&1).unwrap(), &vec![(2, 5)]);

        eprintln!("\nCG for test_add_edge_success:");
        cg.print();
    }

    #[test]
    fn test_add_edge_missing_node() {
        let mut cg = ChronoGraph::new();
        cg.add_node(1);
        let result = cg.add_edge(1, 2, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_neighbors_at() {
        let mut cg = ChronoGraph::new();
        cg.add_node(1);
        cg.add_node(2);
        cg.add_node(3);
        cg.add_edge(1, 2, 5).unwrap();
        cg.add_edge(1, 3, 15).unwrap();
        let neighbors_at_10 = cg.get_neighbors_at(1, 10);
        assert_eq!(neighbors_at_10, vec![2]);
        let neighbors_at_20 = cg.get_neighbors_at(1, 20);
        assert_eq!(neighbors_at_20.len(), 2);
        assert!(neighbors_at_20.contains(&2));
        assert!(neighbors_at_20.contains(&3));

        eprintln!("\nCG for test_get_neighbors_at:");
        cg.print();
    }

    #[test]
    fn test_get_neighbors_no_edge() {
        let mut cg = ChronoGraph::new();
        cg.add_node(1);
        let neighbors = cg.get_neighbors_at(1, 10);
        assert!(neighbors.is_empty());

        eprintln!("\nCG for test_get_neighbors_no_edge:");
        cg.print();
    }

    #[test]
    fn test_get_neighbors_nonexistent_node() {
        let cg = ChronoGraph::new();
        let neighbors = cg.get_neighbors_at(999, 100);
        assert!(neighbors.is_empty());
    }
}

mod benchmark_tests {
    use super::sequential::*;
    use super::chrono::*;
    use rayon::prelude::*;
    use std::mem::size_of_val;
    use std::time::Instant;

    const NODE_COUNT: usize = 10_000;
    const EDGE_PER_NODE: usize = 10;
    const QUERY_TIMESTAMP: u64 = 50;

    #[test]
    fn benchmark_temporal_graph() {
        let mut tg = TemporalGraph::new();

        let start_nodes = Instant::now();
        (0..NODE_COUNT).for_each(|i| tg.add_node(i));
        let duration_nodes = start_nodes.elapsed();

        let node_insert_latency = duration_nodes.as_secs_f64() / NODE_COUNT as f64;

        eprintln!("[TG Benchmark] Added {} nodes in {:?} (insert latency: {:.6} sec/node)", 
            NODE_COUNT, duration_nodes, node_insert_latency);

        let start_edges = Instant::now();
        (0..NODE_COUNT).for_each(|i| {
            (0..EDGE_PER_NODE).for_each(|j| {
                let _ = tg.add_edge(i, (i + j + 1) % NODE_COUNT, (j as u64) * 10);
            });
        });
        let duration_edges = start_edges.elapsed();

        let edge_insert_latency = duration_edges.as_secs_f64() / (NODE_COUNT * EDGE_PER_NODE) as f64;

        eprintln!("[TG Benchmark] Added {} edges in {:?} (insert latency: {:.6} sec/edge)", 
            NODE_COUNT * EDGE_PER_NODE, duration_edges, edge_insert_latency);

        let start_query = Instant::now();
        let total_neighbors: usize = (0..NODE_COUNT)
            .map(|i| tg.get_neighbors_at(i, QUERY_TIMESTAMP).len())
            .sum();
        let duration_query = start_query.elapsed();

        let query_throughput = NODE_COUNT as f64 / duration_query.as_secs_f64();

        println!("[TG Benchmark] Queried neighbors at timestamp {} for {} nodes in {:?} (total neighbors: {}, throughput: {:.2} queries/sec)", 
            QUERY_TIMESTAMP, NODE_COUNT, duration_query, total_neighbors, query_throughput);

        let memory_overhead = size_of_val(&tg);

        println!("[TG Benchmark] Approximate memory overhead: {} bytes", memory_overhead);
    }

    #[test]
    fn benchmark_chrono_graph() {
        let mut cg = ChronoGraph::new();

        let start_nodes = Instant::now();
        (0..NODE_COUNT).for_each(|i| cg.add_node(i));
        let duration_nodes = start_nodes.elapsed();

        let node_insert_latency = duration_nodes.as_secs_f64() / NODE_COUNT as f64;

        eprintln!("[CG Benchmark] Added {} nodes in {:?} (insert latency: {:.6} sec/node)", 
            NODE_COUNT, duration_nodes, node_insert_latency);

        let start_edges = Instant::now();
        (0..NODE_COUNT).for_each(|i| {
            (0..EDGE_PER_NODE).for_each(|j| {
                let _ = cg.add_edge(i, (i + j + 1) % NODE_COUNT, (j as u64) * 10);
            });
        });
        let duration_edges = start_edges.elapsed();

        let edge_insert_latency = duration_edges.as_secs_f64() / (NODE_COUNT * EDGE_PER_NODE) as f64;

        eprintln!("[CG Benchmark] Added {} edges in {:?} (insert latency: {:.6} sec/edge)", 
            NODE_COUNT * EDGE_PER_NODE, duration_edges, edge_insert_latency);

        let start_query = Instant::now();
        let total_neighbors: usize = (0..NODE_COUNT)
            .into_par_iter() 
            .map(|i| cg.get_neighbors_at(i, QUERY_TIMESTAMP).len())
            .sum();
        let duration_query = start_query.elapsed();

        let query_throughput = NODE_COUNT as f64 / duration_query.as_secs_f64();

        println!("[CG Benchmark] Queried neighbors at timestamp {} for {} nodes in {:?} (total neighbors: {}, throughput: {:.2} queries/sec)", 
            QUERY_TIMESTAMP, NODE_COUNT, duration_query, total_neighbors, query_throughput);

        let memory_overhead = size_of_val(&cg);

        println!("[CG Benchmark] Approximate memory overhead: {} bytes", memory_overhead);
    }
}