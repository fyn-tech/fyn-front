import React, { useRef, useEffect, useCallback } from 'react';
import styled from 'styled-components';

import ResizeObserver from 'resize-observer-polyfill';
import * as THREE from 'three';
import { Scene } from '../utilities/three_scene.js';
import AxisCanvas from './axis_canvas.js';

const SimulationContainer = styled.div.attrs({
  className: 'SimulationContainer',
})`
  display: flex;
  flex-wrap: wrap;
  align-items: flex-start;
  flex-direction: column;
  flex: 1;
  width: 100%;
  height: 100%;
  position: relative;
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
      const container = canvas.parentElement;
      const width = container.clientWidth;
      const height = container.clientHeight;
      
      canvas.width = width;
      canvas.height = height;

      // Use captured values
      scene.camera.aspect = width / height;
      scene.camera.updateProjectionMatrix();
      
      renderer.setPixelRatio(window.devicePixelRatio);
      renderer.setSize(width, height, false);
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
      window.addEventListener('resize', handleResize);
      animate();
      
      return () => {
        if (currentCanvas instanceof Element) {
          resizeObserver.unobserve(currentCanvas);
        }
      };
      
    }
  }, [file_path, handleResize]);
  
  return (
    <SimulationContainer>
      <canvas ref={canvasRef} style={{ flexGrow: 1, width: "100vw"}} />
      <AxisCanvas sceneRef={sceneRef} />
    </SimulationContainer>
  );
}

export default SimulationRenderer;