import React, { useRef, useEffect, useCallback } from 'react';
import styled from 'styled-components';

import ResizeObserver from 'resize-observer-polyfill';
import * as THREE from 'three';
import { Scene } from '../utilities/three_scene.js';
import AxisCanvas from './axis_canvas.js';
import Button from './base/Button.js';
import Toolbar from './base/Toolbar.js';

const SimulationContainer = styled.div.attrs({
  className: 'SimulationContainer',
})`
  display: flex;
  flex-wrap: wrap;
  align-items: flex-start;
  flex-direction: column;
  width: 100%;
  height: 100%;
`;

const SimulationRenderer = ({file_path}) => {
  const canvasRef = useRef();
  const sceneRef = useRef();
  const rendererRef = useRef();

  // Define the handleResize function using useCallback at the top level
  const handleResize = useCallback(() => {
    // Capture current values at time of execution
    const canvas = canvasRef.current;
    const scene = sceneRef.current;
    const renderer = rendererRef.current;
  
    if (canvas && scene && renderer) {
      const width = canvas.clientWidth;
      const height = canvas.clientHeight;
  
      // Use captured values
      scene.camera.aspect = width / height;
      scene.camera.updateProjectionMatrix();
      
      renderer.setSize(width, height);
    }
  }, []);
  

  // Set up the scene
  useEffect(() => {
    const currentCanvas = canvasRef.current;
    if (currentCanvas) {
      sceneRef.current = new Scene(currentCanvas);
      
      // Create a renderer and attach it to the canvas
      rendererRef.current = new THREE.WebGLRenderer({ canvas: currentCanvas });
      rendererRef.current.setSize(currentCanvas.clientWidth, currentCanvas.clientHeight);
      sceneRef.current.setupCamera(rendererRef.current);

      // Update the animate function to use the renderer
      const animate = () => {
        requestAnimationFrame(animate);
        sceneRef.current.update();
        rendererRef.current.render(sceneRef.current.scene, sceneRef.current.camera);
      };

      if(file_path) {
        sceneRef.current.add_object(file_path);
      }

      const resizeObserver = new ResizeObserver(entries => {
        for (let entry of entries) {
          // Ensure the callback is not called more than once per frame
          requestAnimationFrame(() => {
            if (entry.target === currentCanvas) {
              handleResize();
            }
          });
        }
      });

      resizeObserver.observe(currentCanvas);
      animate();
      
      return () => {
        if (currentCanvas instanceof Element) {
          resizeObserver.unobserve(currentCanvas);
        }
      };

    }
  }, [file_path, handleResize]);

  const handleClick = (buttonName) => {
    console.log(`Clicked ${buttonName}`);
  };
  
  return (
    <SimulationContainer>
      <Toolbar isVertical={false}>
        <Button text={'Button 1'} onClick={() => handleClick('Button 4')}></Button>
        <Button text={'Button 2'} onClick={() => handleClick('Button 5')}></Button>
        <Button text={'Button 3'} onClick={() => handleClick('Button 6')}></Button>
      </Toolbar>
      <canvas ref={canvasRef} style={{ flexGrow: 1 }} />
      <AxisCanvas sceneRef={sceneRef} />
    </SimulationContainer>
  );
}

export default SimulationRenderer;