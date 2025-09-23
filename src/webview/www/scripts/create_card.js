function create_customer_card(panel, first_name, last_name, email, cid) {
  const card = document.createElement("div");
  const card_content = document.createElement("div");
  const card_name_field = document.createElement("b");
  const card_email_field = document.createElement("p");

  card_name_field.innerText = first_name + " " + last_name;
  card_email_field.innerText = ((email == null) ? "" : email);
  card_content.className = "card-content"
  card.className = "card"
  card.addEventListener("click", function () {customer_machine_content(cid);});

  card_content.appendChild(card_name_field);
  card_content.appendChild(card_email_field);
  card.appendChild(card_content);
  panel.appendChild(card);
}

