import axios from "axios";

let api = axios.create({
    baseURL: "http://localhost:8080",
    timeout: 1000,
    withCredentials: true,
});

export { api };