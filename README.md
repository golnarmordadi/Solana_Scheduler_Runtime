# Solana_Scheduler_Runtime

Creating a scheduler and runtime for Solana in Rust involves understanding how to schedule and execute tasks on the Solana blockchain. Solana's runtime is designed for high throughput and low-latency transaction processing, leveraging a Proof of History (PoH) consensus mechanism. Below is an example of setting up a basic scheduler and runtime for executing tasks on Solana.

## Prerequisites

Ensure you have the following installed:

Rust and Cargo: [rustup.rs](https://rustup.rs/)
Solana CLI: [Solana CLI Installation](https://docs.solanalabs.com/cli/install)

## Explanation

Dependencies:

`solana-client` and `solana-program` are used to interact with the Solana blockchain.
tokio is used for asynchronous runtime and scheduling.

* send_transaction Function:

This function creates and sends a transaction from the payer to the recipient with a specified amount.

* main Function:

The main function initializes the `RPC` client, sets up the payer and recipient, and uses a tokio interval to schedule the `send_transaction` function to run every 10 seconds.

## Running the Project

1- Build the Project:

```sh
cargo build
```

2- Run the Project:

```sh
cargo run
```

## Conclusion

This example provides a basic framework for creating a scheduler and runtime for Solana in Rust. The scheduler uses tokio for asynchronous execution, and the runtime submits transactions to the Solana blockchain at regular intervals. This setup can be extended to include more complex scheduling and task execution logic as needed for your specific use case.
