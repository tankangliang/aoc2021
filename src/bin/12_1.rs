use std::collections::{HashMap, HashSet};
use aoc2021::input_data;

fn is_big(s: &str) -> bool {
    s.chars().nth(0).unwrap().is_uppercase()
}

fn visit(node: &str,
         count: &mut u32,
         adj_list: &HashMap<String, HashSet<String>>,
         visited: &mut HashMap<String, usize>) {
    {
        let node_visit_count = visited.entry(node.to_string()).or_insert(0);
        *node_visit_count += 1;
    }

    let to_visit = adj_list.get(node);
    if let Some(hs) = to_visit {
        for neighbour in hs {
            if neighbour == "end" {
                // println!("{:#?}", visited);
                *count += 1;
            } else {
                // println!("{}", node);
                let is_big = is_big(neighbour);
                match (visited.get(neighbour), is_big) {
                    (_, true) => visit(neighbour, count, adj_list, visited),
                    (None, _) => visit(neighbour, count, adj_list, visited),
                    (Some(visit_count), false) if *visit_count == 0 => visit(neighbour, count, adj_list, visited),
                    _ => {}
                }
            }

        }
    }

    {
        let node_visit_count = visited.entry(node.to_string()).or_insert(0);
        *node_visit_count -= 1;
    }
}

fn main() {
    let content = input_data(12);

    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();

    for line in content.lines() {
        let mut split = line.split("-");
        let a = split.next().unwrap().to_string();
        let b = split.next().unwrap().to_string();
        {
            let a_entry = adj_list.entry(a.clone()).or_insert(
                HashSet::new()
            );

            a_entry.insert(b.clone());
        }
        {
            let b_entry = adj_list.entry(b.clone()).or_insert(
                HashSet::new()
            );
            b_entry.insert(a.clone());
        }
    }

    let mut count = 0_u32;
    let mut visited = HashMap::new();
    visit("start", &mut count, &adj_list, &mut visited);
    println!("{}", count);
}
