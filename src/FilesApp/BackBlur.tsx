import { AnimatePresence, circIn, circOut, easeInOut, easeOut, motion } from "framer-motion";
import { ReactNode } from "react";
import { timeScale } from "../consts";
import "./BackBlur.css";

interface Props {
  blur: string;
  zIndex: number;
  children?: ReactNode;
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
          WebkitBackdropFilter: "blur(0px)",
        }}
        animate={{
          backdropFilter: `blur(${props.blur})`,
          WebkitBackdropFilter: `blur(${props.blur})`,
          transition: {
            duration: timeScale / 3,
          }
        }}
        exit={{
          backdropFilter: "blur(0px)",
          WebkitBackdropFilter: "blur(0px)",
          transition: {
            duration: timeScale / 3,
            ease: circOut,
          }
        }}
      >
        {props.children}
      </motion.div>
  )
}
