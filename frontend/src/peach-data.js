const API_BASE = "https://localhost:8080";

export function has_session() {
    let has_session_url = API_BASE + "/has_session";
    return fetch(has_session_url, {
        credentials: "include",
    }).then(response => {
        return response.ok;
    });
}

export function create_session(code) {
    let create_session_url = API_BASE + "/create_session";
    return fetch(create_session_url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify({
            code: code
        })
    });
}
