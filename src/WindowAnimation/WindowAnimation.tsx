import React, { ReactNode } from "react";
import { AnimationStates, getAnimationState, UrlInfo } from "../UrlGestion";
import "./WindowAnimation.css"
import { timeScale } from "../consts";

type Props = {
  appName: string;
  urlsHandler: UrlInfo;
  children: ReactNode;
}

export default function WindowAnimation(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>("off");
  const [animationTime, setAnimationTime] = React.useState<number>(0);
  const [animationDuration, setAnimationDuration] = React.useState<number>((500 * timeScale));


  React.useEffect(() => {
    let timeout: number;


    // IL FAUT ESSAYER DE METTRE UNE DUREE NEGATIVE AU ANIMATION DELAY (on utilisera animationduration parce que osef)
    if (props.urlsHandler.app !== props.appName && (animationState !== "off" && animationState !== "outro")) {
      if (animationTime > Date.now()) {
        setAnimationTime(animationTime + (500 * timeScale))
        setAnimationDuration((animationTime + (500 * timeScale)) - Date.now());
        console.log("animatio11111nduration :", (animationTime + (500 * timeScale * 2)) - Date.now());
      } else {
        setAnimationTime(Date.now() + (500 * timeScale));
        setAnimationDuration((500 * timeScale));
        console.log("animationduration :", 500);
      }
      setAnimationState("outro");
      timeout = setTimeout(() => {
        setAnimationState("off");
      }, timeScale * 500);
    }
    if (props.urlsHandler.app === props.appName && animationState !== "intro" && animationState !== "inter") {
      if (animationTime > Date.now()) {
        setAnimationTime(animationTime + (500 * timeScale))
        setAnimationDuration((animationTime + (500 * timeScale)) - Date.now());
        console.log("animat11111ionduration :", (animationTime + (500 * timeScale * 2)) - Date.now());
      } else {
        setAnimationTime(Date.now() + (500 * timeScale));
        setAnimationDuration((500 * timeScale));
        console.log("animationduration :", 500);
      }
      setAnimationState("intro");
      timeout = setTimeout(() => {
        setAnimationState("inter");
      }, timeScale * 500);
    }
    return () => {
      // clears timeout before running the new effect
      clearTimeout(timeout);
    };
  }, [props.urlsHandler]);

  if (animationState === "off") {
    return (<></>)
  }
  // let z_index = 0;
  // if (animationState === "intro") { z_index = 2; }
  // if (animationState === "inter") { z_index = 1; }
  // if (animationState === "outro") { z_index = 1; }

  console.log("aya", animationDuration)
  return (
    <div className={'photosAppBackground windowAnimation ' + animationState} style={{ /*zIndex: z_index,*/ animationDelay: animationDuration + "ms" }}>
      {props.children}
    </div>
  )
}