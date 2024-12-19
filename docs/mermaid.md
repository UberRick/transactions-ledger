flowchart TD
Start(["Start"]) --> Import["Import Transactions"]
Import --> Loop{"For each transaction in list"}
Loop --> Find["Find/Create Account for Account ID"] & End(["End"])
Find --> CheckLock{"Is Account Locked?"}
CheckLock -- Yes --> Loop
CheckLock -- No --> Match["Match Transaction Type"]
Match --> Deposit["Deposit?"] & Withdrawal["Withdrawal?"] & Dispute["Dispute?"] & Resolve["Resolve?"] & Chargeback["Chargeback?"]
Deposit --> IncDeposit["Increase Available & Total Funds"]
Withdrawal --> CheckFunds{"Available Funds >= Withdrawal Amount?"}
CheckFunds -- No --> Loop
CheckFunds -- Yes --> DecWithdrawal["Decrease Available & Total Funds"]
Dispute --> FindTxDispute["Find Transaction by Tx"]
Resolve --> FindTxResolve["Is Disputed & Find Transaction by Tx"]
Chargeback --> FindTxChargeback["Is Disputed & Find Transaction by Tx"]
FindTxDispute -- Transaction Found --> DisputeUpdates["Decrease Available Funds & Increase Held Funds"]
FindTxDispute -- Transaction not found --> Loop
FindTxResolve -- Transaction Found --> ResolveUpdates["Increase Available Funds & Decrease Held Funds"]
FindTxResolve -- No Dispute found or Transaction not found --> Loop
FindTxChargeback -- Transaction Found --> ChargebackUpdates["Decrease Held Funds & Decrease Total Funds & Lock Account"]
FindTxChargeback -- No Dispute found or Transaction not found --> Loop
IncDeposit --> Log["Store Transaction"]
Log --> Update["Update Account"]
DecWithdrawal --> Update
DisputeUpdates --> Update
ResolveUpdates --> Update
ChargebackUpdates --> Update
Update --> Loop
