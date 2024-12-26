import React from 'react';
import styled from 'styled-components';

const ToolbarContainer = styled.div.attrs(({ isVertical }) => ({
  className: 'ToolbarContainer',
  isVertical: undefined,
}))`
  display: flex;
  align-items: stretch;
  align-content: stretch;
  justify-content: space-between;
`;
//flex-direction: ${props => props.isVertical ? 'column' : 'row'};

const Toolbar = ({ children, isVertical }) => {
  return (
    <ToolbarContainer isVertical={isVertical}>
      {children}
    </ToolbarContainer>
  );
};

export default Toolbar;