import { FileInfo, FileType } from "./FilesApp"
import Folder from "../assets/folder.svg";
import TextFile from "../assets/textFile.svg";
import CodeFile from "../assets/codeFile.svg";
import BlendFile from "../assets/blendFile.svg";
import ImageFile from "../assets/imageFile.svg";
import VideoFile from "../assets/videoFile.svg";
import SoundFile from "../assets/soundFile.svg";
import UnknownFile from "../assets/unknownFile.svg";
import React from "react";
import { AnimatePresence, motion } from "framer-motion";
import BackBlur from "./BackBlur";
import { easeInOutCubic, timeScale } from "../consts";
import FileRightClick from "./FileRightClick";

export function selectFileIcon(type: FileType) {
  switch (type) {
    case "folder":
      return <img src={Folder} alt="Folder" className='iconImg' />;
    case "prog":
      return <img src={CodeFile} alt="programmation file" className='iconImg' />;
    case "text":
      return <img src={TextFile} alt="text file" className='iconImg' />;
    case "blend":
      return <img src={BlendFile} alt="Blender file" className='iconImg' />;
    case "image":
      return <img src={ImageFile} alt="Image" className='iconImg' />;
    case "video":
      return <img src={VideoFile} alt="Video" className='iconImg' />;
    case "sound":
      return <img src={SoundFile} alt="Sound" className='iconImg' />;
    default:
      return <img src={UnknownFile} alt="unknown file" className='iconImg' />;
    // TODO - compressed file type not made yet
  }
}

interface Props {
  file: FileInfo,
  size: number | string,
}

export default function SelectableFile(props: Props) {
  const [isRightClick, setIsRightClick] = React.useState(false);

  return (
    <>
      {isRightClick &&
        <div style={{ //place holder div when changing apparent location
          height: `${props.size}px`,
          width: `${props.size}px`,
        }} />
      }
      <AnimatePresence>
        {isRightClick &&
          <FileRightClick zIndex={3} />
        }
      </AnimatePresence>
      <motion.div
        className={isRightClick ? '' : 'fileDiv'}
        style={{
          height: isRightClick ? "20rem" : `${props.size}px`,
          width: isRightClick ? "20rem" : `${props.size}px`,
          position: isRightClick ? "fixed" : "relative",
          zIndex: isRightClick ? 4 : 1,
        }}
        layout
        transition={{
          duration: timeScale / 3,
          ease: easeInOutCubic,
          type: "spring"
        }}
        onClick={() => setIsRightClick(!isRightClick)}
      >
        {selectFileIcon(props.file.type)}
        <div className='fileName' style={{
          fontSize: "1.25rem", // TODO - adapt to size changes ?
        }}>
          {props.file.name}
        </div>
      </motion.div>
    </>
  )
}
