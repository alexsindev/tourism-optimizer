use crate::data_structures::{IntervalTree, PriorityQueue};
use crate::models::attraction::{Attraction};
use crate::models::itinerary::{DayPlan, Visit};
use crate::models::graph::Graph;
use crate::models::constraints::SolveParams;
use crate::models::itinerary::Itinerary;
use crate::utils::distance::haversine_distance;
use std::collections::HashSet;
use std::time::Instant;

const CITY_SPEED_KMH: f64 = 30.0;

pub fn solve(attractions: &[Attraction], params: &SolveParams, _seed: u64) -> Itinerary {
    let start = Instant::now();
    
    // Build interval tree for efficient time window queries
    let mut interval_tree = IntervalTree::new();
    for attr in attractions {
        interval_tree.insert(crate::data_structures::interval_tree::Interval {
            start: attr.open_time,
            end: attr.close_time,
            data: attr.id,
        });
    }

    // Build graph for travel time lookups
    let graph = Graph::from_attractions(attractions);
    
    let mut visited = HashSet::new();
    let mut days = Vec::new();
    let mut total_cost = 0.0;

    for day in 1..=params.num_days {
        let day_plan = solve_day(
            day,
            attractions,
            &interval_tree,
            &graph,
            params,
            &mut visited,
            &mut total_cost,
        );
        days.push(day_plan);
    }

    let mut itinerary = Itinerary {
        days,
        total_satisfaction: 0.0,
        total_cost,
        total_attractions: 0,
        algorithm_used: "greedy".to_string(),
        computation_ms: start.elapsed().as_millis(),
        convergence_data: None,
    };
    
    itinerary.compute_totals();
    itinerary
}

fn solve_day(
    day: u32,
    attractions: &[Attraction],
    interval_tree: &IntervalTree,
    _graph: &Graph,
    params: &SolveParams,
    visited: &mut HashSet<u32>,
    total_cost: &mut f64,
) -> DayPlan {
    let mut visits = Vec::new();
    let mut current_time = params.start_time;
    let mut current_lat = params.hotel_lat;
    let mut current_lng = params.hotel_lng;
    let mut day_cost = 0.0;
    let mut day_satisfaction = 0.0;
    let mut travel_time_sum = 0.0;

    loop {
        // Query attractions open at current time
        let open_ids = interval_tree.query_open_at(current_time);
        
        // Build priority queue of feasible attractions
        let mut pq = PriorityQueue::new();
        
        for id in open_ids {
            if visited.contains(&id) {
                continue;
            }
            
            let attr = attractions.iter().find(|a| a.id == id).unwrap();
            
            // Calculate travel time from current position
            let distance = haversine_distance(current_lat, current_lng, attr.location.lat, attr.location.lng);
            let travel_time = ((distance / CITY_SPEED_KMH) * 60.0).ceil() as u32;
            let arrival = current_time + travel_time;
            let departure = arrival + attr.duration;
            
            // Check feasibility
            if arrival < attr.open_time || departure > attr.close_time {
                continue;
            }
            
            // Check budget
            if *total_cost + attr.fee > params.total_budget {
                continue;
            }
            
            // Check daily time budget (including return to hotel)
            let return_distance = haversine_distance(attr.location.lat, attr.location.lng, params.hotel_lat, params.hotel_lng);
            let return_time = ((return_distance / CITY_SPEED_KMH) * 60.0).ceil() as u32;
            let total_time_if_visit = departure + return_time - params.start_time;
            
            if total_time_if_visit > params.daily_time_budget {
                continue;
            }
            
            // Score: preference / (travel_time + 1)
            let score = attr.preference / (travel_time as f64 + 1.0);
            pq.push(score, (id, travel_time));
        }
        
        // Pop best candidate
        if let Some((best_id, travel_time)) = pq.pop() {
            let attr = attractions.iter().find(|a| a.id == best_id).unwrap();
            
            let arrival = current_time + travel_time;
            let departure = arrival + attr.duration;
            
            visits.push(Visit {
                attraction_id: attr.id,
                attraction_name: attr.name.clone(),
                arrival_time: arrival,
                departure_time: departure,
                fee: attr.fee,
                preference: attr.preference,
                category: format!("{:?}", attr.category),
            });
            
            visited.insert(best_id);
            *total_cost += attr.fee;
            day_cost += attr.fee;
            day_satisfaction += attr.preference;
            travel_time_sum += travel_time as f64;
            
            current_time = departure;
            current_lat = attr.location.lat;
            current_lng = attr.location.lng;
        } else {
            break;
        }
    }

    DayPlan {
        day,
        visits,
        total_travel_time: travel_time_sum as u32,
        total_cost: day_cost,
        total_satisfaction: day_satisfaction,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::attraction::{Location, Category};

    fn create_test_attraction(id: u32, lat: f64, lng: f64, preference: f64) -> Attraction {
        Attraction {
            id,
            name: format!("Attraction {}", id),
            location: Location { lat, lng },
            open_time: 540,  // 9 AM
            close_time: 1020, // 5 PM
            duration: 60,
            fee: 5.0,
            preference,
            category: Category::Museum,
        }
    }

    #[test]
    fn test_greedy_visits_at_least_one() {
        let attractions = vec![
            create_test_attraction(1, 13.7563, 100.5018, 0.9),
            create_test_attraction(2, 13.7600, 100.5050, 0.8),
        ];
        
        let params = SolveParams {
            num_days: 1,
            daily_time_budget: 600,
            total_budget: 50.0,
            start_time: 540,
            hotel_lat: 13.7563,
            hotel_lng: 100.5018,
        };
        
        let result = solve(&attractions, &params, 42);
        assert!(result.total_attractions > 0);
    }

    #[test]
    fn test_greedy_no_duplicates() {
        let attractions = vec![
            create_test_attraction(1, 13.7563, 100.5018, 0.9),
            create_test_attraction(2, 13.7600, 100.5050, 0.8),
            create_test_attraction(3, 13.7650, 100.5100, 0.7),
        ];
        
        let params = SolveParams {
            num_days: 2,
            daily_time_budget: 600,
            total_budget: 100.0,
            start_time: 540,
            hotel_lat: 13.7563,
            hotel_lng: 100.5018,
        };
        
        let result = solve(&attractions, &params, 42);
        let mut seen = HashSet::new();
        for day in &result.days {
            for visit in &day.visits {
                assert!(seen.insert(visit.attraction_id), "Duplicate visit found");
            }
        }
    }

    #[test]
    fn test_greedy_respects_budget() {
        let attractions = vec![
            create_test_attraction(1, 13.7563, 100.5018, 0.9),
            create_test_attraction(2, 13.7600, 100.5050, 0.8),
        ];
        
        let params = SolveParams {
            num_days: 1,
            daily_time_budget: 600,
            total_budget: 5.0,  // Only enough for one
            start_time: 540,
            hotel_lat: 13.7563,
            hotel_lng: 100.5018,
        };
        
        let result = solve(&attractions, &params, 42);
        assert!(result.total_cost <= params.total_budget);
        assert!(result.total_attractions <= 1);
    }
}