import React from "react";
import { Navigate } from "react-router-dom";
import DesktopAppLayout from "./DesktopAppLayout";
import PhotosPage from "../PhotosPage/PhotosPage";
import "./HomePage.css"
import MobileAppLayout from "./MobileAppLayout";
import FilesPage from "../FilesPage/FilesPage";

interface Props {
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
}
type AnimationStates = "intro" | "inter" | "outro";

export default function HomePage(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>("intro");
  const [photosPageStartAnimation, setPhotosPageStartAnimation] = React.useState<boolean | "redirect">(false);
  const [filesPageStartAnimation, setFilesPageStartAnimation] = React.useState<boolean | "redirect">(false);
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);

  function launchPhotosPage() {
    setPhotosPageStartAnimation(true);
    setTimeout(() => {
      setPhotosPageStartAnimation("redirect");
    }, 1000);
  }
  function launchFilesPage() {
    setFilesPageStartAnimation(true);
    setTimeout(() => {
      setFilesPageStartAnimation("redirect");
    }, 1000);
  }

  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
    console.log("useeffect on isloggedin");
    if (props.isLoggedIn) {
      setAnimationState("inter")
      console.log("activated on isloggedin");
    }
    //return(window.removeEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches)))
  }, [props.isLoggedIn])

  console.log("is mobile :", isMobile);
  return (
    <>
      {photosPageStartAnimation === true ? <PhotosPage isLoggedIn={props.isLoggedIn} setIsLoggedIn={props.setIsLoggedIn} startAnimation={true} /> : null}
      {photosPageStartAnimation === "redirect" ? <Navigate to="/photos" /> : null}

      {filesPageStartAnimation === true ? <FilesPage isLoggedIn={props.isLoggedIn} setIsLoggedIn={props.setIsLoggedIn} startAnimation={true} /> : null}
      {filesPageStartAnimation === "redirect" ? <Navigate to="/files" /> : null}

      <div className="banner">
        <h1 className="h1HomePage" onClick={() => {
          props.setIsLoggedIn(false);
          setAnimationState("outro");
          setTimeout(() => setAnimationState("intro"), 1100)
        }}>OSCloud</h1>
      </div>
      <div className="centerContent">
        <div className={"widgetAera " + animationState}>
          {isMobile === false ? <DesktopAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
          {isMobile === true ? <MobileAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
        </div>
      </div>
    </>
  )
}