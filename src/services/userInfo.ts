import * as fs from 'fs';
import { getUser } from "~/dependencies/getUser";
import { User } from "../types/Users"
import type { RequestHandler } from 'express';

export const userInfo: RequestHandler = (req, res) => {
  let user: User | undefined = undefined;
  if (res.locals.user.type === "admin" || res.locals.user.name === req.body.name) {
    user = getUser(req.body.name);
  } else {
    return res.sendStatus(401);
  }

  if (!user) { // well, if we couldn't find the user...
    return res.sendStatus(404);
  }
  const { password, ...userWithoutPassword } = user; // remove password from object
  res.send(userWithoutPassword); // sending user object (in json)
}