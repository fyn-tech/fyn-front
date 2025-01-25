import React from 'react';
import styles from "./toolbar.module.css";

const Toolbar = ({ children, isVertical }) => {
  return (
    <div 
      className={`${styles.toolbarContainer} ${isVertical ? styles.vertical : styles.horizontal}`}
    >
      {children}
    </div>
  );
};

export default Toolbar;