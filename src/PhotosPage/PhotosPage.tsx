import React from 'react';
import "./PhotosPage.css"
import "../WindowAnimation.css"
import { transitionToUrl, UrlsHandler } from '../UrlGestion';

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  startAnimation: boolean;
  urlsHandler: UrlsHandler;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>;
}
type AnimationStates = "intro" | "inter" | "outro";

export default function PhotosPage(props: Props) {
  const [isMobile, setIsMobile] = React.useState(window.matchMedia("(max-width: 34.5rem)").matches);

  React.useEffect(() => {
    window.addEventListener("resize", () => setIsMobile(window.matchMedia("(max-width: 34.5rem)").matches));
  }, [props.isLoggedIn])

  return (
    <>
      <div className={'photosAppBackground windowAnimation ' + 'inter'} style={{ zIndex: props.startAnimation ? 2 : "" }} >
        <div className="banner">
          <h1 className="h1HomePage" onClick={() => {
            transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Home", 1100);
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