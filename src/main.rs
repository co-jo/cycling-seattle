use std::fs;
use std::time::Instant;
use std::collections::HashMap;

use log::info;
use log::debug;

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

fn build_map(intersections : & Vec<Intersection>) -> Vec<(usize, Node)> {
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

    // Return sorted Vector.
    let mut vector : Vec<(usize, Node)> = Vec::new();
    for (name, node) in nodes {
        vector.push((node.id, node));
    }
    vector.sort_by(|a, b| a.0.cmp(&b.0));
    
    return vector;
}

fn all_pairs_shortest_arr(array : &mut Matrix<i32>, intersections : & Vec<(usize, Node)>) {

    let size = intersections.len() + 1;
    let mut ids : HashMap<String, i32> = HashMap::new();
    for i in 1..size {
        array[i][i] = 0;
    }

    let mut keys : Vec<String> = intersections
        .iter()
        .map(|(id, node)| node.name.to_string())
        .collect();
    keys.sort();

    // This block offsets (increases) the keys by 1, so 0 => 1.
    let mut count = 1;
    for key in keys {
        ids.insert(key.clone(), count);
        count = count + 1;
    }
    
    debug!("loading node map into matrix ...");
    for (_, from_node) in intersections {
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

    for i in 1..size {
        for j in 1..size {
            // A zero value that is not a self reference.
            if array[i][j] == 0 && i != j{
                array[i][j] = MAX;
            }
        }
    }

    debug!("pre-all_pairs_shortest::print_raw_matrix ...");
    munkres::print_raw_matrix(&array);

    for k in 1..size {
        for i in 1..size {
            for j in 1..size {
                if array[i][j] > array[i][k] + array[k][j] {
                    array[i][j] = array[i][k] + array[k][j];
                }
            }
        }
    }
    
    debug!("post-all_pairs_shortest::print_raw_matrix ...");
    munkres::print_raw_matrix(&array);

}

fn parse(filename : String) -> (Matrix<i32>, Matrix<i32>, HashMap<usize, (String, usize)>) {

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let intersections : Vec<Intersection> = serde_json::from_str(&contents).unwrap();
    debug!("translating JSON into node map ...");
    let nodes = build_map(&intersections);
    let nodes_len = nodes.len();
    
    let mut array : Matrix<i32> = munkres::square(nodes_len+1);

    let before = Instant::now();
    debug!("starting all-pairs-shortest-path ...");
    all_pairs_shortest_arr(&mut array, &nodes);
    info!("floyd-warshall -> {:.2?}", before.elapsed());

    let mut odd_ids : HashMap<usize, (String, usize)> = HashMap::new();
    
    // Translate the index into the array ('array') containing all nodes to
    // the position of the node in the odd array.
    let mut map : Vec<usize> = vec![0; nodes_len+1];
    let mut pos = 1;
    for (_, node) in nodes.iter() {
        if node.edges.len() % 2 == 1 {
            map[pos] = node.id + 1;
            odd_ids.insert(pos, (node.name.clone(), node.id));
            pos = pos + 1;
        }
    }

    let odd_len = odd_ids.len();
    // Make sure 'odd' array is offset by 1 along each axis.
    let mut odd = munkres::square(odd_len+1);
    info!("found {} odd nodes.", odd_len);
    // Add one in the appropriate places to ensure the 'odd' array effectively
    // stars at index 1 instead of 0.
    for i in 1..pos {
        for j in 1..pos {
            odd[i][j] = array[map[i]][map[j]];
        }
        for j in 1..pos {
            odd[j][i] = array[map[j]][map[i]];
        }
    }

    for i in 1..pos {
        odd[i][i] = MAX;
    }

    (array, odd, odd_ids)
}

fn main() {

    env_logger::init();

    // Deserialize JSON into data structures.
   
    let file = "website-alternate.json";
    info!("parsing: {}...", file);
    
    let (_, odd, ids) = parse(file.to_string());
    munkres::print_raw_matrix(&odd);
    let before = Instant::now();
    info!("solving for matching...");
    munkres::solve(odd, ids);
    info!("munkres -> {:.2?}", before.elapsed());
  }