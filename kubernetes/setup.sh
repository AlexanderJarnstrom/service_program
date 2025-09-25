#!/bin/sh

minikube kubectl -- apply -f sp_kubernetes.yaml

minikube kubectl -- create secret generic sp-credentials \
    --from-literal=postgres-password="secret_password" \
    --from-literal=customer-username="sp_customer" \
    --from-literal=customer-password="secret_password" \
    --from-literal=machine-username="sp_machine" \
    --from-literal=machine-password="secret_password"

minikube kubectl -- apply -f database_kubernetes.yaml
minikube kubectl -- apply -f customer_kubernetes.yaml
minikube kubectl -- apply -f machine_kubernetes.yaml
minikube kubectl -- apply -f webview_kubernetes.yaml
