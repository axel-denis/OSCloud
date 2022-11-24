interface Props {
  launchPhotosPage: Function
}

export default function DesktopAppLayout(props: Props) {
  return (
    <>
      <div className="appsWidget">
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="appsWidget">
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="appsWidget">
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="appsWidget">
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
      <div className="doubleWidget">
        double
      </div>
      <div className="doubleWidget">
        double
      </div>
      <div className="appsWidget">
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
        <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      </div>
    </>
  )
}