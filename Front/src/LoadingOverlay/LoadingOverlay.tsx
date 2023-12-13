import "./LoadingOverlay.css"
import "../LoginPage/LoginPage.css"
import loadingIcon from "../icons/loading.svg"
import { AnimatePresence, motion } from 'framer-motion';
import { timeScale } from "../consts";

export default function LoadingOverlay() {
  console.log("loading overlay")
  return (
    <motion.div className="loadingOverlay"
      initial={{
        top: "-10rem",
      }}
      animate={{
        top: "100px",
        transition: {
          duration: timeScale,
        }
      }}
      exit={{
        top: "-10rem",
        transition: {
          duration: timeScale,
        }
      }}
    >
      <img src={loadingIcon} alt="loading" className='loadingRotation' />
      <span className="loadingOverlayText">Loading App...</span>
    </motion.div>
  )
}