import axios from "axios";

const API_URL = "http://localhost:3000/api";

export const login = async (username, password) => {
  try {
    const response = await axios.post(`${API_URL}/login`, {
      username,
      password,
    });
    return { success: true, message: response.data };
  } catch (error) {
    return {
      success: false,
      message: error.response?.data || "An unexpected error occurred.",
    };
  }
};

export const register = async (username, password) => {
  try {
    const response = await axios.post(`${API_URL}/register`, {
      username,
      password,
    });
    return { success: true, message: response.data };
  } catch (error) {
    return {
      success: false,
      message: error.response?.data || "An unexpected error occurred.",
    };
  }
};
