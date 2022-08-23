Application handles all transaction types [Deposit, Withdraw, Dispute, Resolve, Chargeback]
I assured correctness by testing with sample data (included) and writing unit tests.

# How to use

1.  `cargo run -- [absolute path to csv]`
2.  Using relative path. Place the csv in the root of this repo. `cargo run -- test_data.csv`
