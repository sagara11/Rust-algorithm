use std::collections::{HashMap, VecDeque};

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Person {
    name: String,
    value: u128,
}

pub fn have_proper_value(user: &Person) -> bool {
    if user.value == 34 {
        return true;
    }

    false
}

pub fn breadth_first_search(
    graph: HashMap<&Person, Vec<&Person>>,
    initial_value: &Person,
) -> String {
    let mut result = String::new();
    let mut queue: VecDeque<&Person> = VecDeque::new();
    let neighbors_node = graph.get(initial_value).unwrap();
    for e in neighbors_node {
        queue.push_front(e);
    }
    let mut searched: HashMap<&Person, bool> = HashMap::new();

    while queue.len() > 0 {
        let node = queue.pop_back().unwrap();
        if searched.get(node) == None {
            if have_proper_value(node) {
                result.push_str(node.name.as_str());
                break;
            } else {
                let neighbors_node = graph.get(node).unwrap();
                for e in neighbors_node {
                    queue.push_front(e);
                }
                searched.insert(node, true);
            }
        }
    }

    result
}

pub fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let mut i = low - 1;
    let pivot = arr[high as usize];
    for j in low..=high {
        if arr[j as usize] < pivot {
            i += 1;
            arr.swap(j as usize, i as usize);
        }
    }

    arr.swap((i + 1) as usize, high as usize);

    (i + 1) as i32
}

pub fn quick_sort(arr: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let index_pivot = partition(arr, low, high);
        quick_sort(arr, low, index_pivot - 1);
        quick_sort(arr, index_pivot + 1, high);
    }
}

fn main() {
    let mut graph: HashMap<&Person, Vec<&Person>> = HashMap::new();

    let me = Person {
        name: String::from("Thong"),
        value: 10,
    };

    let Bob = Person {
        name: String::from("Bob"),
        value: 5,
    };

    let Claire = Person {
        name: String::from("Claire"),
        value: 20,
    };

    let Alice = Person {
        name: String::from("Alice"),
        value: 11,
    };

    let Peggy = Person {
        name: String::from("Peggy"),
        value: 50,
    };

    let Thom = Person {
        name: String::from("Thom"),
        value: 34,
    };

    let Johny = Person {
        name: String::from("Johny"),
        value: 60,
    };

    let Anuj = Person {
        name: String::from("Anuj"),
        value: 58,
    };

    let me_neighbors = Vec::from([&Bob, &Alice, &Claire]);
    let alice_neighbors = Vec::from([&Peggy]);
    let claire_neighbors = Vec::from([&Thom, &Johny]);
    let bob_neighbors = Vec::from([&Anuj, &Peggy]);

    graph.insert(&me, me_neighbors);
    graph.insert(&Bob, bob_neighbors);
    graph.insert(&Claire, claire_neighbors);
    graph.insert(&Alice, alice_neighbors);
    graph.insert(&Peggy, Vec::new());
    graph.insert(&Anuj, Vec::new());
    graph.insert(&Thom, Vec::new());
    graph.insert(&Johny, Vec::new());

    dbg!(breadth_first_search(graph, &me));

    let mut temp = Vec::from([3, 4, 10, 5, 2, 9, 7]);
    let length = temp.len();
    quick_sort(&mut temp, 0, (length as i32) - 1);

    dbg!(temp);
}
