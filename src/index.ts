import express from "express";
import { Application, Request, Response } from "express";
import cors from "cors";
import { exit } from "process";
require('dotenv').config();
import { authenticateToken } from "./middlewares/authenticateToken";
import { login } from "./services/login";
import { userInfo } from "./services/userInfo";
import { homePage } from "./services/home";

// ERROR HANDLING
if (process.env.IS_ENV_SETUP === undefined) {
  console.log(".env not setup. Create your env as asked in the documentation and add the IS_ENV_SETUP variable in it.");
  exit();
}

//Express app
const app: Application = express();

//convert incoming data into json
app.use(express.json());
app.use(cors()); // Semblerait que ça serve à accepter les co de tous les domaines, ou que de quelques uns
app.use(express.urlencoded({ extended: true })); // sert à lire le utf-8 je crois

//login route
app.post("/login", login);

app.get("/userInfo", authenticateToken, userInfo);

app.get("/home", authenticateToken, homePage)

app.listen(8080, () => console.log("running on 8080"));