import React, { ReactElement, ReactNode } from "react"
import { UrlsHandler } from "../UrlGestion";
import HomePage from "../HomePage/HomePage";

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
  const [render, setRender] = React.useState<[ReactNode | null, ReactNode | null]>([null, null]);
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
    let temp: [ReactNode | null, ReactNode | null] = [null, null]
    if (props.openedApps.nextUrl.app === "Home" || props.openedApps.actualUrl.app === "Home") {
      setDisplayHome(true);
    } else {
      setDisplayHome(false)
    }
    React.Children.forEach(props.children, (child: ReactNode) => {
      if (React.isValidElement(child) && child.props) {
        if (child.props.appName === props.openedApps.nextUrl.app) {
          temp[1] = child;
        }
        if (child.props.appName === props.openedApps.actualUrl.app) {
          temp[0] = child;
          console.log("0router[0] is updated from to", render[0], temp[0])
        }
      }
    });
    if (temp[0] && (temp[0] as any).props.appName === "Home") {
      temp[0] = null;
    }
    if (temp[1] && (temp[1] as any).props.appName === "Home") {
      temp[1] = null;
    }
    setRender(temp);
  }, [props.openedApps]);

  return (
    <>
      {displayHome === true ? home : null}
      {render}
    </>
  )

}