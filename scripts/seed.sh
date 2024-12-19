#!/bin/bash

# Configuration
OUTPUT_FILE="transactions.csv"
NUM_TRANSACTIONS=10000
MAX_CLIENTS=1000
MAX_AMOUNT=1000

# Initialize random seed
RANDOM=$$

# Generate random transaction
generate_transaction() {
    local tx_id=$1
    local client_id=$((RANDOM % MAX_CLIENTS + 1))
    local amount=$(printf "%.2f" "$(awk "BEGIN {print ($RANDOM % $MAX_AMOUNT) / 100}")")
    
    case $((RANDOM % 5)) in
        0)
            # Deposit
            echo "deposit,$client_id,$tx_id,$amount"
            ;;
        1)
            # Withdrawal
            echo "withdrawal,$client_id,$tx_id,$amount"
            ;;
        2)
            # Dispute (referencing a random previous tx_id)
            local ref_tx_id=$((RANDOM % tx_id + 1))
            echo "dispute,$client_id,$ref_tx_id,"
            ;;
        3)
            # Resolve (referencing a random previous tx_id)
            local ref_tx_id=$((RANDOM % tx_id + 1))
            echo "resolve,$client_id,$ref_tx_id,"
            ;;
        4)
            # Chargeback (referencing a random previous tx_id)
            local ref_tx_id=$((RANDOM % tx_id + 1))
            echo "chargeback,$client_id,$ref_tx_id,"
            ;;
    esac
}

# Header
echo "type,client,tx,amount" > $OUTPUT_FILE

# Generate transactions
for ((tx_id=1; tx_id<=NUM_TRANSACTIONS; tx_id++)); do
    generate_transaction $tx_id >> $OUTPUT_FILE
done

echo "Generated $NUM_TRANSACTIONS transactions in $OUTPUT_FILE."
