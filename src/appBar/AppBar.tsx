import { useEffect, useState } from 'react';
import type { FunctionComponent } from 'react';
import "./appBar.css";

interface Props {
  appsCount: number,
}

export default function AppBar(props: Props) {
  const displayAbove = .9; // display bar if mouse is at bottom 10% of the viewport
  const stayDisplayedAbove = .8; // if already displayed, mouse have 20% of the bottom of
                                 // the viewport to move freely without hiding the bar
  const [displayBar, setDisplayBar] = useState<boolean>(true);

  useEffect(() => {
    const handleMouseMove = ({ clientY }: MouseEvent): void => {
      const ratio = clientY / window.innerHeight;

      if (ratio > displayAbove) setDisplayBar(true);
      else if (ratio < stayDisplayedAbove) setDisplayBar(false);
    };

    window.addEventListener('mousemove', handleMouseMove);

    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
    };
  }, []);

  return (
    <div className={"appBar" + (displayBar ? " appBarDisplayed" : " ")} style={{width: 6.8 * props.appsCount + "rem"}}>
      <div className="app appHoverUp" onClick={() => {}}>app</div>
      <div className="app appHoverUp" onClick={() => {}}>app</div>
      <div className="app appHoverUp" onClick={() => {}}>app</div>
      <div className="app appHoverUp" onClick={() => {}}>app</div>
      <div className="app appHoverUp" onClick={() => {}}>app</div>
    </div>
  )
}