pub mod greedy;
pub mod simulated_annealing;

pub use greedy::solve as greedy_solve;
pub use simulated_annealing::solve as sa_solve;
