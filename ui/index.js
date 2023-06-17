const create_link_form = document.getElementById("submit-long-link");
const modal = document.getElementById("modal");
const close_modal_button = document.getElementById("close-modal");
const modal_title = document.getElementById("modal-title");
const modal_text = document.getElementById("modal-text");
const link_redirect = document.getElementById("link_redirect");

create_link_form.addEventListener("submit", create_link);

function close_modal() {
	modal.close();
	// removes close class if present
	modal.classList.remove("close");
	// removes the animation event listener, modal was closed
	modal.removeEventListener("animationend", close_modal);
}

// if user clicks X btn inside modal
close_modal_button.addEventListener("click", (e) => {
	// run close_modal
	modal.addEventListener("animationend", close_modal);
	// add "close" class to <dialog> (this will trigger CSS animations)
	modal.classList.add("close");
	e.preventDefault();
});

create_link_form.addEventListener("submit", create_link);

/// modal.show() will replace the <dialog> element with <dialog open>
async function create_modal(type, title, message) {
	// removing classes from modal / modal_button
	modal.className = "";
	close_modal_button.className = "";

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
			modal.classList.add("error");
			close_modal_button.classList.add("error");

			modal_text.textContent = "Error!";
			modal_title.textContent = "Invalid modal type";

			break;
	}
}

/// main function to handle form that will be sent to server
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
	const { error, data } = short_link;

	if (!res.ok) {
		await create_modal("err", "Error!", error);
	} else {
		const host = window.location.origin;
		const a_tag = `<a target='_blank' href=${host}/${data.short_link}>click here to open it!</a>`;

		await create_modal(
			"ok",
			"Done!",
			`Your short-link ID is: ${data.short_link}, ${a_tag}`
		);
	}
}
