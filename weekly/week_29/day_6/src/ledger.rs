#[derive(Debug)]
pub struct Account {
    pub id: u32,
    // Le solde peut être négatif, mais une transaction ne peut pas le rendre tel.
    pub balance: i64,
}

pub struct Ledger {
    // Le livre peut contenir jusqu'à 16 comptes.
    pub accounts: heapless::Vec<Account, 16>,
}

pub enum LedgerError {
    UnvalidDestination,
    NegativeSold,
    AccountNotFound,
}

impl Ledger {
    pub fn execute_transaction(
        &mut self,
        from_id: u32,
        to_id: u32,
        // Le montant est toujours positif.
        amount: u64,
    ) -> Result<(), LedgerError> {
        if from_id == to_id {
            return Err(LedgerError::UnvalidDestination);
        }

        //GETTING POSITION
        let from_account = self.accounts.iter().position(|e| e.id == from_id);
        let to_account = self.accounts.iter().position(|e| e.id == to_id);

        
        if let (Some(from_index), Some(to_index)) = (from_account, to_account) {
            if from_index == to_index {
                return Err(LedgerError::UnvalidDestination);
            }

            //ORDERING INDEX
            let (i, j) = if from_index < to_index {
                (from_index, to_index)
            } else {
                (to_index, from_index)
            };

            //SLICE ISOLATION
            let (slice_1, slice_2) = self.accounts.split_at_mut(j);

            let acc1 = &mut slice_1[i];
            let acc2 = &mut slice_2[0];

            let (from, to) = if acc1.id == from_id {
                (acc1, acc2)
            } else {
                (acc2, acc1)
            };


            //ATOMIC BALANCE CONDITION
            if from.balance >= amount as i64 {
                from.balance -= amount as i64;
                to.balance += amount as i64;
                Ok(())
            } else {
                Err(LedgerError::NegativeSold)
            }
        } else {
            Err(LedgerError::AccountNotFound)
        }
    }
}
