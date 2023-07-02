import React, { ReactNode } from "react";
import { AnimationStates, getAnimationState,  UrlInfo } from "../UrlGestion";
import "./WindowAnimation.css"
import { timeScale } from "../consts";

type Props = {
  appName: string;
  urlsHandler: UrlInfo[];
  children: ReactNode;
}

export default function WindowAnimation(props: Props) {
  const [animationState, setAnimationState] = React.useState<AnimationStates>();
  const [animationTime, setAnimationTime] = React.useState<number>(0);
  const [animationDuration, setAnimationDuration] = React.useState<number>((500 * timeScale));


  React.useEffect(() => {
    console.log("will set state to", getAnimationState(props.urlsHandler, props.appName))
    setAnimationState(getAnimationState(props.urlsHandler, props.appName));
    const actualTime = Date.now();
    setAnimationDuration(((actualTime - animationTime) >= (500 * timeScale) ? (500 * timeScale) : ((500 * timeScale) + actualTime - animationTime)));
    setAnimationTime(actualTime);
    console.log("animation will be", animationDuration);
  }, [props.urlsHandler]);

  if (animationState === undefined || animationState === "off") {
    return (<></>)
  }
  // let z_index = 0;
  // if (animationState === "intro") { z_index = 2; }
  // if (animationState === "inter") { z_index = 1; }
  // if (animationState === "outro") { z_index = 1; }

  return (
    <div className={'photosAppBackground windowAnimation ' + animationState} style={{ /*zIndex: z_index,*/ animationDuration: (500 * timeScale) + "ms"}}>
      {props.children}
    </div>
  )
}