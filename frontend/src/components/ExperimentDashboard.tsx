import React, { useState } from "react";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  ErrorBar,
  Cell,
} from "recharts";
import { api } from "../api/client";
import type { DatasetInfo, SolveParams, ExperimentResult } from "../types";

interface ExperimentDashboardProps {
  params: SolveParams;
  datasets: DatasetInfo[];
}

const ExperimentDashboard: React.FC<ExperimentDashboardProps> = ({
  params,
  datasets,
}) => {
  const [nRuns, setNRuns] = useState(10);
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<ExperimentResult[]>([]);

  const runExperiments = async () => {
    setLoading(true);
    setResults([]);

    try {
      for (const dataset of datasets) {
        const result = await api.experiment(dataset.name, nRuns, params);
        setResults((prev) => [...prev, result]);
      }
    } catch (error) {
      console.error("Failed to run experiments:", error);
    } finally {
      setLoading(false);
    }
  };

  const chartData = results.map((r) => ({
    name: r.dataset_name,
    greedy: r.greedy.mean_satisfaction,
    greedyError: r.greedy.std_satisfaction,
    sa: r.sa.mean_satisfaction,
    saError: r.sa.std_satisfaction,
  }));

  return (
    <div className="space-y-4">
      {/* Controls */}
      <div className="bg-gray-50 rounded-lg p-4">
        <div className="mb-3">
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Number of Runs
          </label>
          <select
            value={nRuns}
            onChange={(e) => setNRuns(parseInt(e.target.value))}
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            disabled={loading}
          >
            <option value={5}>5 runs</option>
            <option value={10}>10 runs</option>
            <option value={20}>20 runs</option>
            <option value={30}>30 runs</option>
          </select>
        </div>

        <button
          onClick={runExperiments}
          disabled={loading}
          className="w-full bg-purple-500 hover:bg-purple-600 text-white font-medium py-2 px-4 rounded-md disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          {loading
            ? `Running... (${results.length}/${datasets.length})`
            : "Run All Experiments"}
        </button>
      </div>

      {/* Chart */}
      {results.length > 0 && (
        <>
          <div className="bg-white rounded-lg border p-4">
            <h3 className="text-sm font-medium text-gray-700 mb-3">
              Mean Satisfaction ± Std Dev
            </h3>
            <ResponsiveContainer width="100%" height={250}>
              <BarChart data={chartData}>
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="name" />
                <YAxis />
                <Tooltip />
                <Legend />
                <Bar dataKey="greedy" fill="#3B82F6" name="Greedy">
                  <ErrorBar dataKey="greedyError" width={4} strokeWidth={2} />
                </Bar>
                <Bar dataKey="sa" fill="#10B981" name="SA">
                  <ErrorBar dataKey="saError" width={4} strokeWidth={2} />
                </Bar>
              </BarChart>
            </ResponsiveContainer>
          </div>

          {/* Results Table */}
          <div className="bg-white rounded-lg border overflow-hidden">
            <div className="overflow-x-auto">
              <table className="w-full text-sm">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-4 py-2 text-left font-medium text-gray-700">
                      Dataset
                    </th>
                    <th className="px-4 py-2 text-right font-medium text-gray-700">
                      Size
                    </th>
                    <th
                      className="px-4 py-2 text-center font-medium text-gray-700"
                      colSpan={2}
                    >
                      Greedy
                    </th>
                    <th
                      className="px-4 py-2 text-center font-medium text-gray-700"
                      colSpan={2}
                    >
                      SA
                    </th>
                    <th className="px-4 py-2 text-center font-medium text-gray-700">
                      Improvement
                    </th>
                  </tr>
                  <tr className="border-t">
                    <th></th>
                    <th></th>
                    <th className="px-2 py-1 text-xs text-gray-600">
                      Sat (μ±σ)
                    </th>
                    <th className="px-2 py-1 text-xs text-gray-600">
                      Time (ms)
                    </th>
                    <th className="px-2 py-1 text-xs text-gray-600">
                      Sat (μ±σ)
                    </th>
                    <th className="px-2 py-1 text-xs text-gray-600">
                      Time (ms)
                    </th>
                    <th className="px-2 py-1 text-xs text-gray-600">(%)</th>
                  </tr>
                </thead>
                <tbody className="divide-y">
                  {results.map((result) => (
                    <tr key={result.dataset_name} className="hover:bg-gray-50">
                      <td className="px-4 py-2 font-medium">
                        {result.dataset_name}
                      </td>
                      <td className="px-4 py-2 text-right">
                        {result.dataset_size}
                      </td>
                      <td className="px-2 py-2 text-right">
                        {result.greedy.mean_satisfaction.toFixed(2)} ±{" "}
                        {result.greedy.std_satisfaction.toFixed(2)}
                      </td>
                      <td className="px-2 py-2 text-right">
                        {result.greedy.mean_ms.toFixed(1)}
                      </td>
                      <td className="px-2 py-2 text-right">
                        {result.sa.mean_satisfaction.toFixed(2)} ±{" "}
                        {result.sa.std_satisfaction.toFixed(2)}
                      </td>
                      <td className="px-2 py-2 text-right">
                        {result.sa.mean_ms.toFixed(1)}
                      </td>
                      <td className="px-2 py-2 text-right">
                        <span
                          className={
                            result.sa_improvement_pct > 0
                              ? "text-green-600 font-medium"
                              : "text-gray-600"
                          }
                        >
                          {result.sa_improvement_pct > 0 ? "+" : ""}
                          {result.sa_improvement_pct.toFixed(1)}%
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>

          {/* Valid Runs Info */}
          <div className="bg-blue-50 border border-blue-200 rounded-lg p-3">
            <div className="text-sm text-blue-800">
              <strong>Valid Runs:</strong> All results show{" "}
              {results[0]?.greedy.valid_runs}/{nRuns} valid runs for both
              algorithms
            </div>
          </div>
        </>
      )}

      {results.length === 0 && !loading && (
        <div className="text-center text-gray-500 py-8">
          Click "Run All Experiments" to start statistical analysis across all
          datasets
        </div>
      )}
    </div>
  );
};

export default ExperimentDashboard;
