// const { invoke } = window.__TAURI__.tauri;

// let username;
// let email;
// let select = document.querySelector('.dropdown');
// let newEmailInput = document.querySelector('.new-email');
// let newUsernameInput = document.querySelector('.new-username');

// async function getUsers() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   let users = await invoke("get_users", {});
//   users = JSON.parse(users);
//   console.log(users);
//   return users
// }

// async function storeUser(username, email) {
//   let storedUser = await invoke("store_user", { username, email });
//   console.log(storedUser);
// }

// async function login

// async function fillCheckbox() {
//   let users = await getUsers();
//   users.forEach(user => {

//     let option = document.createElement("option");
//     option.setAttribute("value", `${user.username}, ${user.email}`);
//     option.text = user.username;
//     select.append(option);
//   });

//   if (newEmailInput && newUsernameInput) {
//     // Add change event listener to the select element
//     select.addEventListener("change", () => {
//       // Get the selected option's value
//       const selectedOption = select.value;
//       const [username, email] = selectedOption.split(",");

//       // Set newEmailInput and newUsernameInput values with old email and old username
//       newEmailInput.value = email;
//       newUsernameInput.value = username;
//     });
//   }
// }

// async function deleteUser(username) {
//   let deletedUser = await invoke("delete_user", { username });
//   select.innerHTML = "";
//   fillCheckbox();
// }

// async function updateUser(username, email, oldUsername) {
//   let updatedUser = await invoke("update_user", { username, email, oldUsername });
//   select.innerHTML = "";
//   fillCheckbox();
// }

// window.addEventListener("DOMContentLoaded", async () => {
//   if (document.querySelector('form')) {
//     document.querySelector('form').addEventListener('submit', async (e) => {
//       username = document.querySelector(".username").value;
//       email = document.querySelector(".email").value;
//       e.preventDefault();
//       storeUser(username, email)
//     })
//     if (document.querySelector('.get-user-btn')) {
//       document.querySelector('.get-user-btn').addEventListener('click', async (e) => {
//         e.preventDefault();
//         getUsers();
//       })
//     }
//   } else {
//     await fillCheckbox();
//     if (document.querySelector('.delete-btn')) {
//       document.querySelector('.delete-btn').addEventListener('click', async () => {
//         const selectedOption = select.value;
//         const [username, email] = selectedOption.split(",");
//         deleteUser(username);
//       })
//     }

//     if (document.querySelector('.update-btn')) {
//       document.querySelector('.update-btn').addEventListener('click', async () => {
//         const selectedOption = select.value;
//         const [username, email] = selectedOption.split(",");
//         updateUser(newUsernameInput.value, newEmailInput.value, username);
//       })
//     }
//   }
// });