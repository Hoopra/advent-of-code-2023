use super::{
    graph::GraphNode,
    map_2d::{Direction, Position},
    priority_queue::{QueueNode, StateKey},
    Graph,
};
use std::collections::{BinaryHeap, HashMap};

// uses A* search: https://en.wikipedia.org/wiki/A*_search_algorithm
pub fn find_shortest_path_in_graph(
    graph: &Graph,
    start: Position,
    end: Position,
    min_steps: u32,
    max_steps: u32,
) -> Option<u32> {
    let mut end_state: Option<QueueNode> = None;

    let mut queue = BinaryHeap::new();
    let mut came_from: HashMap<StateKey, QueueNode> = HashMap::new();
    let mut distance: HashMap<StateKey, u32> = HashMap::new();

    let initial_state = QueueNode::new(0, start, Direction::None, 0);

    queue.push(initial_state);
    distance.insert(initial_state.into(), 0);

    while queue.len() > 0 {
        let state = queue.pop().unwrap();

        let QueueNode {
            position: current_position,
            steps: current_steps,
            direction: current_direction,
            cost: current_cost,
        } = state;

        if current_position == end && current_steps >= min_steps {
            end_state = Some(state);
            break;
        }

        let key: StateKey = state.into();
        let previous_score = distance.get(&key).unwrap_or(&u32::max_value());

        if previous_score < &current_cost {
            continue;
        }

        let current_node = graph.get(&current_position).unwrap();
        let neighbors = current_node.find_connected_positions(current_direction);

        neighbors.iter().for_each(|(direction, neighbor_position)| {
            let neighbor_node: Option<&GraphNode> = graph.get(neighbor_position);
            let did_change_direction = direction == &current_direction;

            let steps = match did_change_direction {
                true => current_steps + 1,
                false => 1,
            };

            let too_many_steps = steps > max_steps;
            let too_few_steps = match current_direction != Direction::None {
                true => !did_change_direction && current_steps < min_steps,
                _ => false,
            };

            if neighbor_node.is_none() || too_many_steps || too_few_steps {
                return;
            }

            let neigbor_node = neighbor_node.unwrap();
            let new_score = current_cost + neigbor_node.value;

            let next = QueueNode::new(new_score, *neighbor_position, *direction, steps);
            let key = next.into();
            let best_score = distance.get(&key).unwrap_or(&u32::max_value());

            if &next.cost >= best_score {
                return;
            }

            came_from.insert(key, state);
            distance.insert(key, new_score);

            queue.push(next)
        });
    }

    let end_state = end_state.unwrap();
    let path = construct_path(&came_from, graph, start, &end_state);

    // draw_path_in_graph(graph, &path);

    Some(
        path.iter()
            .rev()
            .skip(1)
            .fold(0, |sum, node| sum + node.value),
    )
}

fn construct_path<'a>(
    came_from: &'a HashMap<StateKey, QueueNode>,
    graph: &'a Graph,
    start: Position,
    end_queue_node: &'a QueueNode,
) -> Vec<&'a GraphNode> {
    let end_node = graph.get(&end_queue_node.position).unwrap();
    let mut path: Vec<&GraphNode> = vec![end_node];
    let mut current = end_queue_node;

    while current.position != start {
        let key: StateKey = (*current).into();

        let queue_node = came_from.get(&key).unwrap();
        let node = graph.get(&queue_node.position).unwrap();
        path.push(&node);

        current = queue_node;
    }

    path
}

#[allow(dead_code)]
fn draw_path_in_graph(graph: &Graph, path: &Vec<&GraphNode>) {
    let (x_max, y_max) = graph.size;

    let positions: Vec<Position> = path.iter().map(|value| value.position).collect();

    for node in &positions {
        println!("node: {:?}", node);
    }

    println!("");
    for y in 0..=y_max {
        for x in 0..=x_max {
            let position = (x, y);

            match positions.contains(&position) {
                true => print!("X"),
                _ => print!("{}", graph.get(&position).unwrap().value),
            }
        }
        println!("");
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_shortest_path_in_graph() {
        let text = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";

        let graph = Graph::from_text(text);
        let result = find_shortest_path_in_graph(&graph, (0, 0), graph.size, 1, 3);

        assert_eq!(graph.size, (12, 12));
        assert_eq!(graph.len(), 169);
        assert_eq!(result, Some(102));
    }

    #[test]
    fn finds_shortest_path_in_graph_with_minimum_steps() {
        let text = "111111111111\n999999999991\n999999999991\n999999999991\n999999999991";

        let graph = Graph::from_text(text);
        let result = find_shortest_path_in_graph(&graph, (0, 0), graph.size, 4, 10);
        assert_eq!(result, Some(71));

        let text = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";

        let graph = Graph::from_text(text);
        let result = find_shortest_path_in_graph(&graph, (0, 0), graph.size, 4, 10);
        assert_eq!(result, Some(94));
    }
}
