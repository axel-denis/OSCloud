import React from "react";
import { Navigate } from "react-router-dom";
import DesktopAppLayout from "./DesktopAppLayout";
import PhotosPage from "../PhotosPage/PhotosPage";
import "./HomePage.css"
import MobileAppLayout from "./MobileAppLayout";

interface Props {
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
}
type AnimationStates = "intro" | "inter" | "outro" | "hidden"

export default function HomePage(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>("intro");
  const [photosPageStartAnimation, setPhotosPageStartAnimation] = React.useState<boolean | "redirect">(false);
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);

  function launchPhotosPage() {
    setPhotosPageStartAnimation(true);
    setTimeout(() => {
      setPhotosPageStartAnimation("redirect");
    }, 1000);
  }

  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
    console.log("useeffect on isloggedin");
    if (props.isLoggedIn) {
      setAnimationState("inter")
      console.log("activated on isloggedin");
    }
  }, [props.isLoggedIn])
  React.useEffect(() => {
    if (props.isLoggedIn) {
      setAnimationState("inter")
    }
  }, [])
  console.log("is mobile :", isMobile);
  return (
    <>
      {photosPageStartAnimation === true ? <PhotosPage isLoggedIn={props.isLoggedIn} setIsLoggedIn={props.setIsLoggedIn} startAnimation={true} /> : null}
      {photosPageStartAnimation === "redirect" ? <Navigate to="/photos" /> : null}
      <div className="banner">
        <h1 className="h1HomePage" onClick={() => {
          props.setIsLoggedIn(false);
          setAnimationState("outro");
          setTimeout(() => setAnimationState("intro"), 1100)
        }}>OSCloud</h1>
      </div>
      <div className="centerContent">
        <div className={"widgetAera " + animationState}>
          {isMobile === false ? <DesktopAppLayout launchPhotosPage={launchPhotosPage}/> : null}
          {isMobile === true ? <MobileAppLayout launchPhotosPage={launchPhotosPage}/> : null}
        </div>
      </div>
    </>
  )
}