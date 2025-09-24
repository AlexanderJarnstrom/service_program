async function show_customer_edit(cid) {
  const edit_panel = document.getElementById("edit-panel-customer")
  const customer = await get_customer(cid);

  edit_panel.querySelector("#f-f-name").value = customer["f_name"];
  edit_panel.querySelector("#f-l-name").value = customer["l_name"];
  edit_panel.querySelector("#f-email").value = customer["email"];

  edit_panel.querySelector("form").cid = cid;

  edit_panel.style.display = "inline";
}

async function customer_machine_content(cid) {
  const customer_machines_panel = document.getElementById("customer-machines-panel");
  while (customer_machines_panel.hasChildNodes()) {
    customer_machines_panel.removeChild(customer_machines_panel.firstChild);
  }

  const machines = await get_customer_machines(cid);
  const size = machines["length"];

  for (let i = 0; i < size; i++) {
    let address = machines[i]["address"];
    let next_service = machines[i]["next_service"];
    let last_service = machines[i]["last_service"];
    let id = machines[i]["machine_id"];

    let service = ((next_service == null) ? "" : "Next:" + next_service)
    service = service + ((last_service == null) ? "" : "Last:" + last_service)

    create_machine_card(customer_machines_panel, address, service, id);
  }
}

async function create_content() {
  const customers = await get_customers();
  const customer_panel = document.getElementById("customer-panel");
  const size = customers["length"];

  for (let i = 0; i < size; i++) {
    let first_name = customers[i]["f_name"];
    let last_name = customers[i]["l_name"];
    let email = customers[i]["email"];
    let cid = customers[i]["customer_id"];

    create_customer_card(customer_panel, first_name, last_name, email, cid);
  } 
}

