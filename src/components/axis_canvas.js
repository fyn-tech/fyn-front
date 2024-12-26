import React, { useEffect, useRef } from 'react';
import * as THREE from 'three';

const AxisCanvas = ({ sceneRef }) => {
  const axisCanvasRef = useRef();
  const axisSceneRef = useRef();
  const axisRendererRef = useRef();

  useEffect(() => {
    const currentAxisCanvas = axisCanvasRef.current;
    const currentScene = sceneRef.current;

    if (currentAxisCanvas && currentScene && currentScene.camera) {
      // Create renderer
      axisRendererRef.current = new THREE.WebGLRenderer({ 
        canvas: currentAxisCanvas, 
        alpha: true 
      });
      axisRendererRef.current.setSize(currentAxisCanvas.clientWidth, currentAxisCanvas.clientHeight);

      // Create scene
      axisSceneRef.current = new THREE.Scene();

      // Create camera
      const axisCamera = new THREE.PerspectiveCamera(
        75, 
        currentAxisCanvas.clientWidth / currentAxisCanvas.clientHeight, 
        0.1, 
        1000
      );
      axisCamera.position.z = 5;

      // Add axis helper
      const axesHelper = new THREE.AxesHelper(2.5);
      axesHelper.position.setLength(2);
      axisSceneRef.current.add(axesHelper);

      const currentRenderer = axisRendererRef.current;
      const currentAxisScene = axisSceneRef.current;

      currentRenderer.render(currentAxisScene, axisCamera);

      // Animation loop
      const animate = () => {
        requestAnimationFrame(animate);

        if (currentScene.camera) {
          // Apply inverse transformation
          const cameraTransformation = currentScene.camera.matrix.clone();
          axesHelper.matrix.copy(cameraTransformation).invert();
          axesHelper.rotation.setFromRotationMatrix(axesHelper.matrix);
          currentRenderer.render(currentAxisScene, axisCamera);
        }
      };

      animate();

      // Cleanup
      return () => {
        currentRenderer.dispose();
        if (currentAxisScene) {
          while(currentAxisScene.children.length > 0) {
            currentAxisScene.remove(currentAxisScene.children[0]);
          }
        }
      };
    }
  }, [sceneRef]); // Only depend on sceneRef

  return (
    <canvas 
      ref={axisCanvasRef} 
      style={{ 
        position: 'fixed',  
        bottom: '0', 
        width: '200px', 
        height: '200px' 
      }} 
    />
  );
};

export default AxisCanvas;