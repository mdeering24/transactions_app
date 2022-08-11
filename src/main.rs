use std::{collections::HashMap, fs::File};

use crate::{account::Account, transaction::Transaction};
use ::csv::{DeserializeRecordsIntoIter, Reader};

mod account;
mod csv;
#[cfg(test)]
mod tests;
mod transaction;

fn main() {
    let reader: Reader<File> = csv::import_csv_data();
    let csv_iter: DeserializeRecordsIntoIter<File, Transaction> = reader.into_deserialize();

    let mut client_accounts: HashMap<u16, Account> = HashMap::new();

    for tx in csv_iter.flatten() {
        if let Some(account) = client_accounts.get_mut(&tx.client) {
            account.add_transaction(tx);
        } else {
            let mut account = Account::new(tx.client);
            account.add_transaction(tx);
            client_accounts.insert(account.client, account);
        }
    }

    println!("client,available,held,total,locked");

    for a in client_accounts.values() {
        println!(
            "{},{},{},{},{}",
            a.client, a.available, a.held, a.total, a.locked
        );
    }
}
