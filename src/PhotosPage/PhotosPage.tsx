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
  // const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);
  const [animationState, setAnimationState] = React.useState<AnimationStates>()

  React.useEffect(() => {
    console.log("will set state to", getAnimationState(props.urlsHandler, props.appName))
    setAnimationState(getAnimationState(props.urlsHandler, props.appName));// getAnimationState(props.urlsHandler, props.appName);
  }, [props.urlsHandler]);

  // React.useEffect(() => {
  //   window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
  // }, [props.isLoggedIn]);

  if (animationState === undefined || animationState === "off") {
    return (<></>)
  }
  let z_index = 0;
  if (animationState === "intro") { z_index = 2 }
  if (animationState === "inter") { z_index = 1 }
  if (animationState === "outro") { z_index = 1 }
  return (
    <>
      <div className={'photosAppBackground windowAnimation ' + animationState} style={{ zIndex: z_index }}>
        <Banner text="OSCloud:Photos" onClick={() => {
          transitionToUrl(props.urlsHandler, props.setUrlsHandler, "Home");
        }} />
        <div className='leftPannel'>
          leftPannel
          <button onClick={() => {
            setAnimationState("outro");
            setTimeout(() => {
              setAnimationState("intro");
            }, 300);
          }}>test</button>
        </div>
        <div className='contentDiv'>
        </div>
      </div>
    </>
  )
}