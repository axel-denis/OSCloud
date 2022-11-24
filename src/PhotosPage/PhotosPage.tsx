import React from 'react';
import "./PhotosPage.css"
import { Navigate } from "react-router-dom"
import HomePage from '../HomePage/HomePage';
import un from "../testPhotos/1.png";
import deux from "../testPhotos/2.png";


interface Props {
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  startAnimation: boolean;
}
type AnimationStates = "intro" | "inter" | "outro" | "hidden"

export default function PhotosPage(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>(props.startAnimation ? "intro" : "inter");
  const [redirect, setRedirect] = React.useState<boolean | "waiting">(false);

  React.useEffect(() => {
    console.log("photos anim here");
    if (props.startAnimation) {
      console.log("photos anim VALID");
      setAnimationState("intro");
      setAnimationState("inter");
    }
  }, [props.startAnimation])

  return (
    <>
      {redirect === "waiting" ? <HomePage isLoggedIn={props.isLoggedIn} setIsLoggedIn={props.setIsLoggedIn} /> : null}
      {redirect === true ? <Navigate to="/" /> : null}
      <div className={'mainDivAnim ' + animationState} style={{zIndex: props.startAnimation ? 2 : ""}} >
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