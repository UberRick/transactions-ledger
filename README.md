# Description

Payments engine which reads a series of transactions from a CSV and produces the state of client accounts as a CSV.

## Usage

The payments engine takes an transactions.csv and returns the state of the client accounts which goes to STDOUT.

```sh
cargo run -- transactions.csv > accounts.csv
```

## Error handling

The current implementation of input parsing and deserialization into structs lacks robust error handling. As a result, any malformed or invalid input data could lead to an unrecoverable application failure.

## Scaling / Concurrency

The current implementation relys on a single client account transactions being processed by the same ledger.

Thoughts around scaling:

- The solution could be scaled using some sort hashing algo for example (modulo-based hashing). This could be done outside of the service in which the split would ensure a node processes the same set of client accounts. Further to this the transactions that a single node recieves could be split across many workers using tokio spawn utilizing the client id e.g. worker_index = client_id % num_workers.

## Efficiency

- Input transaction data is streamed allowing each transaction to be processed without loading all the transactions into memory.
- There is opportunity for further optimisation such as removing deposits after a resolve or chargeback
- Using valgrind to profile the service using massif showed a significant drop in memory before and after switching to streaming the transactions.

## Assumptions

- Dispute, Resolve and Chargeback only relate to a deposit transaction
- Client id of Dispute, Resolve or Chargeback tx should match the client id of the tx they reference otherwise ignore

## Ledger Flow Diagram

![](./docs/diagram.png)

## Security

No vulnerabilities found using `cargo audit`

## Improvements with more time

- Currently using hashmap for managing the state of accounts, deposits and disputes. It may have been a good idea to abstract the in memory data stores to a repository with a common interface that would have allowed swapping the hashmap with a database integration without needing to change the ledger implementation at a later date.
- Creating custom errors which provide more context and make it easier to debug if things were to go wrong
