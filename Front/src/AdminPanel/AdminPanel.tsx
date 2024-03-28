import React from 'react';
import "./AdminPanel.css"
import "../Window/Window.css"
import { transitionToUrl, AnimationStates, getAnimationState, UrlInfo } from '../UrlGestion';
import Banner from '../Banner/Banner';
import Window from '../Window/Window';

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  urlsHandler: UrlInfo;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlInfo>>;
}

export default function AdminPanel(props: Props) {
  return (
    <Window>
      <Banner text="OSCloud:AdminPanel" onClick={() => {
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
