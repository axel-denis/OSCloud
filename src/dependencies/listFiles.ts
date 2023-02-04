import { Dirent, promises as fs } from 'fs';

export interface DirAndFiles {
  files: string[],
  directories: string[]
}

//lists files and directories from a given path
export const listFiles = async (path: string): Promise<DirAndFiles> => {
  let data: Dirent[];
  let output: DirAndFiles = {
    files: [],
    directories: []
  };

  try {
    data = await fs.readdir(path, {withFileTypes: true});
  } catch {
    return output;
  }
  data.forEach(file => {
    if (file.isFile()) output.files.push(file.name);
    if (file.isDirectory()) output.directories.push(file.name);
  })
  return output;
}