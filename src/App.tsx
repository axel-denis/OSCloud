import React from 'react';
import './App.css';
import LoginPage from './LoginPage/LoginPage';
import HomePage from './HomePage/HomePage';
import PhotosPage from './PhotosPage/PhotosPage';
import FilesPage from './FilesPage/FilesPage';
import { UrlsHandler, urlToInfo, discreetlyChangeUrlPath } from './UrlGestion';
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
  const [urlsHandler, setUrlsHandler] = React.useState<UrlsHandler>({
    actualUrl: { app: "Home", parameters: null },
    nextUrl: { app: "", parameters: null },
    afterUrl: { app: "", parameters: null }
  });
  // console.log(window.location.pathname)
  // window.history.replaceState(null, "", "/pathname/goes/here")
  React.useEffect(() => {
    console.log("test1", window.location.pathname)
    if (window.location.pathname === "/") {
      discreetlyChangeUrlPath("/Home")
    }
    console.log("test2", window.location.pathname)
    setUrlsHandler({
      actualUrl: urlToInfo(window.location.pathname),
      nextUrl: urlToInfo(""),
      afterUrl: urlToInfo("")
    })
  }, [])
  return (
    <div className="App">
      <LoginPage isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} />
      <CustomRouter openedApps={urlsHandler}>
        <HomePage appName="Home" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
        <PhotosPage appName="Photos" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
        <FilesPage appName="Files" isLoggedIn={isLoggedIn} setIsLoggedIn={setIsLoggedIn} startAnimation={false} urlsHandler={urlsHandler} setUrlsHandler={setUrlsHandler} />
      </CustomRouter>
    </div>
  );
}
