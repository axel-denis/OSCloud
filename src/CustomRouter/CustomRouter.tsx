import React, { ReactElement, ReactNode } from "react"
import { UrlsHandler } from "../UrlGestion";

interface Props {
  openedApps: UrlsHandler;
  children: ReactNode;
}

export default function CustomRouter(props: Props) {
  const [render, setRender] = React.useState<[ReactNode | null, ReactNode | null]>([null, null]);

  // selection of the one or two children to render :
  React.useEffect(() => {
    let temp: [ReactNode | null, ReactNode | null] = [null, null]
    React.Children.forEach(props.children, (child: ReactNode) => {
      if (React.isValidElement(child) && child.props) {
        if (child.props.appName === props.openedApps.nextUrl.app) {
          temp[1] = child;
        }
        if (child.props.appName === props.openedApps.actualUrl.app) {
          temp[0] = child;
        }
      }
    });
    if (temp[1] && (temp[1] as any).props.appName === "Home") { // FIXME - can't find why the default ReactNode type doesn't fit anymore, need to cast into any
      temp[0], temp[1] = temp[1], temp[0]
    }
    setRender(temp);
  }, [props.openedApps]);

  return (
    <>
      {render}
    </>
  )

}