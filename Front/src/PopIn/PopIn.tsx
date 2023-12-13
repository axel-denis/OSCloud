import { ReactNode } from "react";
import "./PopIn.css";
import { motion, AnimatePresence } from "framer-motion";
import BackBlur from "../FilesApp/BackBlur";
import { timeScale } from "../consts";

interface Props {
  zIndex: number;
  children: ReactNode
}

export default function PopIn(props: Props) {
  return (
    <AnimatePresence>
      <BackBlur zIndex={5} blur="15px">
        <motion.div className="popIn"
          initial={{
            marginTop: "100vh",
          }}
          animate={{
            marginTop: "0vh",
            transition: {
              duration: timeScale / 3,
            }
          }}
          exit={{
            marginTop: "100vh",
            transition: {
              duration: timeScale / 3,
            }
          }}
        >
          {props.children}
        </motion.div>
      </BackBlur>
    </AnimatePresence>
  )
}