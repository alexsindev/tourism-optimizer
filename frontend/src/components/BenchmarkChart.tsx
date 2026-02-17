import React from "react";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Cell,
} from "recharts";
import type { BenchmarkResult } from "../types";

interface BenchmarkChartProps {
  result: BenchmarkResult;
}

const BenchmarkChart: React.FC<BenchmarkChartProps> = ({ result }) => {
  const satisfactionData = [
    {
      name: "Greedy",
      value: result.greedy.total_satisfaction,
    },
    {
      name: "SA",
      value: result.simulated_annealing.total_satisfaction,
    },
  ];

  const timeData = [
    {
      name: "Greedy",
      value: result.greedy.computation_ms,
    },
    {
      name: "SA",
      value: result.simulated_annealing.computation_ms,
    },
  ];

  const colors = ["#3B82F6", "#10B981"];

  return (
    <div className="space-y-6">
      <div>
        <h3 className="text-lg font-medium text-gray-800 mb-4">
          Satisfaction Comparison
        </h3>
        <ResponsiveContainer width="100%" height={200}>
          <BarChart data={satisfactionData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="name" />
            <YAxis />
            <Tooltip />
            <Bar dataKey="value" name="Satisfaction">
              {satisfactionData.map((_, index) => (
                <Cell key={`cell-${index}`} fill={colors[index]} />
              ))}
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>

      <div>
        <h3 className="text-lg font-medium text-gray-800 mb-4">
          Computation Time (ms)
        </h3>
        <ResponsiveContainer width="100%" height={200}>
          <BarChart data={timeData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="name" />
            <YAxis />
            <Tooltip />
            <Bar dataKey="value" name="Time (ms)">
              {timeData.map((_, index) => (
                <Cell key={`cell-${index}`} fill={colors[index]} />
              ))}
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>

      <div className="bg-gray-50 rounded-lg p-4">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b">
              <th className="text-left py-2">Algorithm</th>
              <th className="text-right py-2">Satisfaction</th>
              <th className="text-right py-2">Attractions</th>
              <th className="text-right py-2">Cost</th>
              <th className="text-right py-2">Time (ms)</th>
            </tr>
          </thead>
          <tbody>
            <tr className="border-b">
              <td className="py-2 font-medium">Greedy</td>
              <td className="text-right">
                {result.greedy.total_satisfaction.toFixed(2)}
              </td>
              <td className="text-right">{result.greedy.total_attractions}</td>
              <td className="text-right">
                ${result.greedy.total_cost.toFixed(2)}
              </td>
              <td className="text-right">{result.greedy.computation_ms}</td>
            </tr>
            <tr>
              <td className="py-2 font-medium">SA</td>
              <td className="text-right">
                {result.simulated_annealing.total_satisfaction.toFixed(2)}
              </td>
              <td className="text-right">
                {result.simulated_annealing.total_attractions}
              </td>
              <td className="text-right">
                ${result.simulated_annealing.total_cost.toFixed(2)}
              </td>
              <td className="text-right">
                {result.simulated_annealing.computation_ms}
              </td>
            </tr>
          </tbody>
        </table>

        <div className="mt-4 text-center">
          {result.simulated_annealing.total_satisfaction >
          result.greedy.total_satisfaction ? (
            <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800">
              🏆 SA wins with{" "}
              {(
                (result.simulated_annealing.total_satisfaction /
                  result.greedy.total_satisfaction -
                  1) *
                100
              ).toFixed(1)}
              % improvement
            </span>
          ) : (
            <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
              🏆 Greedy wins
            </span>
          )}
        </div>
      </div>
    </div>
  );
};

export default BenchmarkChart;
