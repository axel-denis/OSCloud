import "./Banner.css"
import { ReactNode } from "react";

type Props = {
  text: string;
  onClick: Function;
  leftChildren?: ReactNode,
  rightChildren?: ReactNode,
}

export default function Banner(props: Props) {
  return (
    <>
      <div className="banner">
        {props.leftChildren}
        <h1 className="h1HomePage" onClick={() => props.onClick()}>
          {props.text}
        </h1>
        {props.rightChildren}
      </div>
    </>
  )
}
