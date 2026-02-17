import axios from "axios";
import type {
  DatasetInfo,
  Itinerary,
  SolveParams,
  BenchmarkResult,
  ExperimentResult,
} from "../types";

const API_BASE = "http://localhost:3000/api";

export const api = {
  async getDatasets(): Promise<DatasetInfo[]> {
    const response = await axios.get(`${API_BASE}/datasets`);
    return response.data;
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
