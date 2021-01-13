function init() {
  if (localStorage.getItem("session") != undefined) {
    // we have session
    console.log("logged in");

    return localStorage.getItem("session");
  }

  let params = new URLSearchParams(window.location.search);

  if (params.has('code') && params.has('state')) {
    // if we have a code and state, check if state matches. else error.
    if (localStorage.getItem("state") == undefined || localStorage.getItem("state") != params.get("state")) {
      console.log("error");
      return;
    }

    let code = params.get('code');
    // TODO - post code to backend to create session.
    localStorage.setItem("session", code);
    console.log("code", code);

    window.location.href = "/";
  } else {
    // we dont have code and state, lets get them
    let state = makeid(10);

    // save state to localstorage
    console.log("going to perform auth, state", state);
    window.localStorage.setItem("state", state);

    // doing login
    let params = new URLSearchParams();

    params.set("client_id", "6289e8b7f6d38b068d87");
    params.set("redirect_uri", "http://localhost:8080/");
    params.set("scope", "user:email");
    params.set("state", state);

    window.location.href = "https://github.com/login/oauth/authorize?" + params.toString();
  }
}

function makeid(length) {
  var result = '';
  var characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  var charactersLength = characters.length;
  for (var i = 0; i < length; i++) {
    result += characters.charAt(Math.floor(Math.random() * charactersLength));
  }
  return result;
}

console.log("Hello!!!");

let session = init();

if (session) {
  document.addEventListener("DOMContentLoaded", () => {
    let codeEl = document.createElement("h1");
    codeEl.textContent = session;
    document.body.appendChild(codeEl);
  });
}
