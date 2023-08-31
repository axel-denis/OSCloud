import "./FileRightClickActions.css";
import { motion } from "framer-motion";
import { timeScale } from "../consts";
interface Props {
  file: number;
}

export default function FileRightClickActions(props: Props) {
  return (
    <div className="fileRightClickActionsBody">
      <motion.h2 className="fileRightClickActionsBanner"
        initial={{
          backdropFilter: "blur(15px)",
          backgroundColor: "rgba(0,0,0,0)"
        }}
        animate={{
          backdropFilter: "blur(15px)",
          backgroundColor: "rgba(0,0,0,0.2)",
          transition: {
            duration: timeScale / 4,
          }
        }}
        exit={{
          backdropFilter: "blur(15px)",
          backgroundColor: "rgba(0,0,0,0)",
          transition: {
            duration: timeScale / 4,
          }
        }}
      >
        Actions
      </motion.h2>
      <ul className="fileRightClickActionsList">
        <li className="fileRightClickActionsLine">Rename</li>
        <li className="fileRightClickActionsLine">Delete</li>
        <li className="fileRightClickActionsLine">Copy</li>
        <li className="fileRightClickActionsLine">Move</li>
        <li className="fileRightClickActionsLine">Change Tags</li>
        <li className="fileRightClickActionsLine">Fast Share</li>
        <li className="fileRightClickActionsLine">Advanced Share</li>
        <li className="fileRightClickActionsLine">Open with...</li>
        <li className="fileRightClickActionsLine"></li>
        <li className="fileRightClickActionsLine"></li>
      </ul>
    </div>
  )
}
