import React from 'react';
import "./LoginPage.css"
import nextIcon from "../icons/next.svg"
import loadingIcon from "../icons/loading.svg"
import { backIp, timeScale } from '../consts';
import { AnimatePresence, motion } from 'framer-motion';

interface Props {
  isLoggedIn: boolean;
  setIsLoggedIn: Function;
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
      }, 275);
    }
    setWaitingValidation(false);
  }
  console.log("the props is", props.isLoggedIn)
  return (
    <AnimatePresence>
      {!props.isLoggedIn &&
        <motion.div className={'backPannel'}
          initial={{ top: "100vh" }}
          animate={{ top: "0vh", transition: { duration: .6 * timeScale, ease: "easeOut"} }}
          exit={{ top: "100vh", transition: { duration: .6 * timeScale, ease: "easeIn"} }}
        >
          <form className={'roundedBox centeredForm ' + (errorAnim ? "shake" : "")} onSubmit={handleSubmit}>
            <h1 className="h1LoginPage">
              Connexion
            </h1>
            <div className={'fieldsBox ' + (waitingValidation ? "loading" : "notLoading")}>
              <div style={{ flex: "4" }}>

                <input ref={nameRef} id="name" type="text" placeholder='E-mail' style={{
                  color: errorAnim ? "red" : "",
                  paddingTop: "6px",
                  height: "44px",
                  borderBottom: "solid rgba(190, 190, 190, 255) 1px"
                }} /> {/* 50px de hauteur en tout*/}

                <input ref={passwordRef} id="password" type="password" placeholder='Mot de passe' style={{
                  color: errorAnim ? "red" : "",
                  paddingBottom: "6px",
                  height: "44px"
                }} />

              </div>
              <button className='nextButton'>
                <motion.div className='nextButtonSlider' whileHover={{ left: "-3.7rem" }}>
                  <div className='flexCenter nextButtonDivision'>
                    <img src={nextIcon} alt="next" style={{ height: "30px", opacity: ".5" }} />
                  </div>
                  <div className='flexCenter nextButtonDivision'>
                    <img src={nextIcon} alt="next" style={{ height: "30px", opacity: ".5" }} />
                  </div>
                </motion.div>
              </button>
              <div className={"loadingFieldsBox " + (waitingValidation ? "loading" : "notLoading")}>
                <motion.img src={loadingIcon} alt="loading" className='loadingRotation' />
              </div>
            </div>
          </form>

          <motion.div className='roundedBox decorativeFlyers'
            initial={{left: "calc(50% + 260px)" }}
            animate={{ left: "calc(50% + 150px)", transition: { duration: 1.75 * timeScale, delay: .1 * timeScale, ease: "circOut"} }}
            exit={{left: "calc(50% + 260px)" }}
            style={{
              top: "calc(22% - 100px)",
              width: "150px",
              height: "200px",
              rotate: "15deg",
              zIndex: 3
            }} />
          <motion.div className='roundedBox decorativeFlyers'
            initial={{left: "calc(50% - 550px)" }}
            animate={{ left: "calc(50% - 400px)", transition: { duration: 1.75 * timeScale, delay: .1 * timeScale, ease: "circOut"} }}
            exit={{left: "calc(50% - 550px)" }}
            style={{
              top: "calc(70% - 200px)",
              width: "300px",
              height: "400px",
              rotate: "-10deg"
            }} />
        </motion.div>
      }
    </AnimatePresence>
  )
}
