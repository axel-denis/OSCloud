import { RequestHandler } from "express";
import { getUser } from "~/dependencies/getUser";
const jwt = require("jsonwebtoken");
require('dotenv').config();

export const login: RequestHandler = async (req, res) => {
  const user = await getUser(req.body.name, req.body.password);

  if (user === false)
    return res.sendStatus(500) // case where the json is invalid !

  if (!user || !req.body.name || !req.body.password) { // well, if we couldn't find the user...
    return res.status(401).send("invalid creds");
  }
  const { password, ...userWithoutPassword } = user; // deleting the password from the user object
  res.send({ // creating and sending json web token
    "jwt": jwt.sign(userWithoutPassword, process.env.ACCESS_TOKEN_SECRET, { expiresIn: '1800s' }),
  });
}