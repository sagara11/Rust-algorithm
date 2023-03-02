use std::collections::HashMap;

pub fn initialize_costs<'a>(
    costs: &mut HashMap<&'a str, f64>,
    graph: &HashMap<&'a str, HashMap<&'a str, f64>>,
) {
    let inf = f64::INFINITY;
    for (k, _) in graph {
        costs.insert(k, inf);
    }

    let start_map = graph.get("Start").unwrap();

    for (k, v) in start_map {
        costs.entry(k).and_modify(|e| *e = *v).or_default();
    }
}

pub fn initialize_parent<'a>(
    parents: &mut HashMap<&'a str, &str>,
    graph: &HashMap<&'a str, HashMap<&'a str, f64>>,
) {
    for (k, _) in graph {
        parents.insert(k, "none");
    }
}

pub fn find_lowest_cost_node<'a>(
    costs: &HashMap<&'a str, f64>,
    searched: &HashMap<&str, bool>,
) -> Option<&'a str> {
    let inf = f64::INFINITY;
    let mut lowest_cost = inf;
    let mut lowest_cost_node: Option<&str> = None;

    for (k, v) in costs {
        if !searched.contains_key(k) && *v < lowest_cost {
            lowest_cost = *v;
            lowest_cost_node = Some(k);
        }
    }

    lowest_cost_node
}

pub fn dijkstra_algorithm<'a>(
    graph: &HashMap<&'a str, HashMap<&'a str, f64>>,
    costs: &mut HashMap<&'a str, f64>,
    parents: &mut HashMap<&'a str, &'a str>,
    searched: &mut HashMap<&'a str, bool>,
) {
    let mut node = match find_lowest_cost_node(costs, searched) {
        Some(e) => e,
        None => "none",
    };

    parents
        .entry(node)
        .and_modify(|e| *e = "Start")
        .or_default();

    while node != "none" {
        let cost = *costs.get(node).unwrap();
        let neighbors = graph.get(node).unwrap();

        for (k, v) in neighbors {
            let new_cost = cost + *v;
            let old_cost = *costs.get(k).unwrap();
            if old_cost > new_cost {
                costs.entry(k).and_modify(|e| *e = new_cost).or_default();
                parents.entry(k).and_modify(|k| *k = node).or_default();
            }
        }

        searched.insert(node, true);
        node = match find_lowest_cost_node(costs, searched) {
            Some(e) => e,
            None => "none",
        };
    }
}

fn main() {
    let start_hashmap: HashMap<&str, f64> = HashMap::from([("A", 6.0), ("B", 2.0)]);
    let a_hashmap: HashMap<&str, f64> = HashMap::from([("Fin", 1.0)]);
    let b_hashmap: HashMap<&str, f64> = HashMap::from([("A", 3.0), ("Fin", 5.0)]);

    let graph: HashMap<&str, HashMap<&str, f64>> = HashMap::from([
        ("Start", start_hashmap),
        ("A", a_hashmap),
        ("B", b_hashmap),
        ("Fin", HashMap::new()),
    ]);

    let mut costs: HashMap<&str, f64> = HashMap::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    let mut searched: HashMap<&str, bool> = HashMap::new();

    initialize_costs(&mut costs, &graph);
    initialize_parent(&mut parents, &graph);
    dijkstra_algorithm(&graph, &mut costs, &mut parents, &mut searched);
}
