function add_customer(evt) {
  const f_name = evt.children[0].value;
  const l_name = evt.children[1].value;
  const email = evt.children[2].value;

  fetch(url_customers, {
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
