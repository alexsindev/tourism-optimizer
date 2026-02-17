use crate::algorithms::greedy;
use crate::models::attraction::{Attraction};
use crate::models::itinerary::{ConvergencePoint, DayPlan, Itinerary, Visit};
use crate::models::constraints::SolveParams;
use crate::utils::distance::haversine_distance;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::time::Instant;

const CITY_SPEED_KMH: f64 = 30.0;
const INITIAL_TEMP: f64 = 100.0;
const COOLING_RATE: f64 = 0.995;
const MIN_TEMP: f64 = 0.01;
const MAX_ITERATIONS: usize = 10_000;

type Chromosome = Vec<Vec<u32>>;

pub fn solve(attractions: &[Attraction], params: &SolveParams, seed: u64) -> Itinerary {
    let start = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    
    // Get greedy initial solution
    let greedy_solution = greedy::solve(attractions, params, seed);
    let mut best_chromosome = itinerary_to_chromosome(&greedy_solution);
    let mut best_fitness = evaluate_fitness(&best_chromosome, attractions, params);
    
    let mut current = best_chromosome.clone();
    let mut current_fitness = best_fitness;
    
    let mut temperature = INITIAL_TEMP;
    let mut convergence = Vec::new();
    let mut iteration = 0;
    
    while temperature > MIN_TEMP && iteration < MAX_ITERATIONS {
        // Generate neighbor
        let neighbor = mutate(&current, attractions, params, &mut rng);
        let neighbor_fitness = evaluate_fitness(&neighbor, attractions, params);
        
        let delta = neighbor_fitness - current_fitness;
        
        // Accept or reject - fixed gen_range call
        let accept = if delta > 0.0 {
            true
        } else {
            let prob = (delta / temperature).exp();
            rng.gen_range(0.0..1.0) < prob
        };
        
        if accept {
            current = neighbor;
            current_fitness = neighbor_fitness;
            
            if current_fitness > best_fitness {
                best_chromosome = current.clone();
                best_fitness = current_fitness;
            }
        }
        
        // Record convergence every 50 iterations
        if iteration % 50 == 0 {
            convergence.push(ConvergencePoint {
                iteration: iteration as u32,
                satisfaction: best_fitness,
                temperature,
            });
        }
        
        temperature *= COOLING_RATE;
        iteration += 1;
    }
    
    // Convert best chromosome to itinerary
    let mut itinerary = build_itinerary(&best_chromosome, attractions, params);
    itinerary.algorithm_used = "simulated_annealing".to_string();
    itinerary.computation_ms = start.elapsed().as_millis();
    itinerary.convergence_data = Some(convergence);
    itinerary.compute_totals();
    
    itinerary
}

fn itinerary_to_chromosome(itinerary: &Itinerary) -> Chromosome {
    itinerary.days.iter()
        .map(|day| day.visits.iter().map(|v| v.attraction_id).collect())
        .collect()
}

fn evaluate_fitness(chromosome: &Chromosome, attractions: &[Attraction], params: &SolveParams) -> f64 {
    let mut fitness = 0.0;
    let mut total_cost = 0.0;
    let mut seen = HashSet::new();
    
    for (_day_idx, day_attractions) in chromosome.iter().enumerate() {
        let mut current_time = params.start_time;
        let mut current_lat = params.hotel_lat;
        let mut current_lng = params.hotel_lng;
        
        for &attr_id in day_attractions {
            // Duplicate penalty
            if !seen.insert(attr_id) {
                fitness -= 1.0;
                continue;
            }
            
            let attr = match attractions.iter().find(|a| a.id == attr_id) {
                Some(a) => a,
                None => continue,
            };
            
            // Calculate travel time
            let distance = haversine_distance(current_lat, current_lng, attr.location.lat, attr.location.lng);
            let travel_time = ((distance / CITY_SPEED_KMH) * 60.0).ceil() as u32;
            let arrival = current_time + travel_time;
            let departure = arrival + attr.duration;
            
            // Time window check
            if arrival < attr.open_time || departure > attr.close_time {
                fitness -= 0.5;
                continue;
            }
            
            // Budget check
            if total_cost + attr.fee > params.total_budget {
                fitness -= 0.2 * (total_cost + attr.fee - params.total_budget);
                continue;
            }
            
            // Daily time budget check
            let day_end = departure - params.start_time;
            if day_end > params.daily_time_budget {
                fitness -= 0.3;
                continue;
            }
            
            // Valid visit - add preference
            fitness += attr.preference;
            total_cost += attr.fee;
            current_time = departure;
            current_lat = attr.location.lat;
            current_lng = attr.location.lng;
        }
    }
    
    fitness
}

fn mutate(chromosome: &Chromosome, attractions: &[Attraction], params: &SolveParams, rng: &mut StdRng) -> Chromosome {
    let mut new = chromosome.clone();
    let mutation_type = rng.gen_range(0..3);
    
    match mutation_type {
        0 => swap_within_day(&mut new, rng),
        1 => move_between_days(&mut new, rng),
        2 => insert_unvisited(&mut new, attractions, params, rng),
        _ => unreachable!(),
    }
    
    new
}

fn swap_within_day(chromosome: &mut Chromosome, rng: &mut StdRng) {
    if chromosome.is_empty() {
        return;
    }
    
    let day_idx = rng.gen_range(0..chromosome.len());
    let day = &mut chromosome[day_idx];
    
    if day.len() >= 2 {
        let i = rng.gen_range(0..day.len());
        let j = rng.gen_range(0..day.len());
        day.swap(i, j);
    }
}

fn move_between_days(chromosome: &mut Chromosome, rng: &mut StdRng) {
    if chromosome.len() < 2 {
        return;
    }
    
    let from_day = rng.gen_range(0..chromosome.len());
    let to_day = rng.gen_range(0..chromosome.len());
    
    if from_day != to_day && !chromosome[from_day].is_empty() {
        let idx = rng.gen_range(0..chromosome[from_day].len());
        let attr_id = chromosome[from_day].remove(idx);
        chromosome[to_day].push(attr_id);
    }
}

fn insert_unvisited(chromosome: &mut Chromosome, attractions: &[Attraction], _params: &SolveParams, rng: &mut StdRng) {
    let visited: HashSet<u32> = chromosome.iter().flat_map(|d| d.iter()).copied().collect();
    let unvisited: Vec<u32> = attractions.iter()
        .map(|a| a.id)
        .filter(|id| !visited.contains(id))
        .collect();
    
    if !unvisited.is_empty() && !chromosome.is_empty() {
        let attr_id = unvisited[rng.gen_range(0..unvisited.len())];
        let day_idx = rng.gen_range(0..chromosome.len());
        chromosome[day_idx].push(attr_id);
    }
}

fn build_itinerary(chromosome: &Chromosome, attractions: &[Attraction], params: &SolveParams) -> Itinerary {
    let mut days = Vec::new();
    let mut total_cost = 0.0;
    
    for (day_num, day_attrs) in chromosome.iter().enumerate() {
        let mut visits = Vec::new();
        let mut current_time = params.start_time;
        let mut current_lat = params.hotel_lat;
        let mut current_lng = params.hotel_lng;
        let mut day_cost = 0.0;
        let mut day_satisfaction = 0.0;
        let mut travel_time_sum = 0;
        
        for &attr_id in day_attrs {
            let attr = match attractions.iter().find(|a| a.id == attr_id) {
                Some(a) => a,
                None => continue,
            };
            
            let distance = haversine_distance(current_lat, current_lng, attr.location.lat, attr.location.lng);
            let travel_time = ((distance / CITY_SPEED_KMH) * 60.0).ceil() as u32;
            let arrival = current_time + travel_time;
            let departure = arrival + attr.duration;
            
            // Hard constraint checks
            if arrival < attr.open_time || departure > attr.close_time {
                continue;
            }
            if total_cost + attr.fee > params.total_budget {
                continue;
            }
            if departure - params.start_time > params.daily_time_budget {
                continue;
            }
            
            visits.push(Visit {
                attraction_id: attr.id,
                attraction_name: attr.name.clone(),
                arrival_time: arrival,
                departure_time: departure,
                fee: attr.fee,
                preference: attr.preference,
                category: format!("{:?}", attr.category),
            });
            
            total_cost += attr.fee;
            day_cost += attr.fee;
            day_satisfaction += attr.preference;
            travel_time_sum += travel_time;
            current_time = departure;
            current_lat = attr.location.lat;
            current_lng = attr.location.lng;
        }
        
        days.push(DayPlan {
            day: (day_num + 1) as u32,
            visits,
            total_travel_time: travel_time_sum,
            total_cost: day_cost,
            total_satisfaction: day_satisfaction,
        });
    }
    
    Itinerary {
        days,
        total_satisfaction: 0.0,
        total_cost,
        total_attractions: 0,
        algorithm_used: "simulated_annealing".to_string(),
        computation_ms: 0,
        convergence_data: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::attraction::{Location, Category};
    use crate::utils::verifier::verify_itinerary;

    fn create_test_attraction(id: u32, lat: f64, lng: f64, preference: f64) -> Attraction {
        Attraction {
            id,
            name: format!("Attraction {}", id),
            location: Location { lat, lng },
            open_time: 540,
            close_time: 1020,
            duration: 60,
            fee: 5.0,
            preference,
            category: Category::Museum,
        }
    }

    #[test]
    fn test_sa_produces_valid_itinerary() {
        let attractions = vec![
            create_test_attraction(1, 13.7563, 100.5018, 0.9),
            create_test_attraction(2, 13.7600, 100.5050, 0.8),
            create_test_attraction(3, 13.7650, 100.5100, 0.7),
        ];
        
        let params = SolveParams::default();
        let result = solve(&attractions, &params, 42);
        
        assert!(verify_itinerary(&result, &params).is_ok());
    }

    #[test]
    fn test_sa_respects_budget() {
        let attractions = vec![
            create_test_attraction(1, 13.7563, 100.5018, 0.9),
            create_test_attraction(2, 13.7600, 100.5050, 0.8),
        ];
        
        let params = SolveParams {
            num_days: 1,
            daily_time_budget: 600,
            total_budget: 5.0,
            ..Default::default()
        };
        
        let result = solve(&attractions, &params, 42);
        assert!(result.total_cost <= params.total_budget);
    }

    #[test]
    fn test_sa_has_convergence_data() {
        let attractions = vec![
            create_test_attraction(1, 13.7563, 100.5018, 0.9),
            create_test_attraction(2, 13.7600, 100.5050, 0.8),
        ];
        
        let params = SolveParams::default();
        let result = solve(&attractions, &params, 42);
        
        assert!(result.convergence_data.is_some());
        let convergence = result.convergence_data.unwrap();
        assert!(!convergence.is_empty());
    }
}
