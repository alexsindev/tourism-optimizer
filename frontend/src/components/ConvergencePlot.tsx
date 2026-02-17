import React from "react";
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";
import type { ConvergencePoint } from "../types";

interface ConvergencePlotProps {
  data: ConvergencePoint[];
}

const ConvergencePlot: React.FC<ConvergencePlotProps> = ({ data }) => {
  return (
    <div className="bg-white rounded-lg border p-4">
      <ResponsiveContainer width="100%" height={250}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis
            dataKey="iteration"
            label={{ value: "Iteration", position: "insideBottom", offset: -5 }}
          />
          <YAxis
            yAxisId="left"
            label={{
              value: "Satisfaction",
              angle: -90,
              position: "insideLeft",
            }}
          />
          <YAxis
            yAxisId="right"
            orientation="right"
            label={{ value: "Temperature", angle: 90, position: "insideRight" }}
          />
          <Tooltip />
          <Legend />
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="satisfaction"
            stroke="#3B82F6"
            strokeWidth={2}
            dot={false}
            name="Satisfaction"
          />
          <Line
            yAxisId="right"
            type="monotone"
            dataKey="temperature"
            stroke="#EF4444"
            strokeWidth={2}
            dot={false}
            name="Temperature"
          />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
};

export default ConvergencePlot;
