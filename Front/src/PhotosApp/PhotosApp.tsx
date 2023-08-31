import React from 'react';
import "./PhotosApp.css"
import "../Window/Window.css"
import { transitionToUrl, AnimationStates, getAnimationState, UrlInfo } from '../UrlGestion';
import Banner from '../Banner/Banner';
import Window from '../Window/Window';

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  startAnimation: boolean;
  urlsHandler: UrlInfo;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlInfo>>;
}

export default function PhotosApp(props: Props) {
  return (
    <Window
      appName={props.appName}
      urlsHandler={props.urlsHandler}
    >
      <Banner text="OSCloud:Photos" onClick={() => {
        transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Home");
      }} />
      <div className='leftPannel'>
        leftPannel
      </div>
      <div className='contentDiv'>
      </div>
    </Window>
  )
}
