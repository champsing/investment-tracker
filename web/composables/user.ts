import axios from "axios";

export function logout() {
    localStorage.removeItem('token')
    localStorage.removeItem('username')
}

export function login(data: { token: string, username: string }) {
    localStorage.setItem('token', data.token);
    localStorage.setItem('username', data.username);
}

export function getUserId(): string {
    let token = localStorage.getItem('token')
    if (token == null) {
        return null
    }
    return JSON.parse(atob(token.split('.')[1]))['iss']
}

export function getUsername(): string {
    return localStorage.getItem('username');
}

export async function duplicateUsername(username: string): Promise<boolean | string> {
    let hasUser = (await axios.post("/api/user/exist", {
        username: username,
    })).data;
    return !hasUser || 'username already exists'
}

