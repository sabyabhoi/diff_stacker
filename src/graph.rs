use std::collections::HashMap;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test_edges() {
        // Arrange
        let edges: Vec<(String, Option<String>)> = vec![
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
        .collect();

        // Act
        let graph = Graph::from_edges(&edges);

        // Assert
        assert_eq!(graph.root, "master");
        assert_eq!(graph.adj.len(), 5);
    }
}
