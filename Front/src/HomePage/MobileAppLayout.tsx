interface Props {
  launchPhotosPage: Function,
  launchFilesPage: Function
}

export default function MobileAppLayout(props: Props) {
  return (
    <>
      <div className="app" onClick={() => props.launchPhotosPage()}>app</div>
      <div className="app" onClick={() => props.launchFilesPage()}>app</div>
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