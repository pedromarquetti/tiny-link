const create_link_form = document.getElementById("submit-long-link");
const close_modal = document.querySelector(".close-modal");
const modal = document.querySelector(".modal");
const modal_title = document.querySelector(".modal-title");
const modal_text = document.querySelector(".modal-text");

create_link_form.addEventListener("submit", create_link);

close_modal.addEventListener("click", () => {
	modal.classList.remove("fail");
	modal.classList.remove("ok");
	modal.classList.add("hidden");
});

async function create_modal(type, title, message) {
	modal.classList.remove("fail");
	modal.classList.remove("ok");
	if (type === "err") {
		modal_title.textContent = title;
		modal_text.textContent = message;
		modal.classList.add("fail");
		modal.classList.remove("hidden");
	} else {
		modal_title.textContent = title;
		modal_text.textContent = message;
		modal.classList.add("ok");
		modal.classList.remove("hidden");
	}
}

async function create_link(e) {
	e.preventDefault();

	let long_link = e.target[0].value;
	let payload = { long_url: long_link };
	let res = await fetch("/api/create/", {
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
		await create_modal(
			"ok",
			"Done!",
			`Your short-link ID is: ${data.short_link}`
		);
	}

	console.log(short_link);
}
