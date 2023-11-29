const { invoke } = window.__TAURI__.tauri;

// const debugbtn = document.querySelector('.debug-btn');

// debugbtn.addEventListener('click', async () => {

// })

const loginForm = document.querySelector('.login-form');
const loginUsernameInput = document.querySelector('.login-username-input');
const loginPasswordInput = document.querySelector('.login-password-input');

const registerForm = document.querySelector('.register-form');
const registerUsernameInput = document.querySelector('.register-username-input');
const registerEmailInput = document.querySelector('.register-email-input');
const registerPasswordInput = document.querySelector('.register-password-input');

async function login(username, password) {
    let loginSuccessful = await invoke("login", { username, password });
    if (loginSuccessful) {
        console.log("Login Successful");
        location.replace('/pages/home.html');
        return true;
    } else {
        console.log("Something went wrong");
        return false
    }
}

async function storeUser(username, email, password) {
    let registerSuccessful = await invoke("store_user", { username, email, password });
    if (registerSuccessful) {
        console.log("Register Successful");
        location.replace('/pages/login.html');
        return true;
    }
    else {
        console.log("Something went wrong");
        return false
    }
}

if (loginForm) {
    loginForm.addEventListener("submit", (e) => {
        e.preventDefault()
        if (loginUsernameInput.value === "" || loginPasswordInput.value === "") {
            console.error("Fill in both inputs please");
            return false;
        }

        login(loginUsernameInput.value, loginPasswordInput.value);
    })
}

if (registerForm) {
    registerForm.addEventListener("submit", (e) => {
        e.preventDefault();
        if (registerUsernameInput.value === "" || registerEmailInput.value === "" || registerPasswordInput.value === "") {
            console.error("Fill in all inputs please");
            return false;
        }

        storeUser(registerUsernameInput.value, registerEmailInput.value, registerPasswordInput.value);
    })
}