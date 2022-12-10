import React from 'react';
import "./LoginPage.css"
import nextIcon from "../icons/next.svg"
import loadingIcon from "../icons/loading.svg"
import { backIp } from '../consts';

interface Props {
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
}

interface loginForm {
  name: string,
  password: string
}

export default function LoginPage(props: Props) {
  const [waitingValidation, setWaitingValidation] = React.useState<boolean>(false);
  const [errorAnim, setErrorAnim] = React.useState<boolean>(false);
  const nameRef = React.useRef<HTMLInputElement>(null);
  const passwordRef = React.useRef<HTMLInputElement>(null);

  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setWaitingValidation(true);
    const options = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        name: nameRef.current?.value,
        password: passwordRef.current?.value
      })
    }
    const answer: Response = await fetch(backIp + "/login", options);
    if (answer.status === 200) {
      props.setIsLoggedIn(true);
    } else {
      setErrorAnim(true);
      setTimeout(() => {
        setErrorAnim(false);
      }, 250);
    }
    setWaitingValidation(false);
  }

  return (
    <div className={'backPannel ' + (!props.isLoggedIn ? "opened" : "closed")} >
      <div className={"flexCenter rotationAnimation " + (!props.isLoggedIn ? "opened" : "closed")}>

        <form className={'roundedBox centeredForm ' + (!props.isLoggedIn ? "opened " : "closed ") + (errorAnim ? "shake" : "")} onSubmit={handleSubmit}>
          <h1 className={"h1LoginPage " + (!props.isLoggedIn ? "opened" : "closed")} >Connexion</h1>
          <div className={'fieldsBox ' + (waitingValidation ? "loading" : "notLoading")}>
            <div style={{ flex: "4" }}>
              <input ref={nameRef} id="name" type="text" placeholder='E-mail' style={{ color: errorAnim ? "red" : "", paddingTop: "6px", height: "44px", borderBottom: "solid rgba(0, 0, 0, 0.253) 1px" }} /> {/* 50px de hauteur en tout*/}
              <input ref={passwordRef} id="password" type="password" placeholder='Mot de passe' style={{ color: errorAnim ? "red" : "", paddingBottom: "6px", height: "44px" }} />
            </div>
            <button className='nextButton'>
              <div className='nextButtonSlider'>
                <div className='flexCenter nextButtonDivision'>
                  <img src={nextIcon} alt="next" style={{ height: "30px", opacity: ".5" }} />
                </div>
                <div className='flexCenter nextButtonDivision'>
                  <img src={nextIcon} alt="next" style={{ height: "30px", opacity: ".5" }} />
                </div>
              </div>
            </button>
            <div className={"loadingFieldsBox " + (waitingValidation ? "loading" : "notLoading")}>
              <img src={loadingIcon} alt="loading" className='loadingRotation' style={{ height: "30px", opacity: ".5" }} />
            </div>
          </div>
        </form>
        <div className='roundedBox decorativeFlyers' style={{ left: !props.isLoggedIn ? "calc(50% + 200px)" : "calc(50% + 300px)", top: "calc(22% - 100px)", width: "150px", height: "200px", rotate: "15deg", zIndex: 3 }}></div>
        <div className='roundedBox decorativeFlyers' style={{ left: !props.isLoggedIn ? "calc(50% - 400px)" : "calc(50% - 500px)", top: "calc(70% - 200px)", width: "300px", height: "400px", rotate: "-10deg" }}></div>

      </div>
    </div>
  )
}