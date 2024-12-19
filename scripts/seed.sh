#!/bin/bash

FILE="transactions.csv"

echo "type,client,tx,amount" > "$FILE"
echo "deposit,1,1,1.0" >> "$FILE"
echo "deposit,2,2,2.0" >> "$FILE"
echo "deposit,1,3,2.0" >> "$FILE"
echo "withdrawal,1,4,1.5" >> "$FILE"
echo "withdrawal,2,5,3.0" >> "$FILE"
echo "dispute,2,2," >> "$FILE"
echo "chargeback,2,2," >> "$FILE"
echo "dispute,1,1," >> "$FILE"
echo "resolve,1,1," >> "$FILE"

# Notify the user
echo "Seed data written to $FILE"
