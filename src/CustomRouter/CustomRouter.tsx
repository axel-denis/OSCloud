import React, { ReactNode } from "react"
import { UrlsHandler } from "../UrlGestion";

interface Props {
  openedApps: UrlsHandler;
  children: ReactNode;
}
type Child = string | number | boolean
  | React.ReactElement<any, string | React.JSXElementConstructor<any>>
  | React.ReactFragment | React.ReactPortal | null | undefined;

export default function CustomRouter(props: Props) {
  let render: [Child, Child] = [null, null];

  // selection of the one or two children to render :
  React.Children.forEach(props.children, child => {
    if (React.isValidElement(child) && child.props) {
      if (child.props.appName === props.openedApps.nextUrl.app) {
        render[0] = child;
      }
      if (child.props.appName === props.openedApps.actualUrl.app) {
        render[1] = child;
      }
    }
  });

  return (
    <>
      {render}
    </>
  )
}