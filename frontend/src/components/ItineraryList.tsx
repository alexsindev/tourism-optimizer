import React, { useState } from "react";
import type { Itinerary } from "../types";
import { getDayColor, getCategoryIcon, formatTime } from "../utils/colors";
import ConvergencePlot from "./ConvergencePlot";

interface ItineraryListProps {
  itinerary: Itinerary;
}

const ItineraryList: React.FC<ItineraryListProps> = ({ itinerary }) => {
  const [expandedDays, setExpandedDays] = useState<Set<number>>(new Set([1]));

  const toggleDay = (day: number) => {
    const newExpanded = new Set(expandedDays);
    if (newExpanded.has(day)) {
      newExpanded.delete(day);
    } else {
      newExpanded.add(day);
    }
    setExpandedDays(newExpanded);
  };

  return (
    <div className="space-y-4">
      {/* Summary */}
      <div className="bg-gradient-to-r from-blue-500 to-blue-600 text-white rounded-lg p-4">
        <div className="grid grid-cols-3 gap-2 text-center">
          <div>
            <div className="text-2xl font-bold">
              {itinerary.total_satisfaction.toFixed(2)}
            </div>
            <div className="text-xs opacity-90">Satisfaction</div>
          </div>
          <div>
            <div className="text-2xl font-bold">
              {itinerary.total_attractions}
            </div>
            <div className="text-xs opacity-90">Attractions</div>
          </div>
          <div>
            <div className="text-2xl font-bold">
              ${itinerary.total_cost.toFixed(2)}
            </div>
            <div className="text-xs opacity-90">Total Cost</div>
          </div>
        </div>
      </div>

      {/* Algorithm Badge */}
      <div className="flex items-center justify-between">
        <span className="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-blue-100 text-blue-800">
          {itinerary.algorithm_used === "greedy"
            ? "Greedy Algorithm"
            : "Simulated Annealing"}
        </span>
        <span className="text-sm text-gray-500">
          {itinerary.computation_ms}ms
        </span>
      </div>

      {/* Day Plans */}
      <div className="space-y-3">
        {itinerary.days.map((day) => (
          <div key={day.day} className="border rounded-lg overflow-hidden">
            <button
              onClick={() => toggleDay(day.day)}
              className="w-full px-4 py-3 flex items-center justify-between hover:bg-gray-50 transition-colors"
              style={{ borderLeft: `4px solid ${getDayColor(day.day)}` }}
            >
              <div className="flex items-center space-x-3">
                <div
                  className="w-8 h-8 rounded-full flex items-center justify-center text-white font-bold"
                  style={{ backgroundColor: getDayColor(day.day) }}
                >
                  {day.day}
                </div>
                <div className="text-left">
                  <div className="font-medium">Day {day.day}</div>
                  <div className="text-sm text-gray-500">
                    {day.visits.length} stops · ${day.total_cost.toFixed(2)}
                  </div>
                </div>
              </div>
              <svg
                className={`w-5 h-5 transform transition-transform ${
                  expandedDays.has(day.day) ? "rotate-180" : ""
                }`}
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </button>

            {expandedDays.has(day.day) && (
              <div className="px-4 py-3 bg-gray-50 space-y-2">
                {day.visits.map((visit, idx) => (
                  <div
                    key={visit.attraction_id}
                    className="bg-white rounded p-3 shadow-sm"
                  >
                    <div className="flex items-start space-x-3">
                      <div
                        className="w-6 h-6 rounded-full flex items-center justify-center text-white text-sm font-bold flex-shrink-0"
                        style={{ backgroundColor: getDayColor(day.day) }}
                      >
                        {idx + 1}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="font-medium text-sm">
                          {getCategoryIcon(visit.category)}{" "}
                          {visit.attraction_name}
                        </div>
                        <div className="text-xs text-gray-600 mt-1 space-y-1">
                          <div>
                            🕐 {formatTime(visit.arrival_time)} -{" "}
                            {formatTime(visit.departure_time)}
                          </div>
                          <div className="flex items-center justify-between">
                            <span>💰 ${visit.fee.toFixed(2)}</span>
                            <span>⭐ {visit.preference.toFixed(2)}</span>
                          </div>
                          <div className="text-gray-500">{visit.category}</div>
                        </div>
                      </div>
                    </div>
                    {/* Preference bar */}
                    <div className="mt-2">
                      <div className="h-1.5 bg-gray-200 rounded-full overflow-hidden">
                        <div
                          className="h-full bg-green-500"
                          style={{ width: `${visit.preference * 100}%` }}
                        />
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
      </div>

      {/* Convergence Plot for SA */}
      {itinerary.convergence_data && itinerary.convergence_data.length > 0 && (
        <div className="mt-6">
          <h3 className="text-sm font-medium text-gray-700 mb-2">
            Convergence
          </h3>
          <ConvergencePlot data={itinerary.convergence_data} />
        </div>
      )}
    </div>
  );
};

export default ItineraryList;
