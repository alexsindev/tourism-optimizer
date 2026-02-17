# Correctness Evaluation Guide

## Overview

This project includes comprehensive correctness validation for both the Greedy
and Simulated Annealing algorithms. The validation ensures that all hard
constraints are satisfied and that the algorithms produce valid, feasible
itineraries.

## Validation Methods

### 1. Unit Tests

Run the comprehensive unit test suite:

```bash
cargo test
```

This runs 30+ tests covering:

- No duplicate visits across days
- Time window constraints respected
- Budget constraints respected
- Daily time budget constraints respected
- Chronological ordering of visits
- Correct satisfaction/cost calculations
- Edge cases (tight budget, zero budget, single day, many days)
- Determinism and reproducibility

### 2. Validation CLI Tool

Run the dedicated correctness validation tool:

```bash
cargo run -p validate
```

This tool:

- Tests both algorithms on all three datasets (small, medium, large)
- Tests multiple configurations (default, tight budget, short days, many days)
- Performs 8 comprehensive checks per test
- Provides colored output showing passes/failures
- Exits with code 0 if all tests pass, 1 if any fail

### 3. Manual Verification via Frontend

Use the web UI to visually inspect itineraries:

1. Start the server: `cargo run -p server`
2. Start the frontend: `cd frontend && npm run dev`
3. Solve with different algorithms and parameters
4. Verify results in the UI:
   - Check that no attractions are visited twice
   - Verify arrival/departure times are within opening hours
   - Confirm total cost is under budget
   - Ensure visits don't overlap within each day

### 4. Experiment Runner Validation

Run statistical experiments that include validation:

```bash
# Via frontend: Use "Experiments" tab
# Via API: POST http://localhost:3000/api/experiment
```

The experiment runner:

- Runs N independent trials (default 10)
- Validates each itinerary with the verifier
- Reports `valid_runs` count (target: 10/10)
- Calculates mean ± std dev statistics

## Constraint Verification

The `verify_itinerary` function checks:

1. **No Duplicates**: Each attraction visited at most once across all days
2. **Time Windows**: arrival_time ≥ open_time AND departure_time ≤ close_time
3. **Chronological Order**: Within each day, visits don't overlap
4. **Daily Time Budget**: Each day's duration ≤ daily_time_budget
5. **Total Budget**: total_cost ≤ total_budget

## Expected Results

### Greedy Algorithm

- **Deterministic**: Same input → same output
- **Fast**: <5ms for small, <50ms for medium, <500ms for large
- **Valid**: Should pass all constraints 100% of the time
- **Locally Optimal**: May not be globally optimal

### Simulated Annealing

- **Stochastic**: Different seeds → different (but valid) outputs
- **Slower**: ~100-500ms depending on dataset size
- **Valid**: Should pass all constraints 100% of the time
- **Better Quality**: Higher satisfaction than Greedy on average

## Running All Validation

Complete validation workflow:

```bash
# 1. Generate datasets
cargo run -p generate

# 2. Run unit tests
cargo test --lib -p core

# 3. Run validation tool
cargo run -p validate

# 4. Start server and test via API
cargo run -p server &
curl http://localhost:3000/api/datasets
curl -X POST http://localhost:3000/api/solve \
  -H "Content-Type: application/json" \
  -d '{
    "algorithm": "greedy",
    "dataset": {"type": "builtin", "name": "small"},
    "params": {
      "num_days": 2,
      "daily_time_budget": 600,
      "total_budget": 100,
      "start_time": 540,
      "hotel_lat": 13.7563,
      "hotel_lng": 100.5018
    }
  }'
```

## Debugging Failed Tests

If a test fails:

1. Check the error message for which constraint was violated
2. Look at the test name to identify the scenario
3. Run that specific test with output:
   ```bash
   cargo test test_name -- --nocapture
   ```
4. Add debug prints to the algorithm code
5. Use the validation tool for more detailed output

## Continuous Integration

For CI/CD pipelines:

```bash
# Run all validation and fail if any test fails
cargo test && cargo run -p validate
```

This ensures that:

- All unit tests pass
- All algorithms produce valid itineraries on all datasets
- No regressions are introduced

## Success Criteria

A correct implementation should achieve:

- ✅ All unit tests pass (cargo test)
- ✅ Validation tool reports 100% pass rate
- ✅ Experiments show 10/10 valid runs for all configurations
- ✅ No constraint violations in manual testing
- ✅ Itineraries are feasible and satisfy all hard constraints

## NP-Hardness Note

While these algorithms are heuristics (not guaranteed optimal), they MUST
produce:

- **Feasible** solutions (all constraints satisfied)
- **Valid** solutions (verifier passes)
- **Consistent** solutions (deterministic for same input/seed)

Optimality is not guaranteed, but correctness is mandatory.
