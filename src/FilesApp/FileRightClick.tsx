import BackBlur from "./BackBlur";
import { motion } from "framer-motion";
import { timeScale } from "../consts";
import "./FileRightClick.css"
import FileProperties from "./FileProperties";
import FileRightClickActions from "./FileRightClickActions";

interface Props {
  zIndex: number;
}

export default function FileRightClick(props: Props) {
  return (
    <BackBlur blur={"20px"} zIndex={props.zIndex}>
      <motion.div className="rightClickLayoutDiv"
        style={{flex: 2}}
        initial={{
          translateX: "10vw",
          opacity: 0,
        }}
        animate={{
          translateX: "0vw",
          opacity: 1,
          transition: {
            duration: timeScale / 1,
            type: "spring",
          }
        }}
        exit={{
          translateX: "10vw",
          opacity: 0,
          transition: {
            duration: timeScale / 3,
            type: "spring",
          }
        }}
      >
        {/* div vide pour mettre le reste à droite */}
      </motion.div>
      <motion.div className="rightClickLayoutDiv"
        style={{flex: 2}}
        initial={{
          translateX: "10vw",
          opacity: 0,
        }}
        animate={{
          translateX: "0vw",
          opacity: 1,
          transition: {
            duration: timeScale / 1,
            type: "spring",
          }
        }}
        exit={{
          translateX: "10vw",
          opacity: 0,
          transition: {
            duration: timeScale / 3,
            type: "spring",
          }
        }}
      >
        <FileRightClickActions file={1} />
      </motion.div>
      <motion.div className="rightClickLayoutDiv"
        style={{flex: 4}}
        initial={{
          translateX: "10vw",
          opacity: 0,
        }}
        animate={{
          translateX: "0vw",
          opacity: 1,
          transition: {
            duration: timeScale / 1,
            type: "spring",
          }
        }}
        exit={{
          translateX: "10vw",
          opacity: 0,
          transition: {
            duration: timeScale / 3,
            type: "spring",
          }
        }}
      >
        <FileProperties file_id={1}/>
        <div className="filePropertiesLayout">
          <div style={{color: "#aaa", width: "100%", height: "100%", justifyContent: "center",display: "flex", placeItems: "center"}}>
            select an action first
          </div>
        </div>
      </motion.div>
    </BackBlur>
  )
}
