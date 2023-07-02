import type { RequestHandler } from 'express';
import { checkUserAccessOnFile } from '~/dependencies/checkUserAccessOnFile';
import {DirAndFiles, listFiles } from '~/dependencies/listFiles';

// if the user have access and the path exists, lists the files and directories.
export const files: RequestHandler = async (req, res) => {
  let filesList: DirAndFiles | undefined = undefined;

  if (!req.query.path) return res.sendStatus(404);

  if (await checkUserAccessOnFile(res.locals.user.name, req.query.path.toString())) {
      filesList = await listFiles(req.query.path.toString());
  } else {
    return res.sendStatus(401); // file does not exists or acces denied.
  }
  // it's good that the user or the front can't find a diff between a not existing file
  // and an existing file but denied access. It prevent one from getting a list of files by
  // bruteforcing and finding 404 instead of 401 !

  res.send(filesList);
}