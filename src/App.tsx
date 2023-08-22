import React, { useReducer } from 'react';
import './App.css';
import LoginPage from './LoginPage/LoginPage';
import HomePage from './HomePage/HomePage';
import PhotosApp from './PhotosApp/PhotosApp';
import { urlToInfo, discreetlyChangeUrlPath, UrlInfo } from './UrlGestion';
import { timeScale } from './consts';
import FilesApp from './FilesApp/FilesApp';

export const MobileDevice = React.createContext<boolean>(false);

export default function App() {
  const [isLoggedIn, setIsLoggedIn] = React.useState<boolean>(true); //FIXME - set to false
  const [urlsHandler, setUrlsHandler] = React.useState<UrlInfo>(urlToInfo(window.location.pathname));
  const urlsRef = React.useRef(urlsHandler); // permet d'avoir la vraie valeur de la state même dans les setTimeout
  const [isMobile, setIsMobile] = React.useState<boolean>(false);
  urlsRef.current = urlsHandler;

  // console.log(window.location.pathname)
  // window.history.replaceState(null, "", "/pathname/goes/here")
  React.useEffect(() => {
    if (window.location.pathname === "/") {
      discreetlyChangeUrlPath("/Home")
      setUrlsHandler(urlToInfo("Home"));
    }
    const checkDevice = () => {
      setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches)
    }
    checkDevice();
    window.addEventListener("resize", checkDevice);
    return (() => window.removeEventListener("resize", checkDevice));
  }, []);

  return (
    <MobileDevice.Provider value={isMobile}>
      <div className="App">
        <LoginPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
        <HomePage appName="Home" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
        <PhotosApp appName="Photos" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
        <FilesApp appName="Files" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
      </div>
    </MobileDevice.Provider>
  );
}
