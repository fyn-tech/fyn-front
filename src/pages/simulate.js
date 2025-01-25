import React, { useState } from 'react';
import SimulationRenderer from '../components/SimulationRenderer';
import DynamicSplitScreen from '../components/DynamicSplitScreen';
import ActivitySpace from '../components/container/ActivitySpace';
import './general_page.css';

const Simulate = () => {
  const [geometry, setGeometry] = useState(null);

    
  return (
    <DynamicSplitScreen startingPaneWidth={'30%'}>
      <ActivitySpace setGeometry={setGeometry} />
      <SimulationRenderer file_path={geometry} />
    </DynamicSplitScreen>
  );
};

export default Simulate;