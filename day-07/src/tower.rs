use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub self_weight: u32,
    pub tree_weight: u32,
    children: Vec<Node>,
}

impl Node {
    pub fn print(&self) {
        self.print_tree(0);
    }

    fn print_tree(&self, tab: u32) {
        if tab == 0 {
            println!("{} ({} / {})", self.name, self.self_weight, self.tree_weight);
        } else {
            let prefix = (1..tab).map(|_| "|  ").collect::<String>();
            println!("{}|- {} ({} / {})", prefix, self.name, self.self_weight, self.tree_weight);
        }

        let next_tab = tab + 1;

        for child in &self.children {
            child.print_tree(next_tab);
        }
    }

    /// Are the children of this node balanced?
    pub fn children_balanced(&self) -> bool {
        match self.children.iter().next() {
            None =>
                true,
            Some(first_child) =>
                self.children.iter()
                    .all( | child | child.tree_weight == first_child.tree_weight )
        }
    }


    pub fn required_change_to_balance(&self) -> Option<u32> {
        // If we and our children are balanced, nothing to check!
        if self.children_balanced() {
            return None
        }

        let mut sub_weights = HashMap::new();

        // Depth first check of children
        for child in &self.children {
            let result = child.required_change_to_balance();

            if result.is_some() {
                return result;
            }

            let mut list = sub_weights.remove(&child.tree_weight).unwrap_or(vec![]);
            list.push(child);
            sub_weights.insert(child.tree_weight, list);
        }

        if sub_weights.len() > 1 {
            let (max_weight, _) = sub_weights.iter().max_by_key( | i| ((*i).1.len(), i.0) ).unwrap();
            let (min_weight, min_nodes) = sub_weights.iter().min_by_key( | i| ((*i).1.len(), i.0) ).unwrap();

            assert_eq!(min_nodes.len(), 1, "Expected only one node to be off!");

            let child: &Node = min_nodes.iter().next().unwrap();
            let change = *max_weight as i32 - *min_weight as i32;

            Some((child.self_weight as i32 + change) as u32)
        } else {
            panic!("We should have more than one weight!");
        }
    }
}

pub fn read_tower(input: &str) -> Node {
    let re = Regex::new(
        r"^\s*([a-z]+) \((\d+)\)(\s+->\s+([a-z, ]+))?$"
    ).expect("Unable to compile regex");

    let mut str_nodes    = HashMap::new();
    let mut node_weights = HashMap::new();
    let mut possible_parents = HashSet::new();
    let mut is_child = HashSet::new();

    // Read all the dat into maps
    input.trim().lines().for_each(|line| {
        for cap in re.captures_iter(line) {
            let name   = String::from(&cap[1]);
            let weight = cap[2].parse().expect("Unable to parse weight");

            if is_child.contains(&name) == false {
                possible_parents.insert(name.clone());
            }

            // Does it have links
            let children = match cap.get(4) {
                Some(links) => {
                    let children: Vec<&str> = links.as_str().split(", ").collect();
                    let children_str: Vec<String> = children.iter().map(|child| String::from(*child)).collect();

                    children_str.iter().for_each( | child | {
                        is_child.insert(child.clone());
                        possible_parents.remove(&child.clone());
                    });

                    children_str
                },
                None => vec![],
            };

            str_nodes.insert(name.clone(), children);
            node_weights.insert(name.clone(), weight);
        }
    });

    // Convert the maps into a tree of nodes
    assert_eq!(possible_parents.len(), 1, "Expected only 1 parent");
    let root_name = possible_parents.iter().next().unwrap();

    fn build_tree(node_name: &String, node_weights: &HashMap<String, u32>, str_nodes: &HashMap<String, Vec<String>>) -> Node {
        let weight = *node_weights.get(node_name).unwrap();
        let children = str_nodes.get(node_name).unwrap();
        let children: Vec<Node> = children.iter().map( | child_name | build_tree(child_name, node_weights, str_nodes) ).collect();

        Node {
            name: node_name.clone(),
            self_weight: weight,
            tree_weight: children.iter().fold(weight, | sum, n | sum + n.tree_weight),
            children,
        }
    };

    build_tree(root_name, &node_weights, &str_nodes)
}
