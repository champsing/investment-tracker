export function getUsername(): string {
    let token = localStorage.getItem('token')
    if (token == null) {
        return ""
    }
    return JSON.parse(atob(token.split('.')[1]))['iss']
}

export function userInfo(): { username: string, token: string } {
    let token = localStorage.getItem('token')
    if (token == null) {
        return null
    }
    return JSON.parse(atob(token.split('.')[1]))['iss']
}

