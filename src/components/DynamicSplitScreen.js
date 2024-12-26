import React, { useState, useRef } from 'react';
import styled from 'styled-components';

const Container = styled.div`
  display: flex;
  width: 100%;
  user-select: none; 
`;
const Pane = styled.div`
  overflow: auto;
`;

const Divider = styled.div`
  width: 5px;
  cursor: col-resize;
  background-color: #ccc;
`;

const DynamicSplitScreen = ({
    startingPaneWidth,
    children,
}) => {

  const [dragging, setDragging] = useState(false);
  const [paneWidth, setPaneWidth] = useState(startingPaneWidth);
  const containerRef = useRef();
  const [left, right] = children;

  const handleMouseDown = (e) => {
    setDragging(true);
  };

  const handleMouseUp = () => {
    setDragging(false);
  };

  const handleMouseMove = (e) => {
    if (dragging) {
      const containerRect = containerRef.current.getBoundingClientRect();
      const newPaneWidth = ((e.clientX - containerRect.left) / containerRect.width) * 100;
      setPaneWidth(`${newPaneWidth}%`);
    }
  };

  return (
    <Container
      ref={containerRef}
      onMouseMove={handleMouseMove}
      onMouseUp={handleMouseUp}
      style={{ userSelect: dragging ? 'none' : 'auto' }}
      >
        <Pane style={{ width: paneWidth }}>
          {left}
        </Pane>
        <Divider onMouseDown={handleMouseDown} />
        <Pane style={{ width: `calc(100% - ${paneWidth})` }}>
          {right}
        </Pane>
    </Container>
  );
} 

export default DynamicSplitScreen;