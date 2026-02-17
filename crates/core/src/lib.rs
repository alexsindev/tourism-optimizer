pub mod models;
pub mod algorithms;
pub mod data_structures;
pub mod utils;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use models::attraction::{Attraction, Category, Location};
pub use models::itinerary::{Itinerary, DayPlan, Visit, ConvergencePoint};
pub use models::constraints::SolveParams;
pub use models::graph::{Graph, Edge};
pub use algorithms::{greedy, simulated_annealing};