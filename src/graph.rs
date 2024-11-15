use std::{
    collections::{HashMap, HashSet},
    fmt::{self},
};

pub struct Graph {
    root: String,
    adj: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn from_edges(edges: &Vec<(String, Option<String>)>) -> Self {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();

        let root = edges
            .iter()
            .filter(|(_, y)| y.is_none())
            .next()
            .expect("Failed to find master branch")
            .0
            .clone();

        edges
            .iter()
            .filter(|(_, y)| y.is_some())
            .for_each(|(child, y)| {
                let parent = y.as_ref().unwrap();

                adj.entry(parent.clone())
                    .or_insert_with(Vec::new)
                    .push(child.clone());
            });

        Graph { root, adj }
    }

    fn dfs_recursive(
        &self,
        node: &String,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
        formatting_string: &mut String,
        is_last: bool,
    ) {
        // Mark the node as visited
        visited.insert(node.clone());
        result.push(format!("{}{}", formatting_string, node)); // Add node to result list

        let mut type1 = false;
        let mut type2 = false;
        if formatting_string.ends_with("└─ ") || formatting_string.ends_with("├─ ") {
            type1 = formatting_string.ends_with("└─ ");
            type2 = formatting_string.ends_with("├─ ");
            for _ in 0..3 {
                formatting_string.pop();
            }
            if is_last {
                formatting_string.push_str("   ");
            } else {
                formatting_string.push_str("│  ");
            }
        }

        // Recursively visit all adjacent nodes
        if let Some(neighbors) = self.adj.get(node) {
            for (i, neighbor) in neighbors.iter().enumerate() {
                if i == neighbors.len() - 1 {
                    formatting_string.push_str("└─ ");
                } else {
                    formatting_string.push_str("├─ ");
                }
                if !visited.contains(neighbor) {
                    self.dfs_recursive(
                        neighbor,
                        visited,
                        result,
                        formatting_string,
                        i == neighbors.len() - 1,
                    );
                }
                for _ in 0..3 {
                    formatting_string.pop();
                }
            }
        }

        if type1 || type2 {
            for _ in 0..3 {
                formatting_string.pop();
            }
            if type1 {
                formatting_string.push_str("└─ ");
            } else {
                formatting_string.push_str("├─ ");
            }
        }
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut s = String::new();
        self.dfs_recursive(&self.root, &mut visited, &mut result, &mut s, true);
        write!(f, "{}", result.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_edges() -> Vec<(String, Option<String>)> {
        vec![
            ("master".to_string(), None),
            ("feature1".to_string(), Some("master".to_string())),
            ("feature2".to_string(), Some("master".to_string())),
            ("feature3".to_string(), Some("feature1".to_string())),
            ("feature4".to_string(), Some("feature2".to_string())),
            ("feature5".to_string(), Some("feature3".to_string())),
            ("feature6".to_string(), Some("feature4".to_string())),
        ]
        .into_iter()
        .map(|(x, y)| (x.to_string(), y.map(|s| s.to_string())))
        .collect()
    }

    #[test]
    fn test_bfs() {
        // Arrange
        let edges = get_test_edges();
        let graph = Graph::from_edges(&edges);

        // Act
        println!("{}", graph);
    }

    #[test]
    fn test_from_edges() {
        // Arrange
        let edges = get_test_edges();

        // Act
        let graph = Graph::from_edges(&edges);

        // Assert
        assert_eq!(graph.root, "master");
        assert_eq!(graph.adj.len(), 5);
    }
}
