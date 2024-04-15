import { FileInfo } from "../FilesApp/FilesApp"
import { backIp } from "../consts"

// SECTION - main data types

export type UserInfo = {
    id: number,
    name: string,
    role: "admin" | "user",
    enabled: boolean,
}

// SECTION - json responses data types

export type ListFilesResponse = {
    data?: Omit<ReceivedFileInfo[], 'fetchedAt'>
    errors?: Array<{ message: string }>
}

export type UserInfoResponse = {
    data?: {
        response: Omit<{
            user: UserInfo,
            mount_points: string[],
        }, 'fetchedAt'>
    }
    errors?: Array<{ message: string }>
}

// SECTION - official Back routes

export async function get_user() { // GET /user // user_info
    const options = {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer ' + localStorage.getItem("token"),
        },
    }

    const answer: Response = await fetch(backIp + "/user_info", options);
    const { data, errors }: UserInfoResponse = await answer.json();
    return { answer, data, errors };
}

export async function get_list_files(path: string) { // GET /list_files/:dir // list_files
    const options = {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': 'Bearer ' + localStorage.getItem("token"),
        },
    }

    const answer: Response = await fetch(backIp + "/list_files/" + path, options);
    const { data, errors }: ListFilesResponse = await answer.json();
    return { answer, data, errors };
}

// SECTION - added routes data extractors

export async function getUserMountPoints(): Promise<string[] | undefined> {
    const { answer, data, errors } = await get_user();
    if (answer.status === 200) {
        return data?.response.mount_points;
    } else {
        return undefined;
    }
}

// SECTION - misc

// TODO - fast patch - will be changed once the back returns proper types
type ReceivedFileType = "folder" | "file";

// TODO - fast patch - will be changed once the back returns proper types
interface ReceivedFileInfo {
    name: string;
    type: ReceivedFileType;
}

// TODO - fast patch - will be changed once the back returns proper types
export function receivedFileInfoToFileInfo(data: ReceivedFileInfo): FileInfo { // TODO - tmp until the back returns proper type
    return {
        name: data.name,
        type: data.type === "file" ? "unknown" : "folder",
        size: 10,
        rights: "write",
    }
}
