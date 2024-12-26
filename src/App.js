import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import UserContext from './context/user_context';
import NavigationBar from './components/navigation_bar';
import Register from './pages/register';
import SignIn from './pages/sign_in';
import Simulate from './pages/simulate';
import { deleteCookie } from './utilities/cookies';
import { fetchCsrfToken } from './utilities/api_service';
import './App.css';

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [user, setUser] = useState(null);
  
  const logIn = (userData) => {
    setIsLoggedIn(true);
    setUser(userData);
  };

  const logOut = () => {
    setIsLoggedIn(false);
    setUser(null);
  };

  const handleBeforeUnload = () => {
    deleteCookie('csrftoken');
  };

  useEffect(() => {
    // check if there is a stored user
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      setUser(JSON.parse(storedUser));
      setIsLoggedIn(true);
    }

    fetchCsrfToken(); // get a new csrf token

    // make sure to delete the cookie when the user leaves the page
    window.addEventListener('beforeunload', handleBeforeUnload);
    return () => {
      window.removeEventListener('beforeunload', handleBeforeUnload);
    };
  }, []);

  return (
    <UserContext.Provider value={{ isLoggedIn, user, logIn, logOut }}>
      <Router>
        <div className="navigation-header">  
        <NavigationBar /> {}
        </div>
        <div className="content">
          <Routes>
            <Route path="/sign_in" element={<SignIn />} />
            <Route path="/register" element={<Register />} />
            <Route path="/simulate" element={<Simulate /> } />
            <Route path="*" element={<Simulate /> } />
          </Routes>
        </div>
      </Router>
    </UserContext.Provider>
  );
}

export default App;