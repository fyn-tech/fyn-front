import React, { useContext } from 'react';
import { API_ENDPOINTS } from '../../utilities/api-config';
import { fetchCsrfToken, post } from '../../utilities/api-service';
import Form from './Form';
import UserContext from '../../context/user_context';

const SimulationConfiguration = ({ setGeometry }) => { // need a fix for geometry.
    const title = 'Simulation Configuration';
    const { isLoggedIn, user } = useContext(UserContext);
    const format = 'yaml';
    const endpoint = API_ENDPOINTS.SIMULATION_SUBMISSION;
    const button_text = 'Submit';
    const fields = [
      { label: 'Simulation Time', name: 'time', type: 'number', required: true },
      { label: 'Initial Time Step Size', name: 'time_step_start', type: 'number', required: true },
      { label: 'CFL', name: 'cfl', type: 'number', required: true },
      { label: 'Upwinding Scheme', name: 'up_scheme', type: 'number', required: true },
      { label: 'Convergence Criterion', name: 'convergence', type: 'number', required: true },
      { label: 'Smagorinsky Constant', name: 'smagorinsky_constant', type: 'number', required: true },
      { label: 'Density', name: 'density', type: 'number', required: true },
      { label: 'Viscosity', name: 'viscosity', type: 'number', required: true },
      { label: 'Inlet Velocity', name: 'velocity', type: 'number', required: true },
      { label: 'Mesh Size', name: 'mesh_size', type: 'number', required: true },
      { label: 'Geometry', name: 'mesh_file', type: 'file', required: false }, //todo: sort out file upload
    ];
    
    fetchCsrfToken();

    const onSubmit = async (formData) => {
       
      if (!isLoggedIn) {
        return {status: 'error', message: 'Please log in to submit.' };
      }

      formData.username = user.username;

      const response = await post(endpoint, format, formData);
      return response;
    };

    const handleFormChange = (newFormState) => {
      setGeometry(newFormState['mesh_file']);
    };
  
    return (
      <Form
        title={title}
        fields={fields}
        columns={1}
        button_text={button_text}
        onSubmit={onSubmit}
        on_change={handleFormChange}
      > </Form>);
};

export default SimulationConfiguration;