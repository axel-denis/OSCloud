import React, { useContext } from "react";
import DesktopAppLayout from "./DesktopAppLayout";
import "./HomePage.css"
import MobileAppLayout from "./MobileAppLayout";
import { UrlsHandler, transitionToUrl } from "../UrlGestion";
import Banner from "../Banner/Banner";
import { MobileDevice } from "../App";
import { AnimatePresence, motion } from 'framer-motion';
import { timeScale } from "../consts";

interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  urlsHandler: UrlsHandler;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlsHandler>>;
}

export default function HomePage(props: Props) {
  const isMobile = React.useContext(MobileDevice);
  const [isOpen, setIsOpen] = React.useState<boolean>(false);

  React.useEffect(() => {
    setIsOpen(props.urlsHandler.app === props.appName);
  }, [props.urlsHandler]);

  function launchPhotosPage() {
    transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Photos");
  }
  function launchFilesPage() {
    transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Files");
  }

  // React.useEffect(() => {
  //   console.log("mounted");
  //   return (() => { console.log("unmounted ?") })
  // }, [])

  function easeOutCirc(x: number): number { // https://easings.net/#easeOutCirc
    return Math.sqrt(1 - Math.pow(x - 1, 2));
  }

  return (
    <AnimatePresence>
      {isOpen && props.isLoggedIn &&
        <>
          <Banner text="OSCloud" onClick={() => {
            props.setIsLoggedIn(false);
          }} />
          <div className="centerContent">
            <motion.div
              className={"widgetAera"}
              initial={{
                opacity: isMobile ? 1 : 0,
                paddingTop: isMobile ? "100vh" : "0vh",
                transform: isMobile ? "" : "perspective(500px) rotateX(8deg) rotateY(8deg) scale(1.05) rotate(-1deg)",
              }}
              animate={{
                opacity: 1,
                transform: isMobile ? "" : "perspective(1000px) rotateX(0deg) rotateY(0deg) scale(1.0) rotate(0deg)",
                paddingTop: "0vh",
                transition: {
                  duration: timeScale * (isMobile ? 0.5 : 1),
                  delay: timeScale * 0.25,
                  ease: easeOutCirc
                }
              }}
              exit={{
                opacity: isMobile ? 1 : 0,
                transform: isMobile ? "" : "perspective(500px) rotateX(-20deg) rotateY(-20deg) scale(0.7) rotate(1deg)",
                paddingTop: isMobile ? "100vh" : "0vh",
                transition: {
                  duration: timeScale * (isMobile ? 0.5 : 1),
                  ease: "easeIn"
                }
              }}
            >
              {isMobile === false ? <DesktopAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
              {isMobile === true ? <MobileAppLayout launchPhotosPage={launchPhotosPage} launchFilesPage={launchFilesPage} /> : null}
            </motion.div>
          </div>
        </>
      }
    </AnimatePresence>
  )
}
