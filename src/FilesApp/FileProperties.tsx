interface Props {
  file_id: number;
}

export default function FileProperties(props: Props) {
  return (
    <>
      <div className="filePropertiesLayout">

        <div className="filePropertiesElement">
          <h3 className="filePropertiesTitle">Type</h3>
          Blender File
        </div>

        <div className="filePropertiesElement">
          <h3 className="filePropertiesTitle">Size</h3>
          81 ko
        </div>

        <div className="filePropertiesElement">
          <h3 className="filePropertiesTitle">Location</h3>
          ~/3D/project/projetc.blend
        </div>

        <div className="filePropertiesElement">
          <h3 className="filePropertiesTitle">Owner</h3>
          You
        </div>

        <div className="filePropertiesElement">
          <h3 className="filePropertiesTitle">Created</h3>
          23/08/2023 (by you)
        </div>

        <div className="filePropertiesElement">
          <h3 className="filePropertiesTitle">Last modification</h3>
          26/08/2023 (by you)
        </div>

        <div className="filePropertiesElement">
          <h3 className="filePropertiesTitle">Shared with</h3>
          . . . . . . . . . . . . . . . . . . . . . . . . . .
        </div>
      </div>
    </>
  )
}
