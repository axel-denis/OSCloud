import type { RequestHandler } from 'express';
const jwt = require("jsonwebtoken");

export const authenticateToken: RequestHandler = (req, res, next) => {
  const authHeader = req.headers["authorization"];
  const token = authHeader?.split(' ')[1]; // split 'Bearer token' and takes only the token

  if (!token) {
    return res.sendStatus(401);
  }

  jwt.verify(token, process.env.ACCESS_TOKEN_SECRET, (err: Error, user: string) => { // verify user session
    if (err) {
      return res.sendStatus(401);
    }
    res.locals.user = user; // add data to be transmitted to future functions that will works with this user such as home.ts
    next();
  })
  return;
}