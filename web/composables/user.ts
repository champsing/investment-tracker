import axios from "axios";

export function logout() {
    localStorage.removeItem('token')
    localStorage.removeItem('username')
}

export function login(data: { token: string, username: string }) {
    localStorage.setItem('token', data.token);
    localStorage.setItem('username', data.username);
}

export function userId(): string {
    let token = localStorage.getItem('token')
    if (token == null) {
        return null
    }
    return JSON.parse(atob(token.split('.')[1]))['iss']
}

export function validUsername(username: string, error: (e: string) => void) {
    if (username.length < 6) {
        error('username too short')
    } else {
        axios.post("/api/user/exist", {
            username: username,
        }).then(response => {
            if (response.data) {
                error('username already exists')
            } else {
                error('')
            }
        })
    }
}

export function validPwd(data: { password2: string, err2: string }) {
    if (data.password2.length < 8) {
        data.err2 = 'password too short'
    } else {
        data.err2 = ''
    }
}

export function matchPwd(data: { password2: string, password3: string, err3: string }) {
    if (data.password2 != data.password3) {
        data.err3 = 'password does not match'
    } else {
        data.err3 = ''
    }
}

