import React from 'react';
import Folder from "../assets/folder.svg";
import TextFile from "../assets/textFile.svg";
import CodeFile from "../assets/codeFile.svg";
import BlendFile from "../assets/blendFile.svg";
import ImageFile from "../assets/imageFile.svg";
import VideoFile from "../assets/videoFile.svg";
import SoundFile from "../assets/soundFile.svg";
import UnknownFile from "../assets/unknownFile.svg";
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

type FileType =
  "folder" |
  "unknown"|
  "prog"   |
  "text"   |
  "blend"  |
  "image"  |
  "video"  |
  "sound"  |
  "compressed";

interface FileInfo {
  name: string;
  type: FileType;
  size: number;
  rights: "read" | "write";
}

const sample_data: FileInfo[] = [
  {
    name: "folder1",
    type: "folder",
    size: 1024,
    rights: "write",
  },
  {
    name: "file1.js",
    type: "prog",
    size: 256,
    rights: "write",
  },
  {
    name: "nowrite.txt",
    type: "text",
    size: 42,
    rights: "read",
  },
  {
    name: "project.blend",
    type: "blend",
    size: 1024,
    rights: "write",
  },
  {
    name: "photo.png",
    type: "image",
    size: 256,
    rights: "write",
  },
  {
    name: "video.mp4",
    type: "video",
    size: 42,
    rights: "read",
  },
  {
    name: "music.mp3",
    type: "sound",
    size: 1024,
    rights: "write",
  },
  {
    name: "file1.wtf",
    type: "unknown",
    size: 256,
    rights: "write",
  },
  {
    name: "code.rs",
    type: "prog",
    size: 42,
    rights: "read",
  },
]

function selectFileIcon(info: FileInfo) {
  switch (info.type) {
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
      return <img src={UnknownFile} alt="Sound" className='iconImg' />;
    // TODO - compressed file type not made yet
  }
}

interface DisplayProps {
  files: FileInfo[],
  size: number | string,
}

function MosaicDisplay(props: DisplayProps) {
  return (
    <div className='filesGrid'
      style={{
        gridTemplateColumns: `repeat(auto-fill, ${props.size}px)`
      }}
    >
      {props.files.map((file: FileInfo) => {
        return <div style={{
          width: "100%",
          height: `${props.size}px`,
          borderRadius: "1rem"
        }}>
          {selectFileIcon(file)}
        </div>
      })}
    </div>
  )
}


export default function FilesApp(props: Props) {
  const isMobile = React.useContext(MobileDevice); // FIXME - pas encore mis la condition
  const [lpOpen, setLpOpen] = React.useState(isMobile); // is left panel opened
  const [sliderValue, setSliderValue] = React.useState("100"); //FIXME - just for testing

  return (
    <Window
      appName={props.appName}
      urlsHandler={props.urlsHandler}
    >
      <Banner
        text="OSCloud:Files"
        onClick={() => {
          transitionToUrl(props.urlsHandler, props.setUrlsHandler, "/Home");
        }}
        leftChildren={
          <button
            style={{ marginLeft: "1rem", height: "3rem" }}
            onClick={() => { setLpOpen(!lpOpen) }}
          >menu</button>
        }
        rightChildren={
          <>
            <input type="range" min="50" max="500" value={sliderValue} onChange={
              (event) => { setSliderValue(event.target.value) }
            }
              style={{
                height: ".5rem",
                backgroundColor: "blue",
                width: "50%",
                marginLeft: "17.5%"
              }}
            />
            {sliderValue}
          </>
        }
      />
      <AnimatePresence>
        {lpOpen &&
          <motion.div
            className='leftPannel'
            style={{
              width: isMobile ? "80vw" : "450px",
            }}
            initial={{
              left: isMobile ? "-80vw" : "-450px",
            }}
            animate={{
              left: 0,
              transition: {
                duration: timeScale * 0.25
              }
            }}
            exit={{
              left: isMobile ? "-80vw" : "-450px",
              transition: {
                duration: timeScale * 0.25
              }
            }}
          >
            leftPannel
          </motion.div>
        }
      </AnimatePresence>
      <div className='contentDiv'>
        <MosaicDisplay files={sample_data} size={sliderValue} />
      </div>
    </Window>
  )
}
