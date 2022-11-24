interface Props {
  launchPhotosPage: Function
}

export default function MobileAppLayout(props: Props) {
  return (
    <>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
    </>
  )
}