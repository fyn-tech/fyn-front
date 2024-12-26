import React, { useContext } from 'react';
import { Link } from 'react-router-dom';
import UserContext from '../context/user_context';
import SignOut from './sign_out';
import '../assets/navigation_bar.css'; 

const NavigationBar = () => {
  const { isLoggedIn, user } = useContext(UserContext);
  const initials = isLoggedIn && user ? `${user.first_name[0]}${user.last_name[0]}` : '';

  return (
    <header className="header">
      <div className="header-wrapper">
        <div className="logo">
            <Link to="/simulate" className="header-link">
                <h1>RankOne</h1>
            </Link>
            <p>Creativity Leads Innovation</p>
        </div>
        <nav className="navbar">
            {isLoggedIn ? (
              <>
              <Link to="/simulate">Simulate</Link>
              <SignOut />
              <Link>{initials}</Link>
              </>
              ) : (
              <>
                <Link to="/simulate">Simulate</Link>
                <Link to="/sign_in">Sign In</Link>
                <Link to="/register">Register</Link>
              </>
            )}
        </nav>
      </div>
    </header>
  );
};

export default NavigationBar;
