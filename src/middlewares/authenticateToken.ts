import type { RequestHandler } from 'express';
const jwt = require("jsonwebtoken");

export const authenticateToken: RequestHandler = (req, res, next) => {
  const authHeader = req.headers["authorization"];
  const token = authHeader?.split(' ')[1]; // 'Bearer letoken'

  if (!token) {
    return res.sendStatus(401);
  }

  jwt.verify(token, process.env.ACCESS_TOKEN_SECRET, (err: Error, user: string) => {
    if (err) {
      return res.sendStatus(401);
    }
    res.locals.user = user;
    next();
  })
  return;
}