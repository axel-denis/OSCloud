import React from 'react';
import "./PhotosPage.css"
import "../WindowAnimation.css"
import { Navigate } from "react-router-dom"
import HomePage from '../HomePage/HomePage';
import ProtectorOverlay from '../ProtectorOverlay/ProtectorOverlay';


interface Props {
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  startAnimation: boolean;
}
type AnimationStates = "intro" | "inter";

export default function PhotosPage(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>(props.startAnimation ? "intro" : "inter");
  const [redirect, setRedirect] = React.useState<boolean | "waiting">(false);
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);

  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
    if (props.isLoggedIn) {
      setAnimationState("inter")
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
      <div className={'photosAppBackground windowAnimation ' + animationState} style={{ zIndex: props.startAnimation ? 2 : "" }} >
        <div className="banner">
          <h1 className="h1HomePage" onClick={() => {
            setRedirect("waiting")
            setAnimationState("intro");
            setTimeout(() => {
              setRedirect(true);
            }, 1100)
          }}>OSCloud:Photos</h1>
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