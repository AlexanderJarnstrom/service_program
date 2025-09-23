function get_domain() {
  const host = window.location.protocol + "//" + window.location.host;
  return host;
}

const url_customers = get_domain() + "/api/customers";
const url_machines = get_domain() + "/api/machines";
