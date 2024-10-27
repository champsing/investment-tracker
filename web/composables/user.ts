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

export function validUsername(username: string, ret: (e: string) => void) {
    if (username.length < 6) {
        ret('username too short')
    } else {
        axios.post("/api/user/exist", {
            username: username,
        }).then(response => {
            if (response.data) {
                ret('username already exists')
            } else {
                ret('')
            }
        })
    }
}

export function validPassword(password: string, ret: (e: string) => void) {
    if (password.length < 8) {
        ret('password too short')
    } else {
        ret('')
    }
}

export function matchPassword(p1: string, p2: string, ret: (e: string) => void) {
    if (p1 != p2) {
        ret('password does not match')
    } else {
        ret('')
    }
}
