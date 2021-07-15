use std::fs;
use std::time::Instant;
use std::collections::HashMap;

use log::info;

use serde::{Deserialize, Serialize};

pub mod cases;

pub mod munkres;
use munkres::Matrix;

const MAX : i32 = 1000000;

#[derive(Serialize, Deserialize)]
struct Intersection {
    name: String,
    address: String,
    id: usize,
    altitude: f64,
    latitude: f64,
    longitude: f64,
    neighbours: HashMap<String, String>
}

struct Edge {
    from: String,
    to: String,
    length: i32
}

struct Node {
    id: usize,
    name: String,
    edges: HashMap<String, Edge>
}

fn build_map(intersections : & Vec<Intersection>) -> HashMap<String, Node> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    // Add all intersections first, then populate edges/streets.
    for intersection in intersections {
        if !nodes.contains_key(&intersection.name) {
            nodes.insert(intersection.name.clone(), Node {
                id: intersection.id,
                name: intersection.name.clone(), 
                edges: HashMap::new()
            });
        }
    }

    for intersection in intersections {
        for (name, length) in intersection.neighbours.iter() {
            match nodes.get_mut(&intersection.name) {
                None => (),
                Some(node) => {
                    node.edges.insert(name.clone(), Edge {
                        from: intersection.name.clone(),
                        to: name.clone(),
                        length: length.parse::<f32>().unwrap().ceil() as i32
                    });
                }
            }
        }
    }

    return nodes;
}

fn all_pairs_shortest_arr(array : &mut Matrix<i32>, map : & HashMap<String, Node>) {

    let size = map.len();
    let mut ids : HashMap<String, i32> = HashMap::new();
    for i in 1..size+1 {
        array[i][i] = 0;
    }

    let mut keys : Vec<String> = map.keys().map(|key| key.to_string()).collect();
    keys.sort();

    let mut count = 1;
    for key in keys {
        ids.insert(key.clone(), count);
        count = count + 1;
    }
    
    for (_, from_node) in map {
        for (_, edge) in &from_node.edges {
            if !ids.contains_key(&edge.to) {
                println!("id_not_contains {}", &edge.to);
            }
            if !ids.contains_key(&edge.from) {
                println!("id_not_contains {}", &edge.from);
            }
            let i = *ids.get(&edge.from).unwrap();
            let j = *ids.get(&edge.to).unwrap();
            array[i as usize][j as usize] = edge.length;
        }
    }

    for k in 1..size {
        for i in 1..size {
            for j in 1..size {
                if array[i][j] > array[i][k] + array[k][j] {
                    array[i][j] = array[i][k] + array[k][j];
                }
            }
        }
    }

}

fn parse(filename : String) -> (Matrix<i32>, Matrix<i32>, HashMap<usize, (String, usize)>) {

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let intersections : Vec<Intersection> = serde_json::from_str(&contents).unwrap();
    let nodes = build_map(&intersections);
    let nodes_len = nodes.len();
    
    let mut array : Matrix<i32> = munkres::square(nodes_len+1);

    let before = Instant::now();
    all_pairs_shortest_arr(&mut array, &nodes);
    info!("floyd-warshall -> {:.2?}", before.elapsed());

    let mut odd_ids : HashMap<usize, (String, usize)> = HashMap::new();
    
    //nodes.into_iter()
    //    .filter(|(_, node)| node.edges.len() % 2 == 1)
    //    .map(|(_, node)| (node.id, (node.name, 0)))
    //    .collect();
       
    // Translate the index into the array ('array') containing all nodes to
    // the position of the node in the odd array.
    let mut map : Vec<usize> = vec![0; nodes_len+1];
    let mut pos = 1;
    for (name, node) in nodes {
        if node.edges.len() % 2 == 1 {
            map[pos] = node.id;
            odd_ids.insert(pos, (node.name, node.id));
            pos = pos + 1;
        }
    }

    let odd_len = odd_ids.len();
    // Make sure 'odd' array is offset by 1 along each axis.
    let mut odd = munkres::square(odd_len+1);
    info!("found {} odd nodes.", odd_len);
    // Add one in the appropriate places to ensure the 'odd' array effectively
    // stars at index 1 instead of 0.
    for i in 1..odd_len+1 {
        for j in 1..odd_len+1 {
            odd[i][j] = array[map[i]][map[j]];
        }
        for j in 1..odd_len {
            odd[j][i] = array[map[j]][map[i]];
        }
    }

    for i in 1..odd_len+1 {
        odd[i][i] = MAX;
    }

    (array, odd, odd_ids)
}

fn main() {

    env_logger::init();

    // Deserialize JSON into data structures.
   
    let file = "website.json";
    info!("parsing: {}...", file);
    
    let (_, odd, ids) = parse(file.to_string());

    let before = Instant::now();
    info!("solving for matching...");
    munkres::solve(odd, ids);
    info!("munkres -> {:.2?}", before.elapsed());
  }