export function userGroup(): string {
    let token = localStorage.getItem('token')
    if (token == null) {
        return ""
    }
    return JSON.parse(atob(token.split('.')[1]))['iss']
}
