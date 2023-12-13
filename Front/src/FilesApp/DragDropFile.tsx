import React from "react";
import "./DragDropFile.css";
import PopIn from "../PopIn/PopIn";

export default function DragDropFile() {
  const [dragActive, setDragActive] = React.useState(false);
  const [dropActive, setDropActive] = React.useState(false);

  const handleDrag = function (e: any) {
    e.preventDefault();
    e.stopPropagation();
    console.log("handle drag");
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave") {
      console.log("drag leave");
      setDragActive(false);
    }
  };

  const handleDrop = function (e: any) {
    e.preventDefault();
    e.stopPropagation();
    console.log("handle drop");
    setDragActive(false);
    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      setDropActive(true);
      console.log(e.dataTransfer.files);
    } else {
      console.log(e.dataTransfer);
    }
  };

  return (
    <form className={"dragAndDrop" + (dragActive ? " dragActive" : "")}
      style={{zIndex: dropActive ? 5 : ""}}
      onDragEnter={handleDrag}
      onDragLeave={handleDrag}
      onDragOver={handleDrag}
      onDrop={handleDrop}
      onSubmit={(e) => e.preventDefault()}
    >
      {dropActive &&
        <PopIn zIndex={10}>
          <h2>Do you confirm the upload of these files ?</h2>
          <ul>
            <li>list des files</li>
          </ul>
        </PopIn>
      }
    </form>
  );
};