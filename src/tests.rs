use crate::transaction::TxType;

use super::*;

#[test]
fn test_account_deposit() {
    let mut account = Account::new(1);
    let tx = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 1,
        amount: Some(2.0),
    };
    account.add_transaction(tx);
    assert_eq!(0.0, account.held);
    assert_eq!(2.0, account.available);
    assert_eq!(2.0, account.total);
    assert!(!account.locked);
}

#[test]
fn test_account_withdraw() {
    let mut account = Account::new(1);
    let deposit = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 1,
        amount: Some(2.0),
    };
    let withdraw = Transaction {
        tx_type: TxType::Withdrawal,
        client: 1,
        tx: 1,
        amount: Some(1.0),
    };
    account.add_transaction(deposit);
    account.add_transaction(withdraw);
    assert_eq!(0.0, account.held);
    assert_eq!(1.0, account.available);
    assert_eq!(1.0, account.total);
    assert!(!account.locked);
}

#[test]
fn test_account_withdraw_bad() {
    let mut account = Account::new(1);
    let withdraw = Transaction {
        tx_type: TxType::Withdrawal,
        client: 1,
        tx: 1,
        amount: Some(1.0),
    };
    account.add_transaction(withdraw);
    assert_eq!(0.0, account.held);
    assert_eq!(0.0, account.available);
    assert_eq!(0.0, account.total);
    assert!(!account.locked);
}

#[test]
fn test_account_dispute() {
    let mut account = Account::new(1);
    let deposit_one = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 1,
        amount: Some(2.0),
    };
    let deposit_two = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 2,
        amount: Some(1.0),
    };
    let dispute = Transaction {
        tx_type: TxType::Dispute,
        client: 1,
        tx: 1,
        amount: None,
    };
    account.add_transaction(deposit_one);
    account.add_transaction(deposit_two);
    account.add_transaction(dispute);
    assert_eq!(2.0, account.held);
    assert_eq!(1.0, account.available);
    assert_eq!(3.0, account.total);
    assert!(!account.locked);
}

#[test]
fn test_account_dispute_bad() {
    let mut account = Account::new(1);
    let deposit = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 1,
        amount: Some(3.0),
    };
    let dispute = Transaction {
        tx_type: TxType::Dispute,
        client: 1,
        tx: 2,
        amount: None,
    };
    account.add_transaction(deposit);
    account.add_transaction(dispute);
    assert_eq!(0.0, account.held);
    assert_eq!(3.0, account.available);
    assert_eq!(3.0, account.total);
    assert!(!account.locked);
}

#[test]
fn test_account_resolved() {
    let mut account = Account::new(1);
    let deposit = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 2,
        amount: Some(2.0),
    };
    let dispute = Transaction {
        tx_type: TxType::Dispute,
        client: 1,
        tx: 2,
        amount: None,
    };
    let resolve = Transaction {
        tx_type: TxType::Resolve,
        client: 1,
        tx: 2,
        amount: None,
    };
    account.add_transaction(deposit);
    account.add_transaction(dispute);
    account.add_transaction(resolve);
    assert_eq!(0.0, account.held);
    assert_eq!(2.0, account.available);
    assert_eq!(2.0, account.total);
    assert!(!account.locked);
}

#[test]
fn test_account_resolved_bad() {
    let mut account = Account::new(1);
    let deposit = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 2,
        amount: Some(2.0),
    };
    let dispute = Transaction {
        tx_type: TxType::Dispute,
        client: 1,
        tx: 2,
        amount: None,
    };
    let resolve = Transaction {
        tx_type: TxType::Resolve,
        client: 1,
        tx: 12,
        amount: None,
    };
    account.add_transaction(deposit);
    account.add_transaction(dispute);
    account.add_transaction(resolve);
    assert_eq!(2.0, account.held);
    assert_eq!(0.0, account.available);
    assert_eq!(2.0, account.total);
    assert!(!account.locked);
}

#[test]
fn test_account_chargeback() {
    let mut account = Account::new(1);
    let deposit = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 2,
        amount: Some(2.0),
    };
    let dispute = Transaction {
        tx_type: TxType::Dispute,
        client: 1,
        tx: 2,
        amount: None,
    };
    let chargeback = Transaction {
        tx_type: TxType::Chargeback,
        client: 1,
        tx: 2,
        amount: None,
    };
    account.add_transaction(deposit);
    account.add_transaction(dispute);
    account.add_transaction(chargeback);
    assert_eq!(0.0, account.held);
    assert_eq!(0.0, account.available);
    assert_eq!(0.0, account.total);
    assert!(account.locked);
}

#[test]
fn test_account_chargeback_bad() {
    let mut account = Account::new(1);
    let deposit = Transaction {
        tx_type: TxType::Deposit,
        client: 1,
        tx: 2,
        amount: Some(2.0),
    };
    let dispute = Transaction {
        tx_type: TxType::Dispute,
        client: 1,
        tx: 2,
        amount: None,
    };
    let chargeback = Transaction {
        tx_type: TxType::Chargeback,
        client: 1,
        tx: 22,
        amount: None,
    };
    account.add_transaction(deposit);
    account.add_transaction(dispute);
    account.add_transaction(chargeback);
    assert_eq!(2.0, account.held);
    assert_eq!(0.0, account.available);
    assert_eq!(2.0, account.total);
    assert!(!account.locked);
}
