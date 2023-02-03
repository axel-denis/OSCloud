import { User } from "~/types/Users";
import { promises as fs } from 'fs';

export const getUser = async (userName: string, password: string | null = null): Promise<User | undefined | false> => {
  // getting user and sending 500 internal server error if so.
  let users: User[];
  try {
    users = JSON.parse(await fs.readFile("database/users.json", "utf-8"));
  } catch (error) {
    console.log("database/users.json is not correct.\n", error);
    return false;
  }

  let user: User;
  if (password) { // if we also need to check password
    user = users.filter((user: User) => {
      return user.name === userName && user.password === password;
    })[0]; // only one user, but returns an array, so taking the first.
  } else { // if we don't need to check password
    user = users.filter((user: User) => {
      return user.name === userName;
    })[0]; // only one user, but returns an array, so taking the first.
  }
  if (!user) { // well, if we couldn't find the user...
    return undefined;
  }
  return user;
}
