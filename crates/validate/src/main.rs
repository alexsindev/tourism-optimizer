//! Correctness validation tool for tourism optimizer algorithms

use colored::*;
use core::algorithms::{greedy, simulated_annealing};
use core::models::attraction::Attraction;
use core::models::constraints::SolveParams;
use core::models::itinerary::Itinerary;
use core::utils::verifier::verify_itinerary;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!("{}", "=== Tourism Optimizer Correctness Validator ===".bold().cyan());
    println!();

    // Load all datasets
    let datasets = vec![
        ("small", "data/datasets/small.json"),
        ("medium", "data/datasets/medium.json"),
        ("large", "data/datasets/large.json"),
    ];

    let mut total_tests = 0;
    let mut passed_tests = 0;

    for (name, path) in datasets {
        println!("{}", format!("Testing dataset: {}", name).bold().yellow());
        
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                println!("  {} Failed to load dataset: {}", "✗".red(), e);
                continue;
            }
        };

        let attractions: Vec<Attraction> = match serde_json::from_str(&content) {
            Ok(a) => a,
            Err(e) => {
                println!("  {} Failed to parse dataset: {}", "✗".red(), e);
                continue;
            }
        };

        println!("  Loaded {} attractions", attractions.len());
        println!();

        // Test configurations
        let configs = vec![
            ("Default", SolveParams::default()),
            (
                "Tight Budget",
                SolveParams {
                    total_budget: 50.0,
                    ..Default::default()
                },
            ),
            (
                "Short Days",
                SolveParams {
                    daily_time_budget: 300,
                    ..Default::default()
                },
            ),
            (
                "Many Days",
                SolveParams {
                    num_days: 5,
                    ..Default::default()
                },
            ),
        ];

        for (config_name, params) in configs {
            println!("  Configuration: {}", config_name.bold());
            
            // Test Greedy
            print!("    Greedy: ");
            total_tests += 1;
            let greedy_result = greedy::solve(&attractions, &params, 42);
            let greedy_checks = run_checks(&greedy_result, &attractions, &params);
            if greedy_checks {
                println!("{}", "✓ PASS".green());
                passed_tests += 1;
            } else {
                println!("{}", "✗ FAIL".red());
            }

            // Test Simulated Annealing
            print!("    Simulated Annealing: ");
            total_tests += 1;
            let sa_result = simulated_annealing::solve(&attractions, &params, 42);
            let sa_checks = run_checks(&sa_result, &attractions, &params);
            if sa_checks {
                println!("{}", "✓ PASS".green());
                passed_tests += 1;
            } else {
                println!("{}", "✗ FAIL".red());
            }
        }
        println!();
    }

    // Summary
    println!("{}", "=== Summary ===".bold().cyan());
    println!("Total tests: {}", total_tests);
    println!("Passed: {}", format!("{}", passed_tests).green());
    println!("Failed: {}", format!("{}", total_tests - passed_tests).red());
    
    if passed_tests == total_tests {
        println!();
        println!("{}", "🎉 All tests passed!".bold().green());
        std::process::exit(0);
    } else {
        println!();
        println!("{}", "❌ Some tests failed!".bold().red());
        std::process::exit(1);
    }
}

fn run_checks(itinerary: &Itinerary, attractions: &[Attraction], params: &SolveParams) -> bool {
    let mut all_passed = true;

    // Check 1: Verifier passes
    if let Err(e) = verify_itinerary(itinerary, params) {
        println!();
        println!("      {} Verifier failed: {}", "✗".red(), e);
        all_passed = false;
    }

    // Check 2: No duplicate visits
    let mut seen = HashSet::new();
    for day in &itinerary.days {
        for visit in &day.visits {
            if !seen.insert(visit.attraction_id) {
                println!();
                println!(
                    "      {} Duplicate visit to attraction {}",
                    "✗".red(),
                    visit.attraction_id
                );
                all_passed = false;
            }
        }
    }

    // Check 3: Time windows respected
    for day in &itinerary.days {
        for visit in &day.visits {
            if let Some(attr) = attractions.iter().find(|a| a.id == visit.attraction_id) {
                if visit.arrival_time < attr.open_time || visit.departure_time > attr.close_time {
                    println!();
                    println!(
                        "      {} Time window violated for {}",
                        "✗".red(),
                        visit.attraction_name
                    );
                    all_passed = false;
                }
            }
        }
    }

    // Check 4: Chronological order
    for day in &itinerary.days {
        for i in 1..day.visits.len() {
            if day.visits[i].arrival_time < day.visits[i - 1].departure_time {
                println!();
                println!("      {} Visits overlap on day {}", "✗".red(), day.day);
                all_passed = false;
            }
        }
    }

    // Check 5: Budget respected
    if itinerary.total_cost > params.total_budget {
        println!();
        println!(
            "      {} Budget exceeded: ${:.2} > ${:.2}",
            "✗".red(),
            itinerary.total_cost,
            params.total_budget
        );
        all_passed = false;
    }

    // Check 6: Daily time budget respected
    for day in &itinerary.days {
        if !day.visits.is_empty() {
            let duration =
                day.visits.last().unwrap().departure_time - day.visits.first().unwrap().arrival_time;
            if duration > params.daily_time_budget {
                println!();
                println!(
                    "      {} Daily time budget exceeded on day {}: {} > {}",
                    "✗".red(),
                    day.day,
                    duration,
                    params.daily_time_budget
                );
                all_passed = false;
            }
        }
    }

    // Check 7: Satisfaction calculation correct
    let calculated_satisfaction: f64 = itinerary
        .days
        .iter()
        .flat_map(|d| &d.visits)
        .map(|v| v.preference)
        .sum();
    
    if (itinerary.total_satisfaction - calculated_satisfaction).abs() > 0.001 {
        println!();
        println!(
            "      {} Satisfaction calculation incorrect: {:.3} vs {:.3}",
            "✗".red(),
            itinerary.total_satisfaction,
            calculated_satisfaction
        );
        all_passed = false;
    }

    // Check 8: Cost calculation correct
    let calculated_cost: f64 = itinerary
        .days
        .iter()
        .flat_map(|d| &d.visits)
        .map(|v| v.fee)
        .sum();
    
    if (itinerary.total_cost - calculated_cost).abs() > 0.001 {
        println!();
        println!(
            "      {} Cost calculation incorrect: {:.2} vs {:.2}",
            "✗".red(),
            itinerary.total_cost,
            calculated_cost
        );
        all_passed = false;
    }

    all_passed
}
