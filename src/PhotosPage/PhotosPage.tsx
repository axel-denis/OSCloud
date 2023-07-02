import React from 'react';
import "./PhotosPage.css"
import "../WindowAnimation/WindowAnimation.css"
import { transitionToUrl, AnimationStates, getAnimationState } from '../UrlGestion';
import Banner from '../Banner/Banner';
import WindowAnimation from '../WindowAnimation/WindowAnimation';

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
  return (
    <>
      <WindowAnimation appName={props.appName} urlsHandler={props.urlsHandler}>
        <Banner text="OSCloud:Photos" onClick={() => {
          transitionToUrl(props.urlsHandler, props.setUrlsHandler, "Home");
        }} />
        <div className='leftPannel'>
          leftPannel
        </div>
        <div className='contentDiv'>
        </div>
      </WindowAnimation>
    </>
  )
}