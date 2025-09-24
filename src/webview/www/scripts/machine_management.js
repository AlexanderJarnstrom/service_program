async function delete_machine(id) {
  let url = url_machines + "/" + id;

  const resp = await fetch(
    url,
    {method: 'DELETE'}
  );

  if (!resp.ok) {
    console.log("Failed to delete machine.");
  }

  location.reload();
}

async function mark_done(id) {
  let url = url_machines + "/done/" + id; 

  const resp = await fetch(
    url,
    {method: "PATCH"}
  );

  if (!resp.ok) {
    console.log("Failed to mark as done.");
  }

  location.reload();
}

async function edit_machine(form) {
  const address = form.querySelector("#f-address").value;
  const customer = form.querySelector("#customer-select").value;
  const next_service = form.querySelector("#f-next").value;
  const last_service = form.querySelector("#f-last").value;

  await fetch(url_machines, {
    method: "PUT",
    body: JSON.stringify({
      machine_id: form.mid,
      address: address,
      customer_id: customer,
      next_service: ((next_service == "") ? null : next_service),
      last_service: ((last_service == "") ? null : last_service)
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    }
  });
}

async function get_machine(mid) {
  let resp = await fetch(url_machines + "/" + mid)

  if (!resp.ok) {
    let error = "Could not get machine.";
    console.error(error);
    alert(error);
  }
  
  return await resp.json();
}

async function add_machine(form) {
  const address = form.querySelector("#f-address").value;
  const customer = form.querySelector("#customer-select").value;
  const next_service = form.querySelector("#f-next").value;
  const last_service = form.querySelector("#f-last").value;

  fetch(url_machines, {
    method: "POST",
    body: JSON.stringify({
      address: address,
      customer_id: customer,
      next_service: ((next_service == "") ? null : next_service),
      last_service: ((last_service == "") ? null : last_service)
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    }
  });
}

async function get_customer_machines(cid) {
  const response = await fetch(url_machines + "/customer/" + cid)
  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  return await response.json();
}

async function get_machines() {
  const response = await fetch(url_machines)
  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  return await response.json();
}

function create_machine_card(panel, title, subtitle, id) {
  const card = document.createElement("div");
  const card_content = document.createElement("div");
  const card_address_field = document.createElement("b");
  const card_date_field = document.createElement("p");
  const card_del_button = document.createElement("button");
  const card_edit_button = document.createElement("button");
  const card_done_button = document.createElement("button");

  card_address_field.innerText = title;
  card_address_field.className = "title";

  card_date_field.innerText = subtitle;
  card_date_field.className = "subtitle";

  card_del_button.className = "remove";
  card_del_button.type = "button";
  card_del_button.addEventListener("click", function () {delete_machine(id);});
  card_del_button.innerText = "✖";

  card_done_button.className = "done";
  card_done_button.type = "button";
  card_done_button.addEventListener("click", function () {mark_done(id);});
  card_done_button.innerText = "✔";

  card_edit_button.className = "edit";
  card_edit_button.type = "button";
  card_edit_button.addEventListener("click", function () {show_machine_edit(id);});
  card_edit_button.innerText = "✎";

  card_content.className = "card-content normal"

  card.className = "card"

  card_content.appendChild(card_address_field);
  card_content.appendChild(card_date_field);
  card_content.appendChild(card_del_button);
  card_content.appendChild(card_edit_button);
  card_content.appendChild(card_done_button);
  card.appendChild(card_content);
  panel.appendChild(card);
}


