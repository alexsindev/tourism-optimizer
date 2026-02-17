use crate::models::attraction::Attraction;
use crate::models::itinerary::Itinerary;
use crate::models::constraints::SolveParams;
use crate::utils::verifier::verify_itinerary;

pub struct ExperimentStats {
    pub mean_satisfaction: f64,
    pub std_satisfaction: f64,
    pub mean_ms: f64,
    pub valid_runs: usize,
    pub mean_attractions: f64,
    pub mean_cost: f64,
}

pub fn run_experiments<F>(
    attractions: &[Attraction],
    params: &SolveParams,
    n_runs: usize,
    mut algorithm: F,
) -> ExperimentStats
where
    F: FnMut(&[Attraction], &SolveParams, u64) -> Itinerary,
{
    let mut satisfactions = Vec::new();
    let mut times = Vec::new();
    let mut valid_count = 0;
    let mut total_attractions = 0;
    let mut total_cost = 0.0;

    for run in 0..n_runs {
        let seed = (run as u64) * 13 + 7;
        let itinerary = algorithm(attractions, params, seed);
        
        if verify_itinerary(&itinerary, params).is_ok() {
            satisfactions.push(itinerary.total_satisfaction);
            times.push(itinerary.computation_ms as f64);
            total_attractions += itinerary.total_attractions;
            total_cost += itinerary.total_cost;
            valid_count += 1;
        }
    }

    let mean_sat = if !satisfactions.is_empty() {
        satisfactions.iter().sum::<f64>() / satisfactions.len() as f64
    } else {
        0.0
    };

    let std_sat = if satisfactions.len() > 1 {
        let variance = satisfactions.iter()
            .map(|x| (x - mean_sat).powi(2))
            .sum::<f64>() / (satisfactions.len() - 1) as f64;
        variance.sqrt()
    } else {
        0.0
    };

    let mean_time = if !times.is_empty() {
        times.iter().sum::<f64>() / times.len() as f64
    } else {
        0.0
    };

    ExperimentStats {
        mean_satisfaction: mean_sat,
        std_satisfaction: std_sat,
        mean_ms: mean_time,
        valid_runs: valid_count,
        mean_attractions: if valid_count > 0 { total_attractions as f64 / valid_count as f64 } else { 0.0 },
        mean_cost: if valid_count > 0 { total_cost / valid_count as f64 } else { 0.0 },
    }
}
