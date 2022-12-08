import { RequestHandler } from "express";
import * as fs from 'fs';
import { getUser } from "~/dependencies/getUser";
const jwt = require("jsonwebtoken");
import { User } from "../types/Users"
require('dotenv').config();

export const login: RequestHandler = (req, res) => {
  let users: User[];
  // getting user and sending 500 internal server error if so.
  try {
    users = JSON.parse(fs.readFileSync("database/users.json", "utf-8"));
  } catch (error) {
    console.log("database/users.json is not correct.\n", error);
    return res.sendStatus(500)
  }

  const user = getUser(req.body.name, req.body.password);
  if (!user) { // well, if we couldn't find the user...
    return res.status(401).send("invalid creds");
  }
  const { password, ...userWithoutPassword } = user; // deleting the password from the user object
  res.send({ // creating and sending json web token
    "jwt": jwt.sign(userWithoutPassword, process.env.ACCESS_TOKEN_SECRET, { expiresIn: '1800s' }),
  });
}