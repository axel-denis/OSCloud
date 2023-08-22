import React from 'react';
import "./FilesApp.css"
import "../Window/Window.css"
import { transitionToUrl, UrlInfo } from '../UrlGestion';
import Banner from '../Banner/Banner';
import Window from '../Window/Window';
import { MobileDevice } from '../App';
import { AnimatePresence, motion } from 'framer-motion';
import { timeScale } from '../consts';


interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  urlsHandler: UrlInfo;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlInfo>>;
}

export default function FilesApp(props: Props) {
  const isMobile = React.useContext(MobileDevice);
  const [lpOpen, setLpOpen] = React.useState(!isMobile); // is left panel opened

  return (
    <Window
      appName={props.appName}
      urlsHandler={props.urlsHandler}
    >
      <Banner text="OSCloud:Files" onClick={() => {
        transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Home");
      }} />
      <AnimatePresence>
        <motion.div
          className='leftPannel'
          initial={{
            width: isMobile ? "80vw" : "450px",
          }}
          animate={{
            width: isMobile ? "80vw" : "450px",
            transition: {
              duration: timeScale * 0.25
            }
          }}
          exit={{
            width: 0,
            transition: {
              duration: timeScale * 0.25
            }
          }}
        >
          leftPannel
        </motion.div>
      </AnimatePresence>
      <div className='contentDiv'>
      </div>
    </Window>
  )
}
