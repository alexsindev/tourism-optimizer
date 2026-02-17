export interface Location {
  lat: number;
  lng: number;
}

export interface Visit {
  attraction_id: number;
  attraction_name: string;
  arrival_time: number;
  departure_time: number;
  fee: number;
  preference: number;
  category: string;
}

export interface DayPlan {
  day: number;
  visits: Visit[];
  total_travel_time: number;
  total_cost: number;
  total_satisfaction: number;
}

export interface ConvergencePoint {
  iteration: number;
  satisfaction: number;
  temperature: number;
}

export interface Itinerary {
  days: DayPlan[];
  total_satisfaction: number;
  total_cost: number;
  total_attractions: number;
  algorithm_used: string;
  computation_ms: number;
  convergence_data?: ConvergencePoint[];
}

export interface SolveParams {
  num_days: number;
  daily_time_budget: number;
  total_budget: number;
  start_time: number;
  hotel_lat: number;
  hotel_lng: number;
  end_lat?: number;
  end_lng?: number;
}

export interface DatasetInfo {
  name: string;
  size: number;
}

export interface BenchmarkResult {
  greedy: Itinerary;
  simulated_annealing: Itinerary;
}

export interface AlgorithmStats {
  mean_satisfaction: number;
  std_satisfaction: number;
  mean_ms: number;
  valid_runs: number;
  mean_attractions: number;
  mean_cost: number;
}

export interface ExperimentResult {
  dataset_name: string;
  dataset_size: number;
  greedy: AlgorithmStats;
  sa: AlgorithmStats;
  sa_improvement_pct: number;
}
