/// src/main.rs
///
/// # Author
/// Yifan Jiang, 2025
/// alvinjiang01@hotmail.com
/// GitHub: [AlvinJ404](https://github.com/AlvinJ404/chronograph)
use chronograph::sequential::TemporalGraph;

fn main() {
    let mut tg = TemporalGraph::new();

    tg.add_node(1);
    tg.add_node(2);
    tg.add_node(3);
    tg.add_edge(1, 2, 5); // edge active at time 5
    tg.add_edge(1, 3, 10);
    tg.add_edge(2, 3, 15);
    tg.print();
    println!("Neighbors of 1 at time 6: {:?}", tg.get_neighbors_at(1, 6));
    println!("Neighbors of 1 at time 12: {:?}", tg.get_neighbors_at(1, 12));
}