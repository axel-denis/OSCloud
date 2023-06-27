import React from 'react';
import './App.css';
import LoginPage from './LoginPage/LoginPage';
import { BrowserRouter, Routes, Route } from "react-router-dom";
import HomePage from './HomePage/HomePage';
import PhotosPage from './PhotosPage/PhotosPage';
import FilesPage from './FilesPage/FilesPage';
import { UrlsHandler, UrlInfo, urlToInfo } from './UrlGestion';
import CustomRouter from './CustomRouter/CustomRouter';

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
  const [appsState, setAppsState] = React.useState<UrlsHandler>({
    actualUrl: {app: "Home", parameters: null},
    nextUrl: {app: "", parameters: null}
  });
  console.log(urlToInfo(window.location.pathname))
  // console.log(window.location.pathname)
  // window.history.replaceState(null, "", "/pathname/goes/here")
  return (
    <div className="App">
      {/* <CustomRouter openedApps={appsState}>
        <TestAppName appName="t1">t1</TestAppName>
        <TestAppName appName="t2">t1</TestAppName>
      </CustomRouter> */}
      <LoginPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
      <BrowserRouter>
        <Routes>
          <Route path="/" element={
            <HomePage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
          } />
          <Route path="/photos" element={
            <PhotosPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} />
          } />
          <Route path="/files" element={
            <FilesPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} />
          } />
        </Routes>
      </BrowserRouter>
    </div>
  );
}
