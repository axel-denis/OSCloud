import React from 'react';
import "./FilesApp.css"
import "../Window/Window.css"
import { transitionToUrl, UrlInfo } from '../UrlGestion';
import Banner from '../Banner/Banner';
import Window from '../Window/Window';
import { MobileDevice } from '../App';
import { AnimatePresence, motion } from 'framer-motion';
import { backIp, timeScale } from '../consts';
import SelectableFile from './SelectableFile';


interface Props {
  appName: string;
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
  urlsHandler: UrlInfo;
  setUrlsHandler: React.Dispatch<React.SetStateAction<UrlInfo>>;
}

export type FileType =
  "folder" |
  "unknown" |
  "prog" |
  "text" |
  "blend" |
  "image" |
  "video" |
  "sound" |
  "compressed";

export interface FileInfo {
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
    name: "file1_with_long_name_and_unknown_type.wtf",
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
        return <SelectableFile file={file} size={props.size} />
      })}
    </div>
  )
}


export default function FilesApp(props: Props) {
  const isMobile = React.useContext(MobileDevice); // FIXME - pas encore mis la condition
  const [lpOpen, setLpOpen] = React.useState(isMobile); // is left panel opened
  const [sliderValue, setSliderValue] = React.useState(177); //FIXME - just for testing

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
            <input type="range" min={50} max={500} value={sliderValue} onChange={
              (event) => { setSliderValue(+event.target.value) }
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
