import "./Banner.css"

type Props = {
  text: string;
  onClick: Function;
}

export default function Banner(props: Props) {
  return (
    <>
      <div className="banner">
        <h1 className="h1HomePage" onClick={() => props.onClick()}>
          {props.text}
        </h1>
      </div>
    </>
  )
}