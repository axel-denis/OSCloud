const fs = require("fs");
const express = require("express");
const cors = require("cors");
const random = require("lodash");
const { v4: uuid } = require("uuid");
const jwt = require("jsonwebtoken");
const { exit } = require("process");
require('dotenv').config();
import { authenticateToken } from "./middlewares/authenticateToken";
import { User } from "./types/Users"
import login from "./services/login";

import { Application, Request, Response } from "express";

let users: User[];
try {
  users = JSON.parse(fs.readFileSync("database/users.json", "utf-8"));
} catch (error) {
  console.log("database/users.json is not correct.\n", error);
  exit();
}

if (process.env.IS_ENV_SETUP === undefined) {
  console.log(".env not setup. Create your env as asked in the documentation and add the IS_ENV_SETUP variable in it.");
  exit();
}

const app: Application = express();

app.use(express.json());
app.use(cors()); // le cors c'était pas bien expliqué pourquoi c'était, peut être à virer à l'avenir
app.use(express.urlencoded({extended: true}));

app.post("/login", login);

app.get("/outfits", (req: Request, res: Response) => {
  const tops = ["black", "orange", "navy"];
  const jeans = ["black", "orange", "navy"];
  const shoes = ["black", "orange", "navy"];

  res.json({
    top: <string>random.sample(tops),
    jeans: <string>random.sample(jeans),
    shoes: <string>random.sample(shoes),
  });
});

app.post("/comments", (req: Request, res: Response) => {
  const id = uuid();
  const content = req.body.content;

  if (!content) {
    return res.sendStatus(400);
  }

  fs.mkdirSync("data/comments", {recursive: true});
  fs.writeFileSync(`data/comments/${id}.txt`, content)

  res.status(201).json({
    id: id,
    content: fs.readFileSync(`data/comments/${id}.txt`, "utf-8")
  });
})

app.get("/comments/:id", (req: Request, res: Response) => {
  const id = req.params.id;
  let content;

  try {
    content = fs.readFileSync(`data/comments/${id}.txt`, "utf-8");
  } catch (err) {
    return res.sendStatus(404);
  }

  res.json({
    content: content
  })
})

app.get("/home", authenticateToken, (req: Request, res: Response) => {
  res.send(res.locals.user);
})

app.listen(8080, () => console.log("running on 8080"));