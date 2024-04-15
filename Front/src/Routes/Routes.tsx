import { backIp } from "../consts"

// SECTION - main data types

export type UserInfo = {
    id: number,
    name: string,
    role: "admin" | "user",
    enabled: boolean,
}

// SECTION - json responses data types

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

export async function get_user() { // GET /user
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

// SECTION - added routes data extractors

export async function getUserMountPoints(): Promise<string[] | undefined> {
    const { answer, data, errors } = await get_user();
    if (answer.status === 200 && data) {
        return data.response.mount_points;
    } else {
        return undefined;
    }
}
