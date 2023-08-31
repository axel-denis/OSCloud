import "./ProtectorOverlay.css"

// this is an element to load if you don't want a user to interact with the app (during a transition by example)
export default function ProtectorOverlay() {
  console.log("I exist");
  return (
    <div className="ProtectorOverlay" />
  )
}