import { getUser } from "~/dependencies/getUser";
import { User } from "../types/Users"
import type { RequestHandler } from 'express';

export const userInfo: RequestHandler = async (req, res) => {
  let user: User | undefined | false = undefined;

  if (!req.query.name) return res.sendStatus(404);

  if (res.locals.user.type === "admin" || res.locals.user.name === req.body.name) {
    user = await getUser(req.query.name.toString());
  } else {
    return res.sendStatus(401);
  }

  if (user === false)
    return res.sendStatus(500) // case where the json is invalid !
  if (!user) { // case where we couldn't find the user...
    return res.sendStatus(404);
  }
  const { password, ...userWithoutPassword } = user; // remove password from object
  res.send(userWithoutPassword); // sending user object (in json)
}