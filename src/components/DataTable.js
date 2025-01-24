import React, { useState } from 'react';
import styles from '../assets/DataTable.module.css';

const DataTable = ({ name, data }) => {
  const [sortConfig, setSortConfig] = useState({ key: null, direction: 'asc' });

  if (!data?.length) {
    return <div className={styles.empty}>No data available</div>;
  }

  const columns = Object.keys(data[0]);

  const sortedData = [...data].sort((a, b) => {
    if (!sortConfig.key) return 0;
    
    const aValue = a[sortConfig.key];
    const bValue = b[sortConfig.key];
    
    if (aValue < bValue) return sortConfig.direction === 'asc' ? -1 : 1;
    if (aValue > bValue) return sortConfig.direction === 'asc' ? 1 : -1;
    return 0;
  });

  const requestSort = (key) => {
    const direction = 
      sortConfig.key === key && sortConfig.direction === 'asc' ? 'desc' : 'asc';
    setSortConfig({ key, direction });
  };

  return (
    <div className={styles.wrapper}>
      <h1 className={styles.h1}>{name}</h1>
        <table className={styles.table}>
          <thead>
            <tr className={styles.header}>
              {columns.map((column) => (
                <th
                  key={column}
                  onClick={() => requestSort(column)}
                  className={styles.headerCell}
                >
                  {column}
                  {sortConfig.key === column && (
                    <span className={styles.sortIcon}>
                      {sortConfig.direction === 'asc' ? '↑' : '↓'}
                    </span>
                  )}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {sortedData.map((row, rowIndex) => (
              <tr key={rowIndex} className={styles.row}>
                {columns.map((column) => (
                  <td key={column} className={styles.cell}>
                    {row[column]}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
    </div>
  );
};

export default DataTable;