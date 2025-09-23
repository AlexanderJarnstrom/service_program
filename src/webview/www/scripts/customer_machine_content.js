function create_machine_card(panel, address, next_service, last_service) {
  const card = document.createElement("div");
  const card_content = document.createElement("div");
  const card_address_field = document.createElement("b");
  const card_date_field = document.createElement("p");

  card_address_field.innerText = address;
  let date_field = "";
  date_field = date_field + ((next_service == null) ? "" : "Next service: " + next_service);
  date_field = date_field + ((last_service == null) ? "" : "Last service: " + last_service);
  card_date_field.innerText = date_field;
  card_content.className = "card-content normal"
  card.className = "card"

  card_content.appendChild(card_address_field);
  card_content.appendChild(card_date_field);
  card.appendChild(card_content);
  panel.appendChild(card);
}

async function get_machines(url) {
  const response = await fetch(url)
  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  return await response.json();
}

async function customer_machine_content(cid) {

  const customer_machines_panel = document.getElementById("customer-machines-panel");
  while (customer_machines_panel.hasChildNodes()) {
    customer_machines_panel.removeChild(customer_machines_panel.firstChild);
  }

  const url = url_machines + "/" + cid;
  const machines = await get_machines(url);
  const size = machines["length"];

  console.log(machines);

  for (let i = 0; i < size; i++) {
    let address = machines[i]["address"];
    let next_service = machines[i]["next_service"];
    let last_service = machines[i]["last_service"];

    create_machine_card(customer_machines_panel, address, next_service, last_service);
  }
}
