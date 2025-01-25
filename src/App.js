import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import UserContext from './context/user_context';
import NavigationBar from './components/navigation_bar';
import Register from './pages/register';
import SignIn from './pages/sign_in';
import Simulate from './pages/simulate';
import { deleteCookie } from './utilities/cookies';
import { fetchCsrfToken } from './utilities/api-service';
import './App.css';

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [user, setUser] = useState(null);
  const [isBackendOnline, setIsBackendOnline] = useState(false);
  const [lastOnlineCheck, setLastOnlineCheck] = useState(0);
  
  const logIn = (userData) => {
    setIsLoggedIn(true);
    setUser(userData);
    localStorage.setItem('user', JSON.stringify(userData));
  };

  const logOut = () => {
    setIsLoggedIn(false);
    setUser(null);
    localStorage.removeItem('user');
  };

  const handleBeforeUnload = () => {
    deleteCookie('csrftoken');
  };

  // Check backend connectivity and manage CSRF token
  useEffect(() => {
    const checkBackendStatus = async () => {
      try {
        // Only attempt to fetch CSRF if we haven't tried in the last 10 seconds
        const now = Date.now();
        if (now - lastOnlineCheck < 10000) {
          return;
        }
        setLastOnlineCheck(now);

        await fetchCsrfToken();
        if (!isBackendOnline) {
          setIsBackendOnline(true);
        }
      } catch (error) {
        console.log('Backend connection failed:', error);
        setIsBackendOnline(false);
      }
    };

    // Initial check
    checkBackendStatus();

    // Set up periodic checking - only if currently offline
    const intervalId = !isBackendOnline ? 
      setInterval(checkBackendStatus, 30000) : null; // Check every 30 seconds if offline

    return () => {
      if (intervalId) clearInterval(intervalId);
    };
  }, [isBackendOnline, lastOnlineCheck]);

  // Handle user session
  useEffect(() => {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      setUser(JSON.parse(storedUser));
      setIsLoggedIn(true);
    }

    window.addEventListener('beforeunload', handleBeforeUnload);
    return () => {
      window.removeEventListener('beforeunload', handleBeforeUnload);
    };
  }, []);

  return (
    <UserContext.Provider value={{ isLoggedIn, user, logIn, logOut, isBackendOnline }}>
      <Router>
        <div className="navigation-header">  
          <NavigationBar />
        </div>
        <div className="content">
          <Routes>
            <Route path="/sign_in" element={<SignIn />} />
            <Route path="/register" element={<Register />} />
            <Route path="/simulate" element={<Simulate />} />
            <Route path="*" element={<Simulate />} />
          </Routes>
        </div>
      </Router>
    </UserContext.Provider>
  );
}

export default App;