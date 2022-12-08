import { RequestHandler } from "express";

export const homePage: RequestHandler = (req, res) => {
  res.send(res.locals.user)
}