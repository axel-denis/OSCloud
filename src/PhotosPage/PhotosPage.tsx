import React from 'react';
import "./PhotosPage.css"
import "../WindowAnimation.css"
import { transitionToUrl, UrlsHandler, AnimationStates, getAnimationState } from '../UrlGestion';
import Banner from '../Banner/Banner';

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  startAnimation: boolean;
  urlsHandler: UrlsHandler;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>;
}

export default function PhotosPage(props: Props) {
  const [animationLock, setAnimationLock] = React.useState<boolean>(false);
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);
  const [animationState, setAnimationState] = React.useState<AnimationStates>()

  React.useEffect(() => {
    //setAnimationState(getAnimationState(props.urlsHandler, props.appName));// getAnimationState(props.urlsHandler, props.appName);
    if (animationLock === false) { // prevent from changing it's mind during animation
      console.log("changing animation state...", (props as any).shouldKey, animationLock)
      setAnimationState(getAnimationState(props.urlsHandler, props.appName));// getAnimationState(props.urlsHandler, props.appName);
      setAnimationLock(true);
      console.log("locked")
      setTimeout(() => {
        setAnimationLock(false);
        console.log("unlocked")
      }, 600);
    }
  }, [props.urlsHandler]);

  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
  }, [props.isLoggedIn]);

  if (animationState === undefined) {
    return (<></>)
  }
  return (
    <>
      <div className={'photosAppBackground windowAnimation ' + animationState}>
        <Banner text="OSCloud:Photos" onClick={() => {
          transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Home", 500);
        }} />
        <div className='leftPannel'>
          leftPannel
        </div>
        <div className='contentDiv'>
        </div>
      </div>
    </>
  )
}