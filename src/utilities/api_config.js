const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8000';

export const API_ENDPOINTS = {
  GET_CSRF_TOKEN: `${API_BASE_URL}/get_csrf_token/`,
  REGISTER: `${API_BASE_URL}/accounts/register_user/`,
  SIGN_IN: `${API_BASE_URL}/accounts/sign_in/`,
  SIGN_OUT: `${API_BASE_URL}/accounts/sign_out/`,
  SIMULATION_SUBMISSION: `${API_BASE_URL}/simulation_manager/simulation_submission/`,
};

export const API_COOKIES = {
    CSRF_TOKEN: 'csrftoken',
    SESSION_ID: 'sessionid',
};

export const ENV_CONFIG = {
  isDevelopment: process.env.NODE_ENV === 'development',
  isProduction: process.env.NODE_ENV === 'production',
  apiVersion: process.env.REACT_APP_API_VERSION || 'v1',
};