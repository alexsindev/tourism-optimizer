use std::collections::HashMap;
use crate::models::attraction::Attraction;
use crate::utils::distance::haversine_distance;

const CITY_SPEED_KMH: f64 = 30.0;

#[derive(Debug, Clone)]
pub struct Edge {
    pub to: u32,
    pub travel_time: u32,  // minutes
    pub cost: f64,
}

#[derive(Debug, Clone)]
pub struct Graph {
    adj: HashMap<u32, Vec<Edge>>,
}

impl Graph {
    pub fn from_attractions(attractions: &[Attraction]) -> Self {
        let mut adj = HashMap::new();
        
        for from in attractions {
            let mut edges = Vec::new();
            for to in attractions {
                if from.id != to.id {
                    let distance_km = haversine_distance(
                        from.location.lat,
                        from.location.lng,
                        to.location.lat,
                        to.location.lng,
                    );
                    let travel_time = ((distance_km / CITY_SPEED_KMH) * 60.0).ceil() as u32;
                    edges.push(Edge {
                        to: to.id,
                        travel_time,
                        cost: 0.0,  // no travel cost in this model
                    });
                }
            }
            adj.insert(from.id, edges);
        }
        
        Self { adj }
    }

    pub fn travel_time(&self, from: u32, to: u32) -> Option<u32> {
        self.adj.get(&from)?
            .iter()
            .find(|e| e.to == to)
            .map(|e| e.travel_time)
    }

    pub fn neighbors(&self, node: u32) -> Option<&Vec<Edge>> {
        self.adj.get(&node)
    }
}