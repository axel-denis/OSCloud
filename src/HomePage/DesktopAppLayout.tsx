import React from "react";

interface Props {
  launchPhotosPage: Function,
  launchFilesPage: Function
}

export default function DesktopAppLayout(props: Props) {
  const hoverAnimationDuration = 400;
  // const [hoverAnimationDuration, setHoverAnimationDuration] = React.useState(0);
  // React.useEffect(() => { // protection against hover breaking while ending an app transition
  //   setTimeout(() => {
  //     setHoverAnimationDuration(400);
  //   }, 10);
  // })
  // NOTE - was used to prevent hover from break on transition, see commit 318f0a8

  return (
    <>
      <div className="appsWidget" style={{transition: "transform " + String(hoverAnimationDuration) + "ms"}}>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchFilesPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="appsWidget" style={{transition: "transform " + String(hoverAnimationDuration) + "ms"}}>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="appsWidget" style={{transition: "transform " + String(hoverAnimationDuration) + "ms"}}>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="appsWidget" style={{transition: "transform " + String(hoverAnimationDuration) + "ms"}}>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="doubleWidget" style={{transition: "transform " + String(hoverAnimationDuration) + "ms"}}>
        double
      </div>
      <div className="doubleWidget" style={{transition: "transform " + String(hoverAnimationDuration) + "ms"}}>
        double
      </div>
      <div className="appsWidget" style={{transition: "transform " + String(hoverAnimationDuration) + "ms"}}>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
    </>
  )
}