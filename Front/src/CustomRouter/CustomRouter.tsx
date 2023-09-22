import React, { ReactNode } from "react"
import { UrlInfo } from "../UrlGestion";
import { AnimatePresence } from "framer-motion";

interface Props {
  openedApps: UrlInfo;
  children: ReactNode;
}
type Child = string | number | boolean
  | React.ReactElement<any, string | React.JSXElementConstructor<any>>
  | React.ReactFragment | React.ReactPortal | null | undefined;

export default function CustomRouter(props: Props) {
  let render: Child[] = [];

  React.Children.forEach(props.children, child => {
    if (React.isValidElement(child) && child.props) {
      if (child.key === props.openedApps.app) {
        render.push(child);
      }
    }
  });

  return (
    <AnimatePresence>
      {render}
    </AnimatePresence>
  );
}