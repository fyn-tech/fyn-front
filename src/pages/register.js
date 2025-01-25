import React from 'react';
import Form from '../components/container/Form';
import { API_ENDPOINTS } from '../utilities/api-config';
import { fetchCsrfToken, post } from '../utilities/api-service';
import './general_page.css';

const Register = () => {
  const title = 'Register';
  const format = 'json';
  const endpoint = API_ENDPOINTS.REGISTER;
  const button_text = 'Register';
  const fields = [
    { label: 'First Name*', name: 'first_name', type: 'text', required: true },
    { label: 'Last Name', name: 'last_name', type: 'text', required: false },
    { label: 'Email*', name: 'email', type: 'email', required: true },
    { label: 'Username*', name: 'username', type: 'text', required: true },
    { label: 'Password*', name: 'password', type: 'password', required: true },
    { label: 'Company', name: 'company', type: 'text', required: false },
    { label: 'Country', name: 'country', type: 'text', required: false },
  ];

  fetchCsrfToken();

  const onSubmit = async (formData) => {
    const response = await post(endpoint, format, formData);
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
};

export default Register;