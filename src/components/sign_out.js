import React, { useContext, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import UserContext from '../context/user_context';
import { post } from '../utilities/api-service';
import { API_ENDPOINTS } from '../utilities/api-config';

const SignOut = () => {
  const { logOut } = useContext(UserContext);

  const signOut = useCallback(async () => {
    const response = await post(API_ENDPOINTS.SIGN_OUT, 'json', {});
    if (response.status === 'success') {
      logOut();
      localStorage.removeItem('user');
    }
  }, [logOut]); // Add logOut to dependency array since it's used inside

  useEffect(() => {
    window.addEventListener('beforeunload', signOut);
    return () => {
      window.removeEventListener('beforeunload', signOut);
    };
  }, [signOut]);

  return (
    <Link to="/simulate" onClick={signOut}> Sign Out </Link>
  );
};

export default SignOut;