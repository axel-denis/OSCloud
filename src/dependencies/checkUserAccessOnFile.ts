import { promises as fs } from 'fs';

interface UserPathsData {
  [name: string]: string[]
}

/*
  Check that user have access on a file or folder by checking usersFoldersAccess.json
  Also return false if file or folder does not exists
*/
export const checkUserAccessOnFile = async (username: string, path: string): Promise<boolean> => {
  let authorized_paths: UserPathsData;
  try {
    authorized_paths = JSON.parse(await fs.readFile("database/usersFoldersAccess.json", "utf-8"));
  } catch (error) {
    console.log("database/usersFoldersAccess.json is not correct.\n", error);
    return false;
  }

  if (!authorized_paths[username]) // si l'utilisateur n'est pas enregistré dans le fichier
    return false;

  for (let elem of authorized_paths[username]) { // vérification de l'accès
    if (path.startsWith(elem))
      try {
        await fs.access(path);
        return true;
      } catch {
        return false;
      }
  }
  return false;
}