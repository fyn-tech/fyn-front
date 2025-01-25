/**
 * Checks if a cookie with the specified name exists.
 * @param {string} name - The name of the cookie to check.
 * @returns {boolean} - True if the cookie exists, false otherwise.
 */
export function cookieExists(name){
  return document.cookie.split(';').some(c => {
      return c.trim().startsWith(name + '=');
  });
}

/**
 * Deletes a cookie by name.
 * @param {string} name - The name of the cookie to delete.
 * @param {string} [path] - The path of the cookie.
 * @param {string} [domain] - The domain of the cookie.
 */
export function deleteCookie( name, path, domain ) {
  if( cookieExists( name ) ) {
    document.cookie = name + "=" +
      ((path) ? ";path=" + path : "") +
      ((domain) ? ";domain=" + domain : "") +
      ";expires=Thu, 01 Jan 1970 00:00:01 GMT";
  }
}

/**
 * Retrieves the value of a cookie by its name.
 * @param {string} name - The name of the cookie.
 * @returns {string|null} - The value of the cookie, or null if the cookie is not found.
 */
export function getCookie(name) {
  let cookieValue = null;
  if (document.cookie && document.cookie !== '') {
    let cookies = document.cookie.split(';');
    for (let i = 0; i < cookies.length; i++) {
      let cookie = cookies[i].trim();
      // Does this cookie string begin with the name we want?
      if (cookie.substring(0, name.length + 1) === (name + '=')) {
        cookieValue = decodeURIComponent(cookie.substring(name.length + 1));
        break;
      }
    }
  }
  return cookieValue;
}

  