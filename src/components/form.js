import React, { useState, useEffect } from 'react';
import Button from './button';
import '../assets/form.css'; 

const Form = ({ title, fields, columns, button_text, onSubmit, on_change}) => {
  const formClass = `form-${columns}-columns`;
  const formTitle = title || 'Form';
  const [errorMessage, setErrorMessage] = useState(null); 
  const storedFormState = JSON.parse(sessionStorage.getItem(formTitle + '_form_state'));
  const initialFormState = storedFormState || fields.reduce(
    (acc, field) => ({ ...acc, [field.name]: field.defaultValue || '' }), {}
  );
  const [formState, setFormState] = useState(initialFormState);

  useEffect(() => {
    if(on_change) {
      on_change(formState);
    }
  }, [formState, on_change]);

  const handleChange = (event) => {
    const { name, value } = event.target;
    const isPasswordField = !!fields.find(field => field.name === name && field.type === 'password');
    const isFileField = !!fields.find(field => field.name === name && field.type === 'file');

    if (isPasswordField) {
      setFormState(prevState => {
        const newState = { ...prevState, [name]: value };
        return newState;
      });
    } else if( isFileField ) {
      setFormState(prevState => {
        const newState = { ...prevState, [name]: event.target.files[0] };
        return newState;
      });
    } else {
      setFormState(prevState => {
        const newState = { ...prevState, [name]: value };
        sessionStorage.setItem(formTitle + '_form_state', JSON.stringify(newState));
        return newState;
      });
    }
  };

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      const response = await onSubmit(formState);
      if (response.status === 'error') {
        setErrorMessage(response.message);
      } else {
        setErrorMessage(null);
     
        // clear the stored version.
        setFormState(initialFormState);
        sessionStorage.removeItem(formTitle + '_form_state');
      }
    } catch (error) {
      setErrorMessage(error.message);
    }
  };

  const renderFields = (fields) => {
    return fields.map((field, index) => {
      if ('fields' in field) {
        // This is a section
        return (
          <div key={index} className="form-section">
            <h3>{field.title}</h3>
            {renderFields(field.fields)}
          </div>
        );
      } else {
        // This is a field
        return (
          <div key={index} className="form-element">
            <label className="form-label">{field.label}</label>
            <input className="form-input"
              name={field.name} 
              type={field.type}
              required={field.required} 
              value={field.type === 'file' ? undefined : (formState[field.name] || '')}
              defaultValue={field.defaultValue} 
              onChange={handleChange} />
          </div>
        );
      }
    });
  };

  return (
  <div className="form-container">
    <form onSubmit={handleSubmit} className={formClass}>
        <div className="form-title-row">
          <h2>{formTitle}</h2>
        </div>
        {renderFields(fields)}
        <div className="button-container-1">
          <div className="button-container-2">
            {errorMessage && <div className="error-message">{errorMessage}</div>}
            <Button text={button_text} />
          </div>
        </div>
      </form>
    </div>
    );
  };

export default Form;