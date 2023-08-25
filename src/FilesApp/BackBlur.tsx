import { AnimatePresence, circIn, circOut, easeInOut, easeOut, motion } from "framer-motion";
import { timeScale } from "../consts";
import "./BackBlur.css";

interface Props {
  blur: string;
  zIndex: number;
}

export default function BackBlur(props: Props) {
  return (
      <motion.div
        className="backBlur"
        style={{
          zIndex: props.zIndex,
        }}
        initial={{
          backdropFilter: "blur(0px)",
        }}
        animate={{
          backdropFilter: "blur(20px)",
          transition: {
            duration: timeScale / 2,
          }
        }}
        exit={{
          backdropFilter: "blur(0px)",
          transition: {
            duration: timeScale / 2,
            ease: circOut,
          }
        }}
      />
  )
}
