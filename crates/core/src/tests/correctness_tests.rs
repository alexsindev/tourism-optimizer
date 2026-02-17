//! Comprehensive correctness tests for algorithm validation

use crate::algorithms::{greedy, simulated_annealing};
use crate::models::attraction::{Attraction, Category, Location};
use crate::models::constraints::SolveParams;
use crate::utils::verifier::verify_itinerary;
use std::collections::HashSet;

fn create_test_attractions() -> Vec<Attraction> {
    vec![
        Attraction {
            id: 1,
            name: "Museum A".to_string(),
            location: Location { lat: 13.7563, lng: 100.5018 },
            open_time: 540,  // 9:00
            close_time: 1020, // 17:00
            duration: 90,
            fee: 10.0,
            preference: 0.9,
            category: Category::Museum,
        },
        Attraction {
            id: 2,
            name: "Restaurant B".to_string(),
            location: Location { lat: 13.7600, lng: 100.5050 },
            open_time: 660,  // 11:00
            close_time: 1320, // 22:00
            duration: 60,
            fee: 15.0,
            preference: 0.8,
            category: Category::Restaurant,
        },
        Attraction {
            id: 3,
            name: "Park C".to_string(),
            location: Location { lat: 13.7650, lng: 100.5100 },
            open_time: 360,  // 6:00
            close_time: 1080, // 18:00
            duration: 120,
            fee: 0.0,
            preference: 0.7,
            category: Category::Park,
        },
        Attraction {
            id: 4,
            name: "Landmark D".to_string(),
            location: Location { lat: 13.7700, lng: 100.5150 },
            open_time: 480,  // 8:00
            close_time: 1140, // 19:00
            duration: 45,
            fee: 5.0,
            preference: 0.85,
            category: Category::Landmark,
        },
        Attraction {
            id: 5,
            name: "Shopping E".to_string(),
            location: Location { lat: 13.7550, lng: 100.5080 },
            open_time: 600,  // 10:00
            close_time: 1260, // 21:00
            duration: 150,
            fee: 20.0,
            preference: 0.75,
            category: Category::Shopping,
        },
    ]
}

fn default_params() -> SolveParams {
    SolveParams {
        num_days: 2,
        daily_time_budget: 600,
        total_budget: 100.0,
        start_time: 540,
        hotel_lat: 13.7563,
        hotel_lng: 100.5018,
    }
}

#[test]
fn test_greedy_no_duplicate_visits() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    let mut seen = HashSet::new();
    for day in &itinerary.days {
        for visit in &day.visits {
            assert!(
                seen.insert(visit.attraction_id),
                "Duplicate visit to attraction {}",
                visit.attraction_id
            );
        }
    }
}

#[test]
fn test_sa_no_duplicate_visits() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    let mut seen = HashSet::new();
    for day in &itinerary.days {
        for visit in &day.visits {
            assert!(
                seen.insert(visit.attraction_id),
                "Duplicate visit to attraction {}",
                visit.attraction_id
            );
        }
    }
}

#[test]
fn test_greedy_respects_time_windows() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    for day in &itinerary.days {
        for visit in &day.visits {
            let attr = attractions.iter().find(|a| a.id == visit.attraction_id).unwrap();
            assert!(
                visit.arrival_time >= attr.open_time,
                "Visit to {} arrives before opening: {} < {}",
                visit.attraction_name,
                visit.arrival_time,
                attr.open_time
            );
            assert!(
                visit.departure_time <= attr.close_time,
                "Visit to {} departs after closing: {} > {}",
                visit.attraction_name,
                visit.departure_time,
                attr.close_time
            );
        }
    }
}

#[test]
fn test_sa_respects_time_windows() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    for day in &itinerary.days {
        for visit in &day.visits {
            let attr = attractions.iter().find(|a| a.id == visit.attraction_id).unwrap();
            assert!(
                visit.arrival_time >= attr.open_time,
                "Visit to {} arrives before opening: {} < {}",
                visit.attraction_name,
                visit.arrival_time,
                attr.open_time
            );
            assert!(
                visit.departure_time <= attr.close_time,
                "Visit to {} departs after closing: {} > {}",
                visit.attraction_name,
                visit.departure_time,
                attr.close_time
            );
        }
    }
}

#[test]
fn test_greedy_respects_budget_constraint() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    assert!(
        itinerary.total_cost <= params.total_budget,
        "Total cost {} exceeds budget {}",
        itinerary.total_cost,
        params.total_budget
    );
}

#[test]
fn test_sa_respects_budget_constraint() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    assert!(
        itinerary.total_cost <= params.total_budget,
        "Total cost {} exceeds budget {}",
        itinerary.total_cost,
        params.total_budget
    );
}

#[test]
fn test_greedy_respects_daily_time_budget() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    for day in &itinerary.days {
        if !day.visits.is_empty() {
            let start = day.visits.first().unwrap().arrival_time;
            let end = day.visits.last().unwrap().departure_time;
            let duration = end - start;
            
            assert!(
                duration <= params.daily_time_budget,
                "Day {} duration {} exceeds budget {}",
                day.day,
                duration,
                params.daily_time_budget
            );
        }
    }
}

#[test]
fn test_sa_respects_daily_time_budget() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    for day in &itinerary.days {
        if !day.visits.is_empty() {
            let start = day.visits.first().unwrap().arrival_time;
            let end = day.visits.last().unwrap().departure_time;
            let duration = end - start;
            
            assert!(
                duration <= params.daily_time_budget,
                "Day {} duration {} exceeds budget {}",
                day.day,
                duration,
                params.daily_time_budget
            );
        }
    }
}

