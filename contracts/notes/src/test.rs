#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    Address, Env,
};

use crate::{
    TuitionInstallmentPayContract,
    TuitionInstallmentPayContractClient,
};

#[test]
fn test_happy_path() {
    let env = Env::default();

    let contract_id = env.register(
        TuitionInstallmentPayContract,
        ()
    );

    let client =
        TuitionInstallmentPayContractClient::new(
            &env,
            &contract_id,
        );

    let parent = Address::generate(&env);
    let school = Address::generate(&env);

    client.pay_installment(
        &parent,
        &school,
        &5000,
    );

    let payment =
        client.get_payment(&parent);

    assert_eq!(payment.total_paid, 5000);
}

#[test]
fn test_multiple_installments() {
    let env = Env::default();

    let contract_id = env.register(
        TuitionInstallmentPayContract,
        ()
    );

    let client =
        TuitionInstallmentPayContractClient::new(
            &env,
            &contract_id,
        );

    let parent = Address::generate(&env);
    let school = Address::generate(&env);

    client.pay_installment(
        &parent,
        &school,
        &4000,
    );

    client.pay_installment(
        &parent,
        &school,
        &6000,
    );

    let payment =
        client.get_payment(&parent);

    assert_eq!(payment.completed, true);
}

#[test]
fn test_state_verification() {
    let env = Env::default();

    let contract_id = env.register(
        TuitionInstallmentPayContract,
        ()
    );

    let client =
        TuitionInstallmentPayContractClient::new(
            &env,
            &contract_id,
        );

    let parent = Address::generate(&env);
    let school = Address::generate(&env);

    client.pay_installment(
        &parent,
        &school,
        &3000,
    );

    let payment =
        client.get_payment(&parent);

    assert_eq!(payment.completed, false);
}

#[test]
#[should_panic]
fn test_missing_record() {
    let env = Env::default();

    let contract_id = env.register(
        TuitionInstallmentPayContract,
        ()
    );

    let client =
        TuitionInstallmentPayContractClient::new(
            &env,
            &contract_id,
        );

    let parent = Address::generate(&env);

    client.get_payment(&parent);
}

#[test]
fn test_exact_completion_amount() {
    let env = Env::default();

    let contract_id = env.register(
        TuitionInstallmentPayContract,
        ()
    );

    let client =
        TuitionInstallmentPayContractClient::new(
            &env,
            &contract_id,
        );

    let parent = Address::generate(&env);
    let school = Address::generate(&env);

    client.pay_installment(
        &parent,
        &school,
        &10000,
    );

    let payment =
        client.get_payment(&parent);

    assert_eq!(payment.completed, true);
}