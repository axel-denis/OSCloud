const fs = require('fs');
const { v4: uuid } = require("uuid");
console.log("this program will install sample .env and database/users.json data");

try {
  fs.mkdirSync("database/")
} catch (error) {}

try {
fs.writeFileSync("database/users.json", `[
  {
    "id": 0,
    "name": "Axel",
    "type": "admin",
    "password": "password"
  },
  {
    "id": 1,
    "name": "Arthur",
    "type": "user",
    "password": "mdp"
  }
]`);
} catch (error) {}

try {
  fs.writeFileSync(".env", `ACCESS_TOKEN_SECRET=${uuid()}
  IS_ENV_SETUP=1`);
  } catch (error) {}