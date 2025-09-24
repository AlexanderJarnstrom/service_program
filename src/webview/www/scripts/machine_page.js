async function show_machine_edit(mid) {
  const edit_panel = document.getElementById("edit-panel-machine")
  const machine = await get_machine(mid);
  const customers = await get_customers();
  const size = customers["length"];

  const select = edit_panel.querySelector("#customer-select");
 
  for (let i = 0; i < size; i++) {
    const cid = customers[i]["customer_id"];
    const f_name = customers[i]["f_name"];
    const l_name = customers[i]["l_name"];
    const op = document.createElement("option");
    op.value = cid;
    op.innerText = f_name + " " + l_name;
    select.appendChild(op);
  }

  edit_panel.querySelector("#f-address").value = machine["address"];
  select.value = machine["customer_id"];
  edit_panel.querySelector("#f-next").value = machine["next_service"];
  edit_panel.querySelector("#f-last").value = machine["last_service"];

  edit_panel.querySelector("form").mid = mid;

  edit_panel.style.display = "inline";
}

async function load_machine_list() {
  const machines = await get_machines();
  const size = machines["length"];
  const machine_panel = document.getElementById("machines-panel");

  for (let i = 0; i < size; i++) {
    let address = machines[i]["address"];
    let next_service = machines[i]["next_service"];
    let last_service = machines[i]["last_service"];
    let id = machines[i]["machine_id"];
    let cid = machines[i]["customer_id"];

    let customer = await get_customer(cid);

    let name = customer["f_name"] + " " + customer["l_name"];
    let info = address + " | " + name;

    let service = ((next_service == null) ? "" : "Next:" + next_service)
    service = service + ((last_service == null) ? "" : "Last:" + last_service)

    create_machine_card(machine_panel, info, service, id);
  }
}

async function init_form() {
  const customers = await get_customers();
  const size = customers["length"];
  const select = document.getElementById("customer-select");
 
  for (let i = 0; i < size; i++) {
    const cid = customers[i]["customer_id"];
    const f_name = customers[i]["f_name"];
    const l_name = customers[i]["l_name"];
    const op = document.createElement("option");
    op.value = cid;
    op.innerText = f_name + " " + l_name;
    select.appendChild(op);
  }

  const machines = get_machines();
}

