# Setup

## Kubernetes setup

Edit the credentials to your liking in the `kubernetes/setup.sh` file:

```bash
kubectl create secret generic sp-credentials \
    --from-literal=postgres-password="secret_password" \
    --from-literal=customer-username="sp_customer" \
    --from-literal=customer-password="secret_password" \
    --from-literal=machine-username="sp_machine" \
    --from-literal=machine-password="secret_password"
```

if you're not using `minikube` remove all mentions of it from the startup script and the `--` between kubectl and its options.

## Docker setup

Edit the environment variables in the compose file located in the root folder with actual secrets, and run:

```bash
docker compose up
```

# Documents

These are the microservice specific documentations.

- [Customer API](./customer_api.md)
- [Machine API](./machine_api.md)
- [Database](./database.md)
- [WebView](./webview.md)

## Structure

![structure](./images/structure.png)
