import React, { useState, useEffect } from "react";
import { api } from "./api/client";
import type {
  DatasetInfo,
  Itinerary,
  SolveParams,
  BenchmarkResult,
} from "./types";
import MapView from "./components/MapView";
import ControlPanel from "./components/ControlPanel";
import ItineraryList from "./components/ItineraryList";
import BenchmarkChart from "./components/BenchmarkChart";
import ExperimentDashboard from "./components/ExperimentDashboard";

type TabType = "itinerary" | "benchmark" | "experiment";

function App() {
  const [datasets, setDatasets] = useState<DatasetInfo[]>([]);
  const [selectedDataset, setSelectedDataset] = useState<string>("small");
  const [selectedAlgorithm, setSelectedAlgorithm] = useState<string>("greedy");
  const [params, setParams] = useState<SolveParams>({
    num_days: 2,
    daily_time_budget: 600,
    total_budget: 100,
    start_time: 540,
    hotel_lat: 13.7563,
    hotel_lng: 100.5018,
  });

  const [itinerary, setItinerary] = useState<Itinerary | null>(null);
  const [benchmarkResult, setBenchmarkResult] =
    useState<BenchmarkResult | null>(null);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<TabType>("itinerary");

  useEffect(() => {
    loadDatasets();
  }, []);

  const loadDatasets = async () => {
    try {
      const data = await api.getDatasets();
      setDatasets(data);
    } catch (error) {
      console.error("Failed to load datasets:", error);
    }
  };

  const handleSolve = async () => {
    setLoading(true);
    try {
      const result = await api.solve(
        selectedAlgorithm,
        selectedDataset,
        params,
      );
      setItinerary(result);
      setActiveTab("itinerary");
    } catch (error) {
      console.error("Failed to solve:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleBenchmark = async () => {
    setLoading(true);
    try {
      const result = await api.benchmark(selectedDataset, params);
      setBenchmarkResult(result);
      setActiveTab("benchmark");
    } catch (error) {
      console.error("Failed to benchmark:", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Left Panel - Controls */}
      <div className="w-80 bg-white shadow-lg overflow-y-auto">
        <div className="p-6">
          <h1 className="text-2xl font-bold text-gray-800 mb-6">
            Tourism Route Optimizer
          </h1>
          <ControlPanel
            datasets={datasets}
            selectedDataset={selectedDataset}
            selectedAlgorithm={selectedAlgorithm}
            params={params}
            loading={loading}
            onDatasetChange={setSelectedDataset}
            onAlgorithmChange={setSelectedAlgorithm}
            onParamsChange={setParams}
            onSolve={handleSolve}
            onBenchmark={handleBenchmark}
          />
        </div>
      </div>

      {/* Center Panel - Map */}
      <div className="flex-1 relative">
        <MapView itinerary={itinerary} params={params} />
      </div>

      {/* Right Panel - Results */}
      <div className="w-96 bg-white shadow-lg overflow-y-auto">
        <div className="border-b">
          <div className="flex">
            <button
              className={`flex-1 py-3 px-4 font-medium ${
                activeTab === "itinerary"
                  ? "bg-blue-500 text-white"
                  : "bg-gray-100 text-gray-600 hover:bg-gray-200"
              }`}
              onClick={() => setActiveTab("itinerary")}
            >
              Itinerary
            </button>
            <button
              className={`flex-1 py-3 px-4 font-medium ${
                activeTab === "benchmark"
                  ? "bg-blue-500 text-white"
                  : "bg-gray-100 text-gray-600 hover:bg-gray-200"
              }`}
              onClick={() => setActiveTab("benchmark")}
            >
              Benchmark
            </button>
            <button
              className={`flex-1 py-3 px-4 font-medium ${
                activeTab === "experiment"
                  ? "bg-blue-500 text-white"
                  : "bg-gray-100 text-gray-600 hover:bg-gray-200"
              }`}
              onClick={() => setActiveTab("experiment")}
            >
              Experiments
            </button>
          </div>
        </div>

        <div className="p-4">
          {activeTab === "itinerary" && itinerary && (
            <ItineraryList itinerary={itinerary} />
          )}
          {activeTab === "benchmark" && benchmarkResult && (
            <BenchmarkChart result={benchmarkResult} />
          )}
          {activeTab === "experiment" && (
            <ExperimentDashboard params={params} datasets={datasets} />
          )}
          {activeTab === "itinerary" && !itinerary && (
            <div className="text-center text-gray-500 mt-8">
              Select algorithm and click "Solve" to see results
            </div>
          )}
          {activeTab === "benchmark" && !benchmarkResult && (
            <div className="text-center text-gray-500 mt-8">
              Click "Run Benchmark" to compare algorithms
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
