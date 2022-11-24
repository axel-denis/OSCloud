import React from 'react';
import "./LoginPage.css"
import nextIcon from "../icons/next.svg"
import loadingIcon from "../icons/loading.svg"

interface Props {
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
}

export default function LoginPage(props: Props) {
  const [waitingValidation, setWaitingValidation] = React.useState<boolean>(false);

  function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    setWaitingValidation(true);
    setTimeout(() => {
      props.setIsLoggedIn(true);
      setWaitingValidation(false);
    }, 1000)
  }

  return (
    <div className={'backPannel ' + (!props.isLoggedIn ? "opened" : "closed")} >
      <div className={"flexCenter rotationAnimation " + (!props.isLoggedIn ? "opened" : "closed")}>

        <form className={'roundedBox centeredForm ' + (!props.isLoggedIn ? "opened" : "closed")} onSubmit={handleSubmit}>
          <h1 className={"h1LoginPage " + (!props.isLoggedIn ? "opened" : "closed")} >Connexion</h1>
          <div className={'fieldsBox ' + (waitingValidation ? "loading" : "notLoading")}>
            <div style={{ flex: "4" }}>
              <input id="email" type="text" placeholder='E-mail' style={{ paddingTop: "6px", height: "44px", borderBottom: "solid rgba(0, 0, 0, 0.253) 1px" }} /> {/* 50px de hauteur en tout*/}
              <input id="password" type="password" placeholder='Mot de passe' style={{ paddingBottom: "6px", height: "44px" }} />
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