import React, { ReactElement, ReactNode, cloneElement } from "react"
import { UrlsHandler } from "../UrlGestion";
import HomePage from "../HomePage/HomePage";
import { v4 as uuidv4 } from 'uuid';

interface Props {
  openedApps: UrlsHandler;
  children: ReactNode;
}

export default function CustomRouter(props: Props) {
  /* NOTE
    In this function, Home is handled differently because it always need to be on the
    background, and, to prevent hover animation to break (see commit 318f0a8), must not be
    reloaded by react. So it's set in it's own variable to prevent reloadings, and this
    variable is rendered first to be on the background.

    An other solutions was used on the commit 318f0a8, that was to disable animations durations
    on the first 10ms of the component, and to just order the render array to set Home at [0]
  */
  const [render, setRender] = React.useState<[ReactNode | null, ReactNode | null, ReactNode | null]>([null, null, null]);
  const [render1, setRender1] = React.useState<ReactNode>();
  const [render2, setRender2] = React.useState<ReactNode>();
  const [render3, setRender3] = React.useState<ReactNode>();
  const [displayHome, setDisplayHome] = React.useState<boolean>(true);
  const [home, setHome] = React.useState<ReactNode>()

  React.useEffect(() => {
    React.Children.forEach(props.children, (child: ReactNode) => {
      if (React.isValidElement(child) && child.props) {
        if (child.props.appName === "Home") {
          setHome(child);
        }
      }
    });
  })
  // selection of the one or two children to render :
  React.useEffect(() => {
    console.log("useffect on", props.openedApps);
    let temp: [ReactNode | null, ReactNode | null, ReactNode | null] = [null,null,null]
    if (props.openedApps.nextUrl.app === "Home" || props.openedApps.actualUrl.app === "Home") {
      setDisplayHome(true);
    } else {
      setDisplayHome(false)
    }
    if (render3 !== null && props.openedApps.afterUrl.app === "") {
      setRender3(null);
    }
    if (render2 !== null && props.openedApps.nextUrl.app === "") {
      setRender2(null);
    }
    if (render1 !== null && props.openedApps.actualUrl.app === "") {
      setRender1(null);
    }
    React.Children.forEach(props.children, (child: ReactNode) => {
      if (React.isValidElement(child) && child.props) {
        if (child.props.appName === props.openedApps.afterUrl.app) {
          temp[2] = React.cloneElement(child, {key: 1, shouldKey: 1});
          if (!render3 || !(render3 && (render3 as any).props.appName !== props.openedApps.afterUrl.app)) {
            setRender3(React.cloneElement(child, {key: 3, shouldKey: 3}));
          } else {
            console.log("no reload on 3");
          }
          // temp[2].key = uuidv4();
        }
        if (child.props.appName === props.openedApps.nextUrl.app) {
          temp[1] = React.cloneElement(child, {key: 2, shouldKey: 2});
          if (!render2 || !(render2 && (render2 as any).props.appName !== props.openedApps.nextUrl.app)) {
            setRender2(React.cloneElement(child, {key: 2, shouldKey: 2}));
          } else {
            console.log("no reload on 2");
          }
          // temp[1].key = uuidv4();
        }
        if (child.props.appName === props.openedApps.actualUrl.app) {
          temp[0] = React.cloneElement(child, {key: 3, shouldKey: 3});
          if (!render1 || !(render1 && (render1 as any).props.appName !== props.openedApps.actualUrl.app)) {
            setRender1(React.cloneElement(child, {key: 1, shouldKey: 1}));
          } else {
            console.log("no reload on 1");
          }
          // temp[0].key = uuidv4();
        }
      }
    });
    if (temp[0] && (temp[0] as any).props.appName === "Home") {
      temp[0] = null;
      setRender1(null);
    }
    if (temp[1] && (temp[1] as any).props.appName === "Home") {
      temp[1] = null;
      setRender2(null);
    }
    if (temp[2] && (temp[2] as any).props.appName === "Home") {
      temp[2] = null;
      setRender3(null);
    }
    setRender(temp);
  }, [props.openedApps]);

  return (
    <>
      {displayHome === true ? home : null}
      {render1}
      {render2}
      {render3}
    </>
  )

}