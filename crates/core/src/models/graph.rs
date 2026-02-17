use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::attraction::Attraction;
use crate::utils::distance::haversine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub to: u32,
    pub travel_time: u32,  // minutes
    pub cost: f64,         // travel cost in USD
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    pub attractions: HashMap<u32, Attraction>,
    pub adjacency: HashMap<u32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            attractions: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    // Build a complete graph from a list of attractions
    // Every attraction connects to every other attraction
    pub fn build_complete(attractions: Vec<Attraction>) -> Self {
        let mut graph = Graph::new();

        // Insert all attractions
        for a in &attractions {
            graph.attractions.insert(a.id, a.clone());
            graph.adjacency.insert(a.id, Vec::new());
        }

        // Build edges between every pair (complete graph)
        let ids: Vec<u32> = graph.attractions.keys().cloned().collect();
        for &from in &ids {
            for &to in &ids {
                if from == to { continue; }
                let a = &graph.attractions[&from];
                let b = &graph.attractions[&to];
                let dist_km = haversine(
                    a.location.lat, a.location.lng,
                    b.location.lat, b.location.lng,
                );
                // Assume average speed of 30 km/h in a city
                let travel_time = (dist_km / 30.0 * 60.0) as u32;
                let cost = dist_km * 0.5; // $0.50 per km (taxi estimate)
                graph.adjacency.get_mut(&from).unwrap().push(Edge { to, travel_time, cost });
            }
        }

        graph
    }

    pub fn travel_time(&self, from: u32, to: u32) -> u32 {
        self.adjacency[&from]
            .iter()
            .find(|e| e.to == to)
            .map(|e| e.travel_time)
            .unwrap_or(u32::MAX)
    }

    pub fn travel_cost(&self, from: u32, to: u32) -> f64 {
        self.adjacency[&from]
            .iter()
            .find(|e| e.to == to)
            .map(|e| e.cost)
            .unwrap_or(f64::MAX)
    }
}