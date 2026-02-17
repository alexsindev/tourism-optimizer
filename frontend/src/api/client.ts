import axios from "axios";
import type {
  DatasetInfo,
  Itinerary,
  SolveParams,
  BenchmarkResult,
  ExperimentResult,
} from "../types";

const API_BASE = "http://localhost:3000/api";

// Add request interceptor for debugging
axios.interceptors.request.use(
  (config) => {
    console.log("API Request:", config.method?.toUpperCase(), config.url);
    return config;
  },
  (error) => {
    console.error("API Request Error:", error);
    return Promise.reject(error);
  },
);

// Add response interceptor for debugging
axios.interceptors.response.use(
  (response) => {
    console.log("API Response:", response.status, response.config.url);
    return response;
  },
  (error) => {
    console.error(
      "API Response Error:",
      error.response?.status,
      error.response?.data || error.message,
    );
    return Promise.reject(error);
  },
);

export const api = {
  async getDatasets(): Promise<DatasetInfo[]> {
    try {
      const response = await axios.get(`${API_BASE}/datasets`);
      return response.data;
    } catch (error) {
      console.error("Failed to fetch datasets:", error);
      throw error;
    }
  },

  async solve(
    algorithm: string,
    datasetName: string,
    params: SolveParams,
  ): Promise<Itinerary> {
    const response = await axios.post(`${API_BASE}/solve`, {
      algorithm,
      dataset: {
        type: "builtin",
        name: datasetName,
      },
      params,
    });
    return response.data.itinerary;
  },

  async benchmark(
    datasetName: string,
    params: SolveParams,
  ): Promise<BenchmarkResult> {
    const response = await axios.post(`${API_BASE}/benchmark`, {
      dataset: {
        type: "builtin",
        name: datasetName,
      },
      params,
    });
    return response.data;
  },

  async experiment(
    datasetName: string,
    nRuns: number,
    params: SolveParams,
  ): Promise<ExperimentResult> {
    const response = await axios.post(`${API_BASE}/experiment`, {
      dataset: datasetName,
      n_runs: nRuns,
      params,
    });
    return response.data;
  },
};
