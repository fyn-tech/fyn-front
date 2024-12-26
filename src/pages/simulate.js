import React, { useContext, useState } from 'react';
import UserContext from '../context/user_context';
import SimulationRenderer from '../components/SimulationRenderer';
import Form from '../components/form';
import DynamicSplitScreen from '../components/DynamicSplitScreen';
import { fetchCsrfToken, post } from '../utilities/api_service';
import './general_page.css';

const Simulate = () => {
    const [geometry, set_geometry] = useState(null); 
    const { isLoggedIn, user } = useContext(UserContext);

    const title = 'Simulation Configuration';
    const format = 'yaml';
    const endpoint = 'http://localhost:8000/simulation_submission/';
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
         set_geometry(newFormState['mesh_file']);
    };
    
    return (
      <DynamicSplitScreen startingPaneWidth={'23%'}>
          <Form
            title={title}
            fields={fields}
            columns={1}
            button_text={button_text}
            onSubmit={onSubmit}
            on_change={handleFormChange}
          > </Form>
          <SimulationRenderer file_path={geometry} />
        </DynamicSplitScreen>
    );
};

export default Simulate;