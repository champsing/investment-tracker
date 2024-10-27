import axios from "axios";

export function logout() {
    localStorage.removeItem('token')
    localStorage.removeItem('username')
}

export function login(data: { token: string, username: string }) {
    localStorage.setItem('token', data.token);
    localStorage.setItem('username', data.username);
}

export function validUsr(data: { username1: string, err1: string }) {
    if (data.username1.length < 6) {
        data.err1 = 'username too short'
    } else {
        axios.post("/api/user/exist", {
            username: data.username1,
        }).then(response => {
            if (response.data) {
                data.err1 = 'username already exists'
            } else {
                data.err1 = ''
            }
        }).catch(_ => {
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

export function userId(): string {
    let token = localStorage.getItem('token')
    if (token == null) {
        return null
    }
    return JSON.parse(atob(token.split('.')[1]))['iss']
}

