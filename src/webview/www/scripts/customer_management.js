function create_customer_card(panel, first_name, last_name, email, cid) {
  const card = document.createElement("div");
  const card_content = document.createElement("div");
  const card_name_field = document.createElement("b");
  const card_email_field = document.createElement("p");
  const card_del_button = document.createElement("button");
  const card_edit_button = document.createElement("button");

  card_name_field.innerText = first_name + " " + last_name;
  card_name_field.className = "title";
  card_name_field.addEventListener("click", function () {customer_machine_content(cid);});

  card_email_field.innerText = ((email == null) ? "" : email);
  card_email_field.className = "subtitle";

  card_del_button.className = "remove";
  card_del_button.type = "button";
  card_del_button.addEventListener("click", function () {delete_customer(cid);});
  card_del_button.innerText = "✖";

  card_edit_button.className = "edit";
  card_edit_button.type = "button";
  card_edit_button.addEventListener("click", function () {show_customer_edit(cid);});
  card_edit_button.innerText = "✎";

  card_content.className = "card-content"

  card.className = "card"

  card_content.appendChild(card_name_field);
  card_content.appendChild(card_email_field);
  card_content.appendChild(card_del_button);
  card_content.appendChild(card_edit_button);
  card.appendChild(card_content);
  panel.appendChild(card);
}

async function delete_customer(cid) {
  let resp = await fetch(url_machines + "/customer/" + cid);
  if (!resp.ok) {
    throw new error("Could not get machines.");
  }

  const machines = await resp.json();
  const size = machines["length"];

  for (let i = 0; i < size; i++) {
    await delete_machine(machines[i]["machine_id"]);
  }

  resp = await fetch(url_customers + "/" + cid, {
    method: "DELETE"
  });

  if (!resp.ok) {
    throw new error("Could not delete customer.");
  }

  location.reload();
}

async function edit_customer(form) {
  const f_name = form.querySelector("#f-f-name").value;
  const l_name = form.querySelector("#f-l-name").value;
  const email = form.querySelector("#f-email").value;

  await fetch(url_customers, {
    method: "PUT",
    body: JSON.stringify({
      customer_id: form.cid,
      f_name: f_name,
      l_name: l_name,
      email: email
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    }
  });
}


async function add_customer(evt) {
  const f_name = evt.children[0].value;
  const l_name = evt.children[1].value;
  const email = evt.children[2].value;

  await fetch(url_customers, {
    method: "POST",
    body: JSON.stringify({
      f_name: f_name,
      l_name: l_name,
      email: email
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    }
  });
}

async function get_customers() {
  console.log(url_customers);
  const response = await fetch(url_customers);
  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  return await response.json();
}

async function get_customer(cid) {
  const response = await fetch(url_customers + "/" + cid);
  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  return await response.json();
}
