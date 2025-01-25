import React from 'react';
import styled from 'styled-components';

const StyledButton = styled.button`
    background-color: #61dafb;
    border: none;
    border-radius: 2px; 
    color: white;
    padding: 10px 20px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 14px;
    margin: 4px 2px;
    cursor: pointer;
    transition-duration: 0.4s;
    width: 100px; 
    height: 40px; 
    line-height: 20px; 
    overflow: hidden; 

    &:hover {
        background-color: #bbeeff;
    }

    &.success {
        background-color: #4CAF50;
    }
`;

const Button = ({text, onClick, variant}) => {
  return (
    <StyledButton onClick={onClick} variant={variant}>{text}</StyledButton>
  );
};

export default Button;