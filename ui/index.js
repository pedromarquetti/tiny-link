const modal = document.querySelector(".modal");

const info_modal_div = document.getElementById("info-modal");
const register_login_modal_div = document.getElementById(
	"login-register-modal"
);
const footer_msg = document.getElementById("login-register-message");

const close_modal_button = document.getElementById("close-modal");
const cancel_button = document.querySelectorAll(".cancel-button");
const modal_title = document.getElementById("modal-title");
const modal_text = document.getElementById("modal-text");

const login_register_button = document.getElementById("login-register-button");
const user_login_form = document.getElementById("user-login-form");
const user_register_form = document.getElementById("user-register-form");

const link_redirect = document.getElementById("link_redirect");
const create_link_form = document.getElementById("submit-long-link");

/** Delay next exec.
 * @param ms: delay in milliseconds
 */
const delay = (ms) => new Promise((res) => setTimeout(res, ms));

/**
 * closes modal and cleans up classNames
 * */
function close_modal() {
	modal.close();
	// removes close class if present
	modal.classList.remove("close");
	// removes the animation event listener, modal was closed
	modal.removeEventListener("animationend", close_modal);

	// cleaning up...
	close_modal_button.className = "";
	modal_title.textContent = "";
	modal_text.textContent = "";
	modal.id = "";
	info_modal_div.classList.add("hidden");
	register_login_modal_div.classList.add("hidden");
	user_register_form.classList.add("hidden");
	user_login_form.classList.add("hidden");
}
/**
 * Handles animation for modal.close()
 * @argument e-> event, used to prevent page reload
 */
function handle_close(e) {
	// run close_modal
	modal.addEventListener("animationend", close_modal);
	// add "close" class to <dialog> (this will trigger CSS animations)
	modal.classList.add("close");

	e.preventDefault();
}

// if user clicks X/Cancel btn inside modal, close modal
close_modal_button.addEventListener("click", handle_close);
cancel_button.forEach((button) => {
	button.addEventListener("click", handle_close);
});

// if user press 'esc' with modal.show[Modal]
modal.addEventListener("keydown", (e) => {
	const key = e.keyCode;

	if (key === 27) {
		// run handle_close
		handle_close(e);
	}
});

// show login/register form if user clicks login/register button
login_register_button.addEventListener("click", () => {
	modal.className = "modal";
	modal.id = "login-register";
	register_login_modal_div.classList.remove("hidden");
	modal_title.textContent = "Login or Register";

	modal.showModal();
});

/**
 * Information modal to be displayed
 * @argument type: info/error/success
 * @argument title: modal title to be displayed
 * @argument message: message to be displayed
 */
async function create_link_modal(type, title, message) {
	// removing classes from modal / modal_button
	modal.className = "modal";
	info_modal_div.classList.remove("hidden");

	//spawning modal
	modal.show();
	switch (type) {
		case "err":
			// with these classes
			modal.classList.add("error");
			close_modal_button.classList.add("error");

			// and this content
			modal_title.textContent = title;
			modal_text.textContent = message;

			break;
		case "ok":
			modal.classList.add("ok");
			close_modal_button.classList.add("ok");

			modal_title.textContent = title;
			modal_text.innerHTML = message;

			break;

		default:
			modal.classList.add("info");
			close_modal_button.classList.add("info");

			modal_text.textContent = "!";
			modal_title.textContent = message;

			break;
	}
}

create_link_form.addEventListener("submit", create_link);

/**
 * Main function to handle link shortener form that will be sent to server
 * @
 * */
async function create_link(e) {
	// don't reload page on form submit
	e.preventDefault();

	let long_link = e.target[0].value;
	let payload = { long_url: long_link };
	let res = await fetch("/api/link/create/", {
		method: "POST",
		body: JSON.stringify(payload),
		headers: {
			"Content-Type": "application/json",
		},
		mode: "cors",
	});

	let short_link = await res.json();
	const { error, message } = short_link;

	//spawning modal
	modal.id = "info-modal";

	if (!res.ok) {
		await create_link_modal("err", "Error!", error);
	} else {
		const host = window.location.origin;
		const a_tag = `<a target='_blank' href=${host}/${message.short_link}>click here to open it!</a>`;

		await create_link_modal(
			"ok",
			"Done!",
			`Your short-link ID is: ${message.short_link}, ${a_tag}`
		);
	}
}

function change_user_form(type) {
	if (type === "login") {
		modal_title.textContent = "Login with your account";

		user_register_form.className = "hidden";
		user_login_form.className = "login-register-form";
	} else {
		modal_title.textContent = "Register new user ";
		user_login_form.className = "hidden";
		user_register_form.className = "login-register-form";
	}
}

user_register_form.addEventListener("submit", register_function);
async function register_function(e) {
	e.preventDefault();
	const user = document.getElementById("register-username").value;
	const password = document.getElementById("register-password").value;

	const payload = {
		user_name: `${user}`,
		user_pwd: `${password}`,
	};

	let res = await fetch("/api/user/create/", {
		method: "POST",
		body: JSON.stringify(payload),
		headers: {
			"Content-Type": "application/json",
		},
		// mode: "cors",
	});
	const { error, message } = await res.json();
	if (!res.ok) {
		footer_msg.textContent = error;
		await delay(3000);
		footer_msg.textContent = "";
		user_login_form.reset();
	} else {
		footer_msg.textContent = message;
		await delay(3000);
		user_login_form.reset();
		handle_close(e);
	}

	user_register_form.reset();
}

user_login_form.addEventListener("submit", login_function);
/** Sends login / register info to the server */
async function login_function(e) {
	e.preventDefault();

	const user = document.getElementById("login-username").value;
	const password = document.getElementById("login-password").value;

	const payload = {
		user_name: user,
		user_pwd: password,
	};

	let res = await fetch("/api/user/login/", {
		method: "POST",
		body: JSON.stringify(payload),
		headers: {
			"Content-Type": "application/json",
		},
		mode: "cors",
	});
	const { error, message } = await res.json();
	if (!res.ok) {
		footer_msg.textContent = error;
		await delay(3000);
		footer_msg.textContent = "";
		user_login_form.reset();
	} else {
		footer_msg.textContent = message;
		await delay(3000);
		user_login_form.reset();
		handle_close(e);
	}
}
