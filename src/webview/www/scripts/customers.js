async function get_customers() {
  console.log(url_customers);
  const response = await fetch(url_customers);
  if (!response.ok) {
    throw new Error(`Response status: ${response.status}`);
  }

  return await response.json();
}

async function create_content() {
  const customers = await get_customers();
  const customer_panel = document.getElementById("customer-panel");
  const size = customers["length"];

  console.log(customers);

  for (let i = 0; i < size; i++) {
    let first_name = customers[i]["f_name"];
    let last_name = customers[i]["l_name"];
    let email = customers[i]["email"];
    let cid = customers[i]["customer_id"];

    create_customer_card(customer_panel, first_name, last_name, email, cid);
  } 
}

create_content();
