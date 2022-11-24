import React from 'react';
import logo from './logo.svg';
import './App.css';
import LoginPage from './LoginPage/LoginPage';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from './HomePage/HomePage';
import PhotosPage from './PhotosPage/PhotosPage';

function App() {
  const [isLoggedIn, setIsLoggedIn] = React.useState<boolean>(false);

  return (
    <div className="App">
      <LoginPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
      <BrowserRouter>
        <Routes>
          <Route path="/" element={
            <HomePage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
          } />
          <Route path="/photos" element={
            <PhotosPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} />
          } />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
