import { deleteCookie, getCookie } from "./cookies";
import { API_ENDPOINTS } from "./api_config";
import yaml from 'js-yaml';

/**
 * Fetches the CSRF token from the server.
 * @returns {Promise} A promise that resolves to the CSRF token.
 */
export const fetchCsrfToken = async () => {
  deleteCookie('csrftoken');
  try {
    const data = await get(API_ENDPOINTS.GET_CSRF_TOKEN);
    return data;
  } catch (error) {
    console.error('Error fetching CSRF token:', error);
    throw error;
  }
};

/**
 * Performs a GET request to the specified API URL.
 * @param {string} api_url - The URL of the API endpoint.
 * @returns {Promise<any>} - A promise that resolves to the response data.
 */
export const get = async (api_url) => {
  const response = await fetch(api_url, {
    method: 'GET',
    headers: {
      'X-CSRFToken': getCookie('csrftoken'),
    },
    credentials: 'include'
  });

  const data = await handleResponse(response);
  return data;

};

/**
 * Sends a POST request to the specified API URL with the provided data.
 * @param {string} api_url - The URL of the API endpoint.
 * @param {string} outputFormat - The desired format of the output data ('yaml' or 'json').
 * @param {Object} data - The data to be sent in the request body.
 * @returns {Promise<Object>} - A promise that resolves to the response data from the API.
 */
export const post = async (api_url, outputFormat, data) => {
  let outputBody = {};
  let outputHeader = {};
  
  if (outputFormat === 'yaml') {
    outputHeader['Content-Type'] = 'application/x-yaml';
    outputBody = yaml.dump(data);
  } else if (outputFormat === 'json') {
    outputHeader['Content-Type'] = 'application/json';
    outputBody = JSON.stringify(data);
  }
  outputHeader['X-CSRFToken'] = getCookie('csrftoken');

  const response = await fetch(api_url, {
    method: 'POST',
    headers: outputHeader,
    body: outputBody,
    credentials: 'include'
  });

  const responseData = await handleResponse(response);
  console.log(responseData);
  return responseData;
};

/**
 * Handles the response from an API request.
 * @param {Response} response - The response object from the API request.
 * @returns {Promise<Object>} - The parsed response data.
 * @throws {Error} - If the response is not successful or an error occurs.
 */
export const handleResponse = async (response) => {
  if (!response.ok) {
    console.log('Status:', response.status);
    const text = await response.text();
    console.log('Response:', text);
    throw new Error('Something went wrong');
  }
  const responseData = await response.json();
  return responseData;
}
