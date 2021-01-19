import * as Util from './util';
import * as PeachData from './peach-data';

const OAUTH_CLIENT_ID = "6289e8b7f6d38b068d87";

function redirect_to_provider() {
  // we dont have code and state, lets get them
  let state = Util.makeid(10);

  // save state to localstorage
  console.log("going to perform auth, state", state);
  window.localStorage.setItem("state", state);

  // doing login
  let params = new URLSearchParams();

  params.set("client_id", OAUTH_CLIENT_ID);
  params.set("redirect_uri", "https://localhost:8080/");
  params.set("scope", "user:email");
  params.set("state", state);

  window.location.href = "https://github.com/login/oauth/authorize?" + params.toString();

  return new Promise((res, rej) => {
    // blank, unresolved promise
  });
}

function get_session() {
  let params = new URLSearchParams(window.location.search);

  if (params.has('code') && params.has('state')) {
    // if we have a code and state, check if state matches. else error.
    if (localStorage.getItem("state") == undefined || localStorage.getItem("state") != params.get("state")) {
      return Promise.reject("error");
    }

    let code = params.get('code');

    return PeachData.create_session(code).then(s=>{
      // clear the url.
      window.location.href = "/";

      return new Promise((res, rej) => {
        // blank, unresolved promise
      });
    });
  }

  return redirect_to_provider();
}

function init() {
  return PeachData.has_session().then(session => {
    if(session) {
      return localStorage.getItem("session");
    } else {
      return get_session();
    }
  });
}

console.log("Hello!!!");

init().then(session => {
  console.log("init", session);

  document.addEventListener("DOMContentLoaded", () => {
    let codeEl = document.createElement("h1");
    codeEl.textContent = session;
    document.body.appendChild(codeEl);
  });
});
