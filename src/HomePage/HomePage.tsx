import React from "react";
import DesktopAppLayout from "./DesktopAppLayout";
import "./HomePage.css"
import MobileAppLayout from "./MobileAppLayout";
import { UrlsHandler, transitionToUrl } from "../UrlGestion";
import Banner from "../Banner/Banner";

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  urlsHandler: UrlsHandler;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>;
}
type AnimationStates = "intro" | "inter" | "outro"; // virer outro ?

export default function HomePage(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>("intro");
  const [photosPageStartAnimation, setPhotosPageStartAnimation] = React.useState<boolean | "redirect">(false);
  const [filesPageStartAnimation, setFilesPageStartAnimation] = React.useState<boolean | "redirect">(false);
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);

  function launchPhotosPage() {
    transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Photos");
  }
  function launchFilesPage() {
    transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Files");
  }
  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
    if (props.isLoggedIn) {
      setAnimationState("inter")
    }
    //return(window.removeEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches)))
  }, [props.isLoggedIn])

  return (
    <>
      <Banner text="OSCloud" onClick={() => {
        props.setIsLoggedIn(false);
        setAnimationState("outro");
        setTimeout(() => setAnimationState("intro"), 1100)
      }}></Banner>
      <div className="centerContent">
        <div className={"widgetAera " + animationState}>
          {isMobile === false ? <DesktopAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
          {isMobile === true ? <MobileAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
        </div>
      </div>
    </>
  )
}