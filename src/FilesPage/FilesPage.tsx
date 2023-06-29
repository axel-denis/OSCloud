import React from 'react';
import "./FilesPage.css"
import "../WindowAnimation.css"
import { Navigate } from "react-router-dom"
import HomePage from '../HomePage/HomePage';
import ProtectorOverlay from '../ProtectorOverlay/ProtectorOverlay';
import { transitionToUrl, UrlsHandler } from '../UrlGestion';

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  startAnimation: boolean;
  urlsHandler: UrlsHandler;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>;
}
type AnimationStates = "intro" | "inter";

export default function FilesPage(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>(props.startAnimation ? "intro" : "inter");
  const [redirect, setRedirect] = React.useState<boolean | "waiting">(false);
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);

  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
    if (props.isLoggedIn) {
      setAnimationState("inter");
    }
  }, [props.isLoggedIn])

  React.useEffect(() => {
    if (props.startAnimation) {
      setAnimationState("inter");
    }
  }, [props.startAnimation])

  return (
    <>
      {redirect !== false ? <ProtectorOverlay /> : null}
      {redirect === "waiting" ? <HomePage isLoggedIn={props.isLoggedIn} setIsLoggedIn={props.setIsLoggedIn} /> : null}
      {redirect === true ? <Navigate to="/" /> : null}

      <div className={'FilesAppBackground windowAnimation ' + animationState} style={{ zIndex: props.startAnimation ? 2 : "" }} >
        <div className="banner">
          <h1 className="h1HomePage" onClick={() => {
            setRedirect("waiting")
            setAnimationState("intro");
            setTimeout(() => {
              setRedirect(true);
            }, 1100)
          }}>OSCloud:Files</h1>
        </div>
        <div className='leftPannel'>
          leftPannel
        </div>
        <div className='contentDiv'>
        </div>
      </div>
    </>
  )
}