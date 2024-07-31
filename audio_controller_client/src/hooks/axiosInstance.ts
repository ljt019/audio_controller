import axios from "axios";

export const baseURL = "http://192.168.1.59:3030";

export const axiosInstance = axios.create({
  baseURL: baseURL,
});
