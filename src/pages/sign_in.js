import React, { useContext } from 'react';
import { useNavigate } from 'react-router-dom';
import UserContext from '../context/user_context'; 
import Form from '../components/form';
import { API_ENDPOINTS } from '../utilities/api_config';
import { fetchCsrfToken, post } from '../utilities/api_service';
import './general_page.css';

const SignIn = () => {
  const title = 'Sign in';
  const format = 'json';
  const endpoint = API_ENDPOINTS.SIGN_IN;
  const button_text = 'Sign In';
  const fields = [
    { label: 'Username', name: 'username', type: 'text', required: true },
    { label: 'Password', name: 'password', type: 'password', required: true },
  ];
  const navigate = useNavigate();
  const { logIn } = useContext(UserContext);

  fetchCsrfToken();

  const onSubmit = async (formData) => {
    const response = await post(endpoint, format, formData);
    if (response.status === 'success') { 
      logIn(response.userData);
      localStorage.setItem('user', JSON.stringify(response.userData));
      navigate('/simulate');
    }
    return response;
  };

  return (
    <div className="main-content">
      <Form 
        title={title} 
        fields={fields}
        columns={2}
        button_text={button_text}
        onSubmit={onSubmit}
      > </Form>
    </div>
  );
}

export default SignIn;