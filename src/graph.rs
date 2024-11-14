use std::{
    collections::{HashMap, HashSet},
    fmt::{self, format, Display},
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

    pub fn dfs(&self) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut s = String::new();
        self.dfs_recursive(&self.root, &mut visited, &mut result, &mut s);
        result
    }

    fn dfs_recursive(
        &self,
        node: &String,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
        formatting_string: &mut String,
    ) {
        // Mark the node as visited
        visited.insert(node.clone());
        println!("{}{}", formatting_string, node);
        result.push(node.clone()); // Add node to result list

        let mut check: bool = false;
        if formatting_string.ends_with("├─") {
            // change the last character of formatting_str to a whitespace
            formatting_string.pop();
            formatting_string.pop();
            formatting_string.push_str("│ ");
            check = true;
        }
        formatting_string.push_str("├─");

        // Recursively visit all adjacent nodes
        if let Some(neighbors) = self.adj.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_recursive(neighbor, visited, result, formatting_string);
                }
            }
        }

        formatting_string.pop();
        formatting_string.pop();
        if check {
            formatting_string.pop();
            formatting_string.pop();
            formatting_string.push_str("├─");
        }
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
        graph.dfs();
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
