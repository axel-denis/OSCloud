import React, { ReactNode } from "react";
import { AnimationStates, getAnimationState, UrlInfo } from "../UrlGestion";
import "./Window.css"
import { timeScale } from "../consts";
import { motion, AnimatePresence } from "framer-motion";
import { MobileDevice } from "../App";

type Props = {
  appName: string;
  urlsHandler: UrlInfo;
  children: ReactNode;
}

export default function Window(props: Props) {
  const [isOpen, setIsOpen] = React.useState<boolean>(false);
  const isMobile = React.useContext(MobileDevice);

  React.useEffect(() => {
    setIsOpen(props.urlsHandler.app === props.appName);
  }, [props.urlsHandler]);

  const variants = {
    open: {
      top: 0,
      opacity: 1,
      borderRadius: 0,
      transform: isMobile ? "" : "perspective(500px) rotateX(0deg) scale(1.0)",
      transition: {
        duration: .75 * timeScale,
        ease: "easeOut"
      }
    },
    closed: {
      top: "100vh",
      opacity: 0,
      borderRadius: isMobile ? 0 : "6rem", // desktop only
      transform: isMobile ? "" : "perspective(1000px) rotateX(-20deg) scale(0.3)",
      transition: {
        duration: .75 * timeScale,
        ease: "easeIn"
      }
    }
  }

  return (
    <motion.div
      className={'appBackground windowAnimation'}
      initial={variants.closed}
      animate={variants.open}
      exit={variants.closed}
    >
      <div className='windowInnerDiv'>
        {props.children}
      </div>
    </motion.div>
  )
}
