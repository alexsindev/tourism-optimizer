use crate::models::itinerary::Itinerary;
use crate::models::constraints::SolveParams;
use std::collections::HashSet;

pub fn verify_itinerary(itinerary: &Itinerary, params: &SolveParams) -> Result<(), String> {
    let mut seen_ids = HashSet::new();

    // Check no duplicates across all days
    for day in &itinerary.days {
        for visit in &day.visits {
            if !seen_ids.insert(visit.attraction_id) {
                return Err(format!(
                    "Duplicate visit to attraction {} on day {}",
                    visit.attraction_id, day.day
                ));
            }
        }
    }

    // Check each day
    for day in &itinerary.days {
        // Check chronological order and time windows
        for i in 0..day.visits.len() {
            let visit = &day.visits[i];
            
            // Time window check (we don't have attraction data here, assume arrival/departure are valid)
            if visit.arrival_time > visit.departure_time {
                return Err(format!(
                    "Invalid time: arrival {} > departure {} for attraction {}",
                    visit.arrival_time, visit.departure_time, visit.attraction_id
                ));
            }

            // Check chronological order
            if i > 0 {
                let prev = &day.visits[i - 1];
                if visit.arrival_time < prev.departure_time {
                    return Err(format!(
                        "Overlapping visits on day {}: attraction {} starts before {} ends",
                        day.day, visit.attraction_id, prev.attraction_id
                    ));
                }
            }
        }

        // Check daily time budget
        if !day.visits.is_empty() {
            let first_arrival = day.visits[0].arrival_time;
            let last_departure = day.visits.last().unwrap().departure_time;
            let total_time = last_departure - first_arrival;
            
            if total_time > params.daily_time_budget {
                return Err(format!(
                    "Day {} exceeds time budget: {} > {}",
                    day.day, total_time, params.daily_time_budget
                ));
            }
        }
    }

    // Check total budget
    if itinerary.total_cost > params.total_budget {
        return Err(format!(
            "Total cost ${:.2} exceeds budget ${:.2}",
            itinerary.total_cost, params.total_budget
        ));
    }

    Ok(())
}
