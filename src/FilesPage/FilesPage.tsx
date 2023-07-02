import React from 'react';
import "./FilesPage.css"
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

export default function FilesPage(props: Props) {
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);
  const [animationState, setAnimationState] = React.useState<AnimationStates>()

  React.useEffect(() => {
    setAnimationState(getAnimationState(props.urlsHandler, props.appName));// getAnimationState(props.urlsHandler, props.appName);
  }, [props.urlsHandler]);

  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
  }, [props.isLoggedIn])

  if (animationState === undefined) {
    return (<></>)
  }
  return (
    <>
      <div className={'FilesAppBackground windowAnimation ' + animationState} >
        <Banner text="OSCloud:Files" onClick={() => {
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