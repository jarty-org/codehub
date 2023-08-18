import axios, { AxiosResponse, AxiosRequestConfig } from "axios";
import Message from "@/components/message";

const baseURL: string = import.meta.env.VITE_BASE_API;
const axiosInstance = axios.create({
  baseURL,
  timeout: 10000,
  headers: {
    "Content-Type": "application/json",
  },
});

axiosInstance.interceptors.request.use(
  (config) => {
    return config;
  },
  async (error) => await Promise.reject(error)
);

axiosInstance.interceptors.response.use(
  (response: AxiosResponse) => {
    return response.data;
  },
  (error) => {
    const data = error.response.data;
    Message({ type: "error", message: data || error.message });
    return Promise.reject(error);
  }
);

const request = async <T>(config: AxiosRequestConfig): Promise<T> => {
  const res = await axiosInstance(config);
  return res as T;
};

export default request;
