import React from 'react';
import './App.css';
import LoginPage from './LoginPage/LoginPage';
import HomePage from './HomePage/HomePage';
import PhotosPage from './PhotosPage/PhotosPage';
import FilesPage from './FilesPage/FilesPage';
import {urlToInfo, discreetlyChangeUrlPath, UrlInfo } from './UrlGestion';

function TestAppName(props: any) {
  return (<>
    tezst<br></br>
    tezst<br></br>
    tezst<br></br>
    tezst<br></br>
    tezst<br></br>
    tezst<br></br>
    {props.appName}
  </>);
}

export default function App() {
  const [isLoggedIn, setIsLoggedIn] = React.useState<boolean>(true); //FIXME - set to false
  const [urlsHandler, setUrlsHandler] = React.useState<UrlInfo[]>([urlToInfo(window.location.pathname)]);
  // console.log(window.location.pathname)
  // window.history.replaceState(null, "", "/pathname/goes/here")
  React.useEffect(() => {
    if (window.location.pathname === "/") {
      discreetlyChangeUrlPath("/Home")
      setUrlsHandler([urlToInfo("Home")]);
    }
  }, [])

  React.useEffect(() => {
    console.log("useeffect urlhandler", urlsHandler)
    if (urlsHandler.length >= 2) {
      setTimeout(() => {
        setUrlsHandler(urlsHandler.slice(1));
      }, 500);
    }
  }, [urlsHandler]);
  return (
    <div className="App">
      <LoginPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
      <HomePage appName="Home" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
      <PhotosPage appName="Photos" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
      <FilesPage appName="Files" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
    </div>
  );
}
