//! Tarjan's algorithm for finding strongly connected components.
//!
//! This module provides an implementation of Tarjan's algorithm for finding
//! strongly connected components in a directed graph.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// Implements Tarjan's algorithm for finding strongly connected components.
pub struct Tarjan<T> {
    /// The graph represented as an adjacency list.
    graph: HashMap<T, Vec<T>>,
    /// The index of each vertex.
    index: HashMap<T, usize>,
    /// The lowlink value of each vertex.
    lowlink: HashMap<T, usize>,
    /// The set of vertices on the stack.
    on_stack: HashSet<T>,
    /// The stack of vertices.
    stack: Vec<T>,
    /// The current index.
    current_index: usize,
    /// The strongly connected components.
    components: Vec<Vec<T>>,
}

impl<T> Tarjan<T>
where
    T: Eq + Hash + Clone,
{
    /// Creates a new Tarjan algorithm instance.
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
            index: HashMap::new(),
            lowlink: HashMap::new(),
            on_stack: HashSet::new(),
            stack: Vec::new(),
            current_index: 0,
            components: Vec::new(),
        }
    }

    /// Default implementation delegates to new()
    pub fn default_instance() -> Self {
        Self::new()
    }

    /// Adds a vertex to the graph.
    pub fn add_vertex(&mut self, vertex: T) {
        self.graph.entry(vertex).or_insert_with(|| Vec::new());
    }

    /// Adds an edge from `from` to `to`.
    pub fn add_edge(&mut self, from: T, to: T) {
        self.add_vertex(from.clone());
        self.add_vertex(to.clone());

        if let Some(edges) = self.graph.get_mut(&from) {
            edges.push(to);
        }
    }

    /// Finds all strongly connected components in the graph.
    pub fn find_components(&mut self) -> &[Vec<T>] {
        self.index.clear();
        self.lowlink.clear();
        self.on_stack.clear();
        self.stack.clear();
        self.current_index = 0;
        self.components.clear();

        for vertex in self.graph.keys().cloned().collect::<Vec<_>>() {
            if !self.index.contains_key(&vertex) {
                self.strong_connect(vertex);
            }
        }

        &self.components
    }

    /// Performs the strong connect operation on a vertex.
    fn strong_connect(&mut self, vertex: T) {
        self.index.insert(vertex.clone(), self.current_index);
        self.lowlink.insert(vertex.clone(), self.current_index);
        self.current_index += 1;
        self.stack.push(vertex.clone());
        self.on_stack.insert(vertex.clone());

        // Consider successors of vertex
        if let Some(successors) = self.graph.get(&vertex).cloned() {
            for successor in successors {
                if !self.index.contains_key(&successor) {
                    self.strong_connect(successor.clone());

                    // Update lowlink of vertex
                    let vertex_lowlink = self.lowlink.get(&vertex).cloned().unwrap_or(0);
                    let successor_lowlink = self.lowlink.get(&successor).cloned().unwrap_or(0);
                    let new_lowlink = vertex_lowlink.min(successor_lowlink);
                    self.lowlink.insert(vertex.clone(), new_lowlink);
                } else if self.on_stack.contains(&successor) {
                    // Successor is in stack and hence in the current SCC
                    let vertex_lowlink = self.lowlink.get(&vertex).cloned().unwrap_or(0);
                    let successor_index = self.index.get(&successor).cloned().unwrap_or(0);
                    let new_lowlink = vertex_lowlink.min(successor_index);
                    self.lowlink.insert(vertex.clone(), new_lowlink);
                }
            }
        }

        let vertex_index = self.index.get(&vertex).cloned().unwrap_or(0);
        let vertex_lowlink = self.lowlink.get(&vertex).cloned().unwrap_or(0);

        if vertex_index == vertex_lowlink {
            let mut component = Vec::new();
            loop {
                let w = self.stack.pop().expect("Stack should not be empty");
                self.on_stack.remove(&w);
                component.push(w.clone());
                if w == vertex {
                    break;
                }
            }
            self.components.push(component);
        }
    }
}

impl<T> Default for Tarjan<T>
where
    T: Eq + Hash + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn test_tarjan_simple() {
        let mut tarjan = Tarjan::new();

        // Add vertices and edges
        tarjan.add_edge(1, 2);
        tarjan.add_edge(2, 3);
        tarjan.add_edge(3, 1);
        tarjan.add_edge(3, 4);
        tarjan.add_edge(4, 5);
        tarjan.add_edge(5, 4);

        // Find components
        let components = tarjan.find_components();

        // There should be 2 components: [1, 2, 3] and [4, 5]
        assert_eq!(components.len(), 2);

        // Check that each component contains the expected vertices
        let mut found_123 = false;
        let mut found_45 = false;

        for component in components {
            if component.len() == 3
                && component.contains(&1)
                && component.contains(&2)
                && component.contains(&3)
            {
                found_123 = true;
            } else if component.len() == 2 && component.contains(&4) && component.contains(&5) {
                found_45 = true;
            }
        }

        assert!(found_123);
        assert!(found_45);
    }

    #[test]
    fn test_tarjan_single_vertex() {
        let mut tarjan = Tarjan::new();

        // Add a single vertex
        tarjan.add_vertex(1);

        // Find components
        let components = tarjan.find_components();

        // There should be 1 component: [1]
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].len(), 1);
        assert_eq!(components[0][0], 1);
    }

    #[test]
    fn test_tarjan_no_cycles() {
        let mut tarjan = Tarjan::new();

        // Add vertices and edges in a DAG
        tarjan.add_edge(1, 2);
        tarjan.add_edge(2, 3);
        tarjan.add_edge(3, 4);

        // Find components
        let components = tarjan.find_components();

        // There should be 4 components, each with a single vertex
        assert_eq!(components.len(), 4);

        for component in components {
            assert_eq!(component.len(), 1);
        }
    }

    #[test]
    fn test_tarjan_complex() {
        let mut tarjan = Tarjan::new();

        // Add vertices and edges in a more complex graph
        tarjan.add_edge(1, 2);
        tarjan.add_edge(2, 3);
        tarjan.add_edge(3, 1);
        tarjan.add_edge(3, 4);
        tarjan.add_edge(4, 5);
        tarjan.add_edge(5, 6);
        tarjan.add_edge(6, 4);
        tarjan.add_edge(6, 7);

        // Find components
        let components = tarjan.find_components();

        // There should be 3 components: [1, 2, 3], [4, 5, 6], and [7]
        assert_eq!(components.len(), 3);

        // Check that each component contains the expected vertices
        let mut found_123 = false;
        let mut found_456 = false;
        let mut found_7 = false;

        for component in components {
            if component.len() == 3
                && component.contains(&1)
                && component.contains(&2)
                && component.contains(&3)
            {
                found_123 = true;
            } else if component.len() == 3
                && component.contains(&4)
                && component.contains(&5)
                && component.contains(&6)
            {
                found_456 = true;
            } else if component.len() == 1 && component.contains(&7) {
                found_7 = true;
            }
        }

        assert!(found_123);
        assert!(found_456);
        assert!(found_7);
    }
}
