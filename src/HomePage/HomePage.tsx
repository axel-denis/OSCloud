import React, { useContext } from "react";
import DesktopAppLayout from "./DesktopAppLayout";
import "./HomePage.css"
import MobileAppLayout from "./MobileAppLayout";
import { UrlsHandler, transitionToUrl } from "../UrlGestion";
import Banner from "../Banner/Banner";
import { MobileDevice } from "../App";

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  urlsHandler: UrlsHandler;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>;
}
type AnimationStates = "intro" | "inter" | "outro"; // virer outro ?

export default function HomePage(props: Props) {
  const isMobile = React.useContext(MobileDevice);

  function launchPhotosPage() {
    transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Photos");
  }
  function launchFilesPage() {
    transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Files");
  }

  React.useEffect(() => {
    console.log("mounted");
    return (() => {console.log("unmounted ?")})
  }, [])
  React.useEffect(() => {
    // animation ?
  }, [props.isLoggedIn])

  return (
    <>
      <Banner text="OSCloud" onClick={() => {
        props.setIsLoggedIn(false);
      }}></Banner>
      <div className="centerContent">
        <div className={"widgetAera"}>
          {isMobile === false ? <DesktopAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
          {isMobile === true ? <MobileAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
        </div>
      </div>
    </>
  )
}