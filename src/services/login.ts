import { Request, Response } from "express";
import * as fs from 'fs';
const jwt = require("jsonwebtoken");
import { User } from "../types/Users"
require('dotenv').config();

export default (req: Request, res: Response) => {
  let users: User[];
  try {
    users = JSON.parse(fs.readFileSync("database/users.json", "utf-8"));
  } catch (error) {
    console.log("database/users.json is not correct.\n", error);
    return res.sendStatus(500)
  }
  const user: User = users.filter((user: User) => {
    return user.name === req.body.name && user.password === req.body.password;
  })[0];
  if (!user) {
    return res.status(401).send("invalid creds");
  }
  const { password, ...userWithoutPassword } = user;
  res.send({
    "jwt": jwt.sign(userWithoutPassword, process.env.ACCESS_TOKEN_SECRET, { expiresIn: '1800s' }),
  });
}