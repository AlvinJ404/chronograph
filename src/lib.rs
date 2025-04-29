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
    use std::time::Instant;

    #[test]
    fn benchmark_add_nodes_edges_and_queries_TG() {
        let timestamp = 50;
        let mut tg = TemporalGraph::new();
        let node_count = 10_000;
        let edge_per_node = 10;

        let start_nodes = Instant::now();
        (0..node_count).for_each(|i| tg.add_node(i));
        let duration_nodes = start_nodes.elapsed();
        eprintln!("\n[Benchmark] Added {} nodes in {:?}", node_count, duration_nodes);

        let start_edges = Instant::now();
        (0..node_count).for_each(|i| {
            (0..edge_per_node).for_each(|j| {
                let _ = tg.add_edge(i, (i + j + 1) % node_count, (j as u64) * 10);
            });
        });
        let duration_edges = start_edges.elapsed();
        eprintln!("[Benchmark] Added {} edges in {:?}", node_count * edge_per_node, duration_edges);

        let start_query = Instant::now();
        let total_neighbors: usize = (0..node_count)
            .map(|i| tg.get_neighbors_at(i, timestamp).len())
            .sum();
        let duration_query = start_query.elapsed();
        println!(
            "[Benchmark] TG Queried neighbors at timestamp {} for {} nodes in {:?} (total neighbors returned: {})",
            timestamp, node_count, duration_query, total_neighbors
        );
    }

    #[test]
    fn benchmark_add_nodes_edges_and_queries_CG() {
        let timestamp = 50;
        let mut cg = ChronoGraph::new();
        let node_count = 10_000;
        let edge_per_node = 10;
        let timestamp = 50;

        let start_nodes = Instant::now();
        (0..node_count).for_each(|i| cg.add_node(i));
        let duration_nodes = start_nodes.elapsed();
        eprintln!("\n[Benchmark] Added {} nodes in {:?}", node_count, duration_nodes);

        let start_edges = Instant::now();
        (0..node_count).for_each(|i| {
            (0..edge_per_node).for_each(|j| {
                let _ = cg.add_edge(i, (i + j + 1) % node_count, (j as u64) * 10);
            });
        });
        let duration_edges = start_edges.elapsed();
        eprintln!("[Benchmark] Added {} edges in {:?}", node_count * edge_per_node, duration_edges);

        let start_query = Instant::now();
        let total_neighbors: usize = (0..node_count)
            .into_par_iter() // <--- PARALLEL query
            .map(|i| cg.get_neighbors_at(i, timestamp).len())
            .sum();
        let duration_query = start_query.elapsed();
        println!(
            "[Benchmark] CG Queried neighbors at timestamp {} for {} nodes in {:?} (total neighbors returned: {})",
            timestamp, node_count, duration_query, total_neighbors
        );
    }
}