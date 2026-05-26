#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub struct Installment {
    pub parent: Address,
    pub school: Address,
    pub total_paid: i128,
    pub completed: bool,
}

#[contracttype]
pub enum DataKey {
    Payment(Address),
}

#[contract]
pub struct TuitionInstallmentPayContract;

#[contractimpl]
impl TuitionInstallmentPayContract {

    // Parent sends installment payment
    pub fn pay_installment(
        env: Env,
        parent: Address,
        school: Address,
        amount: i128,
    ) {
        parent.require_auth();

        let key = DataKey::Payment(parent.clone());

        let mut installment = env.storage().instance().get(&key).unwrap_or(
            Installment {
                parent: parent.clone(),
                school: school.clone(),
                total_paid: 0,
                completed: false,
            }
        );

        installment.total_paid += amount;

        // Mark completed if tuition reaches target
        if installment.total_paid >= 10000 {
            installment.completed = true;
        }

        env.storage().instance().set(&key, &installment);
    }

    // Get payment information
    pub fn get_payment(
        env: Env,
        parent: Address,
    ) -> Installment {
        env.storage()
            .instance()
            .get(&DataKey::Payment(parent))
            .unwrap()
    }
}