import React, { useContext, useEffect } from 'react';
import { Link } from 'react-router-dom';
import UserContext from '../context/user_context';
import { post } from '../utilities/api_service';
import { API_ENDPOINTS } from '../utilities/api_config';

const SignOut = () => {
  const { logOut } = useContext(UserContext);

  const signOut = async () => {
    const response = await post(API_ENDPOINTS.SIGN_OUT, 'json', {});
    if (response.status === 'success') {
      logOut();
      localStorage.removeItem('user');
    }
  };

  useEffect(() => {
    window.addEventListener('beforeunload', signOut);
    return () => {
      window.removeEventListener('beforeunload', signOut);
    };
  }, []);

  return (
    <Link to="/simulate" onClick={signOut}> Sign Out </Link>
  );
};

export default SignOut;