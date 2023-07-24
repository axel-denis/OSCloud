import React, { useReducer } from 'react';
import './App.css';
import LoginPage from './LoginPage/LoginPage';
import HomePage from './HomePage/HomePage';
import PhotosPage from './PhotosPage/PhotosPage';
import FilesPage from './FilesPage/FilesPage';
import {urlToInfo, discreetlyChangeUrlPath, UrlInfo } from './UrlGestion';
import { timeScale } from './consts';

export default function App() {
  const [isLoggedIn, setIsLoggedIn] = React.useState<boolean>(true); //FIXME - set to false
  const [urlsHandler, setUrlsHandler] = React.useState<UrlInfo>(urlToInfo(window.location.pathname));
  const urlsRef = React.useRef(urlsHandler); // permet d'avoir la vraie valeur de la state même dans les setTimeout
  urlsRef.current = urlsHandler;

  // console.log(window.location.pathname)
  // window.history.replaceState(null, "", "/pathname/goes/here")
  React.useEffect(() => {
    if (window.location.pathname === "/") {
      discreetlyChangeUrlPath("/Home")
      setUrlsHandler(urlToInfo("Home"));
    }
  }, []);

  return (
    <div className="App">
      <LoginPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
      <HomePage appName="Home" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
      <PhotosPage appName="Photos" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
      {/* <FilesPage appName="Files" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} /> */}
    </div>
  );
}
