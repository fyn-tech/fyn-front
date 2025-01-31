import React, { useState, useEffect } from 'react';
import Toolbar from '../base/Toolbar';
import Button from '../base/Button';
import SimulationConfiguration from './SimulationConfiguration';
import DataTable from '../base/DataTable';
import styles from './activity-space.module.css'

// fictious data.
export const hardwareData = [
  {
    computerName: "COMP-001-LAB",
    currentJob: "CFD-Analysis-329",
    physical: "Lab-Room-101",
    CPUs: 32,
    Memory: "128GB",
    GPU: "NVIDIA RTX 4090",
    Harddrive: "2TB SSD",
    lastContact: "2024-01-25 13:45:22"
  },
  {
    computerName: "WORKSTATION-A15",
    currentJob: "Idle",
    physical: "Office-203",
    CPUs: 16,
    Memory: "64GB",
    GPU: "NVIDIA RTX 3080",
    Harddrive: "1TB NVMe",
    lastContact: "2024-01-25 13:44:58"
  },
  {
    computerName: "SERVER-CALC-01",
    currentJob: "MATLAB-Sim-892",
    physical: "Server-Room-1",
    CPUs: 64,
    Memory: "256GB",
    GPU: "NVIDIA A6000",
    Harddrive: "4TB RAID",
    lastContact: "2024-01-25 13:45:01"
  },
  {
    computerName: "HPC-NODE-05",
    currentJob: "FEA-Batch-556",
    physical: "Data-Center-3",
    CPUs: 48,
    Memory: "192GB",
    GPU: "2x NVIDIA A100",
    Harddrive: "2TB NVMe",
    lastContact: "2024-01-25 13:44:12"
  },
  {
    computerName: "RENDER-WS-02",
    currentJob: "Rendering-Job-445",
    physical: "Graphics-Lab",
    CPUs: 24,
    Memory: "128GB",
    GPU: "NVIDIA RTX 4080",
    Harddrive: "4TB SSD",
    lastContact: "2024-01-25 13:45:15"
  },
  {
    computerName: "COMPUTE-VM-08",
    currentJob: "Idle",
    physical: "Virtual",
    CPUs: 8,
    Memory: "32GB",
    GPU: "NVIDIA T4",
    Harddrive: "500GB SSD",
    lastContact: "2024-01-25 13:43:55"
  }
];


const ActivityType = Object.freeze({
  ANALYSIS_SETUP: 'ANALYSIS_SETUP',
  HARDWARE_MANAGER: 'HARDWARE_MANAGER',
});

const ActivitySpace = ({ setGeometry }) => {
  const [activityState, setActivityState] = useState(ActivityType.ANALYSIS_SETUP); 
  const [activityComponent, setActivityComponent] = useState(null);

  useEffect(() => {
    switch (activityState) {
      case ActivityType.ANALYSIS_SETUP:
          setActivityComponent(<SimulationConfiguration setGeometry={setGeometry}></SimulationConfiguration>);
          break;
      case ActivityType.HARDWARE_MANAGER:
          setActivityComponent(<DataTable name="Empty Table" data={""} />);
          break;
      default:
          setActivityComponent(null);
    }
  }, [activityState, setGeometry]);


  // Example of an activity component
  return (
  <div className={styles.activity_space}>
    <Toolbar isVertical={true}>
      <Button text={"AS"} onClick={() => setActivityState(ActivityType.ANALYSIS_SETUP)}></Button>
      <Button text={"HWM"} onClick={() => setActivityState(ActivityType.HARDWARE_MANAGER)}></Button>
    </Toolbar>
    <div className={styles.divider}></div>
    {activityComponent}
  </div>
);

};


export default ActivitySpace;