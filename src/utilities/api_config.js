const API_BASE_URL = 'http://localhost:8000';

export const API_ENDPOINTS = {
  GET_CSRF_TOKEN: `${API_BASE_URL}/get_csrf_token/`,
  REGISTER: `${API_BASE_URL}/register/`,
  SIGN_IN: `${API_BASE_URL}/sign_in/`,
  SIGN_OUT: `${API_BASE_URL}/sign_out/`,
};

export const API_COOKIES = {
    CSRF_TOKEN: 'csrftoken',
    SESSION_ID: 'sessionid',
};