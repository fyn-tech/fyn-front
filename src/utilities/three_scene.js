import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { STLLoader } from 'three/examples/jsm/loaders/STLLoader.js';

export class Scene {
    
    constructor(canvas) {
        // Create the scene
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0xf0f0f0); // Very light grey
        
        // Create the ambient light
        this.ambientLight = new THREE.AmbientLight(0xffffff); // Soft white light
        this.ambientLight.intensity = 0.5;
        this.scene.add(this.ambientLight);
               
    }

    setupCamera(renderer, near = 0.1, far = 1000) {
        const fov = 75;
        const aspect = window.innerWidth / window.innerHeight;  // the canvas default
        
        // Create the camera
        this.camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
        this.camera.position.z = 5;
        this.camera.lookAt( 0, 0, 0 );

        // Create the light
        this.spotlight = new THREE.PointLight(0xffffff, 1, 1000);
        this.spotlight.decay = 0; 
        this.spotlight.position.copy(this.camera.position);
        this.spotlight.lookAt(this.camera.getWorldDirection(new THREE.Vector3()));
        this.spotlight.angle = Math.PI / 2;
        this.scene.add(this.spotlight);

        // Update the controls with the new camera
        this.controls = new OrbitControls(this.camera, renderer.domElement);
    }

    add_object(file) {
        const reader = new FileReader();
        reader.onload = (event) => {
          const contents = event.target.result;
    
          const loader = new STLLoader();
          const geometry = loader.parse(contents);
    
          const material = new THREE.MeshStandardMaterial({ color: 0x808080, metalness: 0.5, roughness: 0.5 });
          const mesh = new THREE.Mesh(geometry, material);
          this.scene.add(mesh);

          // Calculate the bounding box of the object
          const boundingBox = new THREE.Box3().setFromObject(mesh);
          const size = boundingBox.getSize(new THREE.Vector3());
          const center = boundingBox.getCenter(new THREE.Vector3());
          const maxDim = Math.max(size.x, size.y, size.z);
    
          // Update the camera
          this.camera.near = maxDim / 100; // Near clipping plane at 1% of the largest dimension
          this.camera.far = maxDim * 10000; // Far clipping plane at 100 times the largest dimension
          this.camera.position.x = center.x + maxDim;
          this.camera.position.y = center.y + maxDim;
          this.camera.position.z = center.z + maxDim;  
          this.camera.lookAt(center);
          this.camera.updateProjectionMatrix();
        };
        reader.readAsBinaryString(file);
    }

    update() {

        // Update the controls
        this.controls.update();

        // Update the spotlight to match the camera's position and direction
        this.spotlight.position.copy(this.camera.position);
        this.spotlight.lookAt(this.camera.getWorldDirection(new THREE.Vector3()));
    }
}