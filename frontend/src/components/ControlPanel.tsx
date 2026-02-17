import React from "react";
import type { DatasetInfo, SolveParams } from "../types";

interface ControlPanelProps {
  datasets: DatasetInfo[];
  selectedDataset: string;
  selectedAlgorithm: string;
  params: SolveParams;
  loading: boolean;
  onDatasetChange: (dataset: string) => void;
  onAlgorithmChange: (algorithm: string) => void;
  onParamsChange: (params: SolveParams) => void;
  onSolve: () => void;
  onBenchmark: () => void;
  pinDropMode?: "start" | "end" | null;
  onPinDropModeChange?: (mode: "start" | "end" | null) => void;
}

const ControlPanel: React.FC<ControlPanelProps> = ({
  datasets,
  selectedDataset,
  selectedAlgorithm,
  params,
  loading,
  onDatasetChange,
  onAlgorithmChange,
  onParamsChange,
  onSolve,
  onBenchmark,
  pinDropMode,
  onPinDropModeChange,
}) => {
  return (
    <div className="space-y-6">
      {/* Algorithm Selection */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Algorithm
        </label>
        <select
          value={selectedAlgorithm}
          onChange={(e) => onAlgorithmChange(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          disabled={loading}
        >
          <option value="greedy">Greedy (Baseline)</option>
          <option value="simulated_annealing">Simulated Annealing</option>
        </select>
      </div>

      {/* Dataset Selection */}
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Dataset
        </label>
        <select
          value={selectedDataset}
          onChange={(e) => onDatasetChange(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          disabled={loading}
        >
          {datasets.map((ds) => (
            <option key={ds.name} value={ds.name}>
              {ds.name} ({ds.size} attractions)
            </option>
          ))}
        </select>
      </div>

      {/* Parameters */}
      <div className="space-y-4">
        <h3 className="text-sm font-medium text-gray-700">Parameters</h3>

        {/* Location Selection */}
        <div className="space-y-2">
          <label className="block text-xs text-gray-600">Start Point</label>
          <div className="flex items-center space-x-2">
            <input
              type="text"
              value={`${params.hotel_lat.toFixed(4)}, ${params.hotel_lng.toFixed(4)}`}
              readOnly
              className="flex-1 px-2 py-1 text-xs border border-gray-300 rounded bg-gray-50"
            />
            <button
              onClick={() =>
                onPinDropModeChange?.(pinDropMode === "start" ? null : "start")
              }
              className={`px-3 py-1 text-xs rounded ${
                pinDropMode === "start"
                  ? "bg-blue-500 text-white"
                  : "bg-gray-200 hover:bg-gray-300"
              }`}
              disabled={loading}
            >
              📍 {pinDropMode === "start" ? "Cancel" : "Set"}
            </button>
          </div>
        </div>

        <div className="space-y-2">
          <label className="block text-xs text-gray-600">
            End Point (Optional)
          </label>
          <div className="flex items-center space-x-2">
            <input
              type="text"
              value={
                params.end_lat && params.end_lng
                  ? `${params.end_lat.toFixed(4)}, ${params.end_lng.toFixed(4)}`
                  : "Same as start"
              }
              readOnly
              className="flex-1 px-2 py-1 text-xs border border-gray-300 rounded bg-gray-50"
            />
            <button
              onClick={() =>
                onPinDropModeChange?.(pinDropMode === "end" ? null : "end")
              }
              className={`px-3 py-1 text-xs rounded ${
                pinDropMode === "end"
                  ? "bg-red-500 text-white"
                  : "bg-gray-200 hover:bg-gray-300"
              }`}
              disabled={loading}
            >
              🏁 {pinDropMode === "end" ? "Cancel" : "Set"}
            </button>
          </div>
          {params.end_lat && params.end_lng && (
            <button
              onClick={() =>
                onParamsChange({
                  ...params,
                  end_lat: undefined,
                  end_lng: undefined,
                })
              }
              className="text-xs text-red-600 hover:underline"
            >
              Clear end point
            </button>
          )}
        </div>

        <div>
          <label className="block text-xs text-gray-600 mb-1">
            Number of Days
          </label>
          <input
            type="number"
            min="1"
            max="7"
            value={params.num_days}
            onChange={(e) =>
              onParamsChange({ ...params, num_days: parseInt(e.target.value) })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            disabled={loading}
          />
        </div>

        <div>
          <label className="block text-xs text-gray-600 mb-1">
            Daily Time Budget (minutes)
          </label>
          <input
            type="number"
            min="120"
            max="720"
            step="60"
            value={params.daily_time_budget}
            onChange={(e) =>
              onParamsChange({
                ...params,
                daily_time_budget: parseInt(e.target.value),
              })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            disabled={loading}
          />
          <p className="text-xs text-gray-500 mt-1">
            {Math.floor(params.daily_time_budget / 60)}h{" "}
            {params.daily_time_budget % 60}m per day
          </p>
        </div>

        <div>
          <label className="block text-xs text-gray-600 mb-1">
            Total Budget ($)
          </label>
          <input
            type="number"
            min="10"
            max="1000"
            step="10"
            value={params.total_budget}
            onChange={(e) =>
              onParamsChange({
                ...params,
                total_budget: parseFloat(e.target.value),
              })
            }
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            disabled={loading}
          />
        </div>

        <div>
          <label className="block text-xs text-gray-600 mb-1">Start Time</label>
          <input
            type="time"
            value={`${Math.floor(params.start_time / 60)
              .toString()
              .padStart(2, "0")}:${(params.start_time % 60)
              .toString()
              .padStart(2, "0")}`}
            onChange={(e) => {
              const [hours, minutes] = e.target.value.split(":").map(Number);
              onParamsChange({ ...params, start_time: hours * 60 + minutes });
            }}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            disabled={loading}
          />
        </div>
      </div>

      {/* Action Buttons */}
      <div className="space-y-2">
        <button
          onClick={onSolve}
          disabled={loading}
          className="w-full bg-blue-500 hover:bg-blue-600 text-white font-medium py-2 px-4 rounded-md disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          {loading ? "Solving..." : "Solve"}
        </button>

        <button
          onClick={onBenchmark}
          disabled={loading}
          className="w-full bg-green-500 hover:bg-green-600 text-white font-medium py-2 px-4 rounded-md disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          {loading ? "Running..." : "Run Benchmark"}
        </button>
      </div>
    </div>
  );
};

export default ControlPanel;
