import React, { useEffect, useRef } from 'react';
import * as THREE from 'three';


const AxisCanvas = ({ sceneRef, canvasRef }) => {
  const axisCanvasRef = useRef();
  const axisSceneRef = useRef();
  const axisRendererRef = useRef();

  useEffect(() => {
    if (axisCanvasRef.current && sceneRef.current && sceneRef.current.camera) {
      axisRendererRef.current = new THREE.WebGLRenderer({ canvas: axisCanvasRef.current, alpha: true });
      axisRendererRef.current.setSize(axisCanvasRef.current.clientWidth, axisCanvasRef.current.clientHeight);

      // Create a scene for the axis
      axisSceneRef.current = new THREE.Scene();

      // Create a camera for the axis
      const axisCamera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
      axisCamera.position.z = 5;

      // Add the axis helper to the axis scene
      const axesHelper = new THREE.AxesHelper(2.5);
      axesHelper.position.setLength(2);
      axisSceneRef.current.add(axesHelper);

      axisRendererRef.current.render(axisSceneRef.current, axisCamera);

    
      // Update the animate function to use the renderer
      const animate = () => {
        requestAnimationFrame(animate);

        // Apply the inverse transformation to the axesHelper
        const cameraTransformation = sceneRef.current.camera.matrix.clone();
        axesHelper.matrix.copy(cameraTransformation).invert();
        axesHelper.rotation.setFromRotationMatrix(axesHelper.matrix);
        axisRendererRef.current.render(axisSceneRef.current, axisCamera);
      };

      animate();
    }
  }, [sceneRef.current]);

  return <canvas ref={axisCanvasRef} style={{ position: 'fixed',  bottom:'0', width: '200px', height: '200px' }} />;};

export default AxisCanvas;