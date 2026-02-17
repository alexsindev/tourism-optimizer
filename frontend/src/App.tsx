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
  const [error, setError] = useState<string | null>(null);
  const [pinDropMode, setPinDropMode] = useState<"start" | "end" | null>(null);

  useEffect(() => {
    loadDatasets();
  }, []);

  const loadDatasets = async () => {
    try {
      setError(null);
      const data = await api.getDatasets();
      console.log("Loaded datasets:", data);
      setDatasets(data);
      if (data.length > 0 && !selectedDataset) {
        setSelectedDataset(data[0].name);
      }
    } catch (error) {
      console.error("Failed to load datasets:", error);
      setError(
        "Failed to connect to server. Make sure the Rust server is running on http://localhost:3000",
      );
    }
  };

  const handleSolve = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await api.solve(
        selectedAlgorithm,
        selectedDataset,
        params,
      );
      setItinerary(result);
      setActiveTab("itinerary");
    } catch (error: any) {
      console.error("Failed to solve:", error);
      setError(
        error.response?.data?.message ||
          "Failed to solve. Check console for details.",
      );
    } finally {
      setLoading(false);
    }
  };

  const handleBenchmark = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await api.benchmark(selectedDataset, params);
      setBenchmarkResult(result);
      setActiveTab("benchmark");
    } catch (error: any) {
      console.error("Failed to benchmark:", error);
      setError(
        error.response?.data?.message ||
          "Failed to benchmark. Check console for details.",
      );
    } finally {
      setLoading(false);
    }
  };

  const handleStartPointChange = (lat: number, lng: number) => {
    setParams({ ...params, hotel_lat: lat, hotel_lng: lng });
    setPinDropMode(null);
  };

  const handleEndPointChange = (lat: number, lng: number) => {
    setParams({ ...params, end_lat: lat, end_lng: lng });
    setPinDropMode(null);
  };

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Left Panel - Controls */}
      <div className="w-80 bg-white shadow-lg overflow-y-auto">
        <div className="p-6">
          <h1 className="text-2xl font-bold text-gray-800 mb-6">
            Tourism Route Optimizer
          </h1>

          {/* Error Message */}
          {error && (
            <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
              <p className="text-sm">{error}</p>
              <button
                onClick={loadDatasets}
                className="mt-2 text-xs underline hover:no-underline"
              >
                Retry
              </button>
            </div>
          )}

          {datasets.length === 0 && !error && (
            <div className="mb-4 p-3 bg-yellow-100 border border-yellow-400 text-yellow-700 rounded">
              <p className="text-sm">Loading datasets...</p>
            </div>
          )}

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
            pinDropMode={pinDropMode}
            onPinDropModeChange={setPinDropMode}
          />
        </div>
      </div>

      {/* Center Panel - Map */}
      <div className="flex-1 relative">
        {pinDropMode && (
          <div className="absolute top-4 left-1/2 transform -translate-x-1/2 z-1000 bg-blue-500 text-white px-4 py-2 rounded-lg shadow-lg">
            {pinDropMode === "start"
              ? "📍 Click on map to set start point"
              : "🏁 Click on map to set end point"}
          </div>
        )}
        <MapView
          itinerary={itinerary}
          params={params}
          onStartPointChange={handleStartPointChange}
          onEndPointChange={handleEndPointChange}
          pinDropMode={pinDropMode}
        />
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