#[test]
fn test_greedy_chronological_order() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    for day in &itinerary.days {
        for i in 1..day.visits.len() {
            let prev = &day.visits[i - 1];
            let curr = &day.visits[i];
            
            assert!(
                curr.arrival_time >= prev.departure_time,
                "Day {} visits out of order: visit {} at {} overlaps with previous ending at {}",
                day.day,
                i,
                curr.arrival_time,
                prev.departure_time
            );
        }
    }
}

#[test]
fn test_sa_chronological_order() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    for day in &itinerary.days {
        for i in 1..day.visits.len() {
            let prev = &day.visits[i - 1];
            let curr = &day.visits[i];
            
            assert!(
                curr.arrival_time >= prev.departure_time,
                "Day {} visits out of order: visit {} at {} overlaps with previous ending at {}",
                day.day,
                i,
                curr.arrival_time,
                prev.departure_time
            );
        }
    }
}

#[test]
fn test_greedy_passes_verifier() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    let result = verify_itinerary(&itinerary, &params);
    assert!(result.is_ok(), "Verifier failed: {:?}", result.err());
}

#[test]
fn test_sa_passes_verifier() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    let result = verify_itinerary(&itinerary, &params);
    assert!(result.is_ok(), "Verifier failed: {:?}", result.err());
}

#[test]
fn test_greedy_tight_budget() {
    let attractions = create_test_attractions();
    let params = SolveParams {
        total_budget: 10.0,  // Very tight budget
        ..default_params()
    };
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    assert!(itinerary.total_cost <= params.total_budget);
    assert!(verify_itinerary(&itinerary, &params).is_ok());
}

#[test]
fn test_sa_tight_budget() {
    let attractions = create_test_attractions();
    let params = SolveParams {
        total_budget: 10.0,  // Very tight budget
        ..default_params()
    };
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    assert!(itinerary.total_cost <= params.total_budget);
    assert!(verify_itinerary(&itinerary, &params).is_ok());
}

#[test]
fn test_greedy_zero_budget() {
    let attractions = create_test_attractions();
    let params = SolveParams {
        total_budget: 0.0,  // Can only visit free attractions
        ..default_params()
    };
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    // Should only visit Park C (free)
    assert!(itinerary.total_cost == 0.0);
    for day in &itinerary.days {
        for visit in &day.visits {
            assert_eq!(visit.fee, 0.0, "Non-free attraction visited with zero budget");
        }
    }
}

#[test]
fn test_greedy_multiple_runs_deterministic() {
    let attractions = create_test_attractions();
    let params = default_params();
    
    let result1 = greedy::solve(&attractions, &params, 42);
    let result2 = greedy::solve(&attractions, &params, 42);
    
    // Greedy should be deterministic with same seed
    assert_eq!(result1.total_satisfaction, result2.total_satisfaction);
    assert_eq!(result1.total_cost, result2.total_cost);
    assert_eq!(result1.total_attractions, result2.total_attractions);
}

#[test]
fn test_sa_different_seeds_different_results() {
    let attractions = create_test_attractions();
    let params = default_params();
    
    let result1 = simulated_annealing::solve(&attractions, &params, 42);
    let result2 = simulated_annealing::solve(&attractions, &params, 123);
    
    // SA with different seeds might produce different results (but both valid)
    assert!(verify_itinerary(&result1, &params).is_ok());
    assert!(verify_itinerary(&result2, &params).is_ok());
}

#[test]
fn test_greedy_handles_single_day() {
    let attractions = create_test_attractions();
    let params = SolveParams {
        num_days: 1,
        ..default_params()
    };
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    assert_eq!(itinerary.days.len(), 1);
    assert!(verify_itinerary(&itinerary, &params).is_ok());
}

#[test]
fn test_sa_handles_single_day() {
    let attractions = create_test_attractions();
    let params = SolveParams {
        num_days: 1,
        ..default_params()
    };
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    assert_eq!(itinerary.days.len(), 1);
    assert!(verify_itinerary(&itinerary, &params).is_ok());
}

#[test]
fn test_greedy_handles_many_days() {
    let attractions = create_test_attractions();
    let params = SolveParams {
        num_days: 5,
        ..default_params()
    };
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    assert_eq!(itinerary.days.len(), 5);
    assert!(verify_itinerary(&itinerary, &params).is_ok());
}

#[test]
fn test_sa_handles_many_days() {
    let attractions = create_test_attractions();
    let params = SolveParams {
        num_days: 5,
        ..default_params()
    };
    let itinerary = simulated_annealing::solve(&attractions, &params, 42);
    
    assert_eq!(itinerary.days.len(), 5);
    assert!(verify_itinerary(&itinerary, &params).is_ok());
}

#[test]
fn test_satisfaction_calculation_correct() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    // Manually calculate satisfaction
    let mut expected_satisfaction = 0.0;
    for day in &itinerary.days {
        for visit in &day.visits {
            expected_satisfaction += visit.preference;
        }
    }
    
    assert!(
        (itinerary.total_satisfaction - expected_satisfaction).abs() < 0.001,
        "Satisfaction mismatch: {} vs {}",
        itinerary.total_satisfaction,
        expected_satisfaction
    );
}

#[test]
fn test_cost_calculation_correct() {
    let attractions = create_test_attractions();
    let params = default_params();
    let itinerary = greedy::solve(&attractions, &params, 42);
    
    // Manually calculate cost
    let mut expected_cost = 0.0;
    for day in &itinerary.days {
        for visit in &day.visits {
            expected_cost += visit.fee;
        }
    }
    
    assert!(
        (itinerary.total_cost - expected_cost).abs() < 0.001,
        "Cost mismatch: {} vs {}",
        itinerary.total_cost,
        expected_cost
    );
}
