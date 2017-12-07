use regex::Regex;
use std::collections::HashMap;

pub struct Node {
    name: String,
    weight: u32,
    children: Vec<Node>,
}

impl Node {
    fn new(name: String, weight: u32) -> Self {
        Node { name, weight, children: vec![] }
    }

    fn add_child(&mut self, child: Node) -> &Node {
        self.children.push(child);
        &self.children[self.children.len() - 1]
    }
}

pub fn read_tower<'a>(input: &str) {
    let re = Regex::new(
        r"^\s*([a-z]+) \((\d+)\)(\s+->\s+([a-z, ]+))?$"
    ).expect("Unable to compile regex");

    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut node_references: HashMap<String, &'a Node> = HashMap::new();

    let mut children_links = HashMap::new();

    input.trim().lines().for_each(|line| {
        for cap in re.captures_iter(line) {
            let mut node: &'a Node = &Node::new(
                String::from(&cap[1]),
                cap[2].parse().expect("Unable to parse weight")
            );

            // Does it have links
            if let Some(links) = cap.get(4) {
                let children: Vec<&str> = links.as_str().split(", ").collect();
                let children_str: Vec<String> = children.iter().map(|child| String::from(*child)).collect();

                children_links.insert(node.name.clone(), children_str);
            }

            node_references.insert(node.name.clone(), &node);
        }
    });

    for (parentName, children) in &children_links {
        let mut parent = nodes.remove(parentName).unwrap();

        for childName in children {
            let child = nodes.remove(childName).unwrap();

            parent.add_child(child);
        }

        nodes.insert(parentName.clone(), parent);
    }
}
