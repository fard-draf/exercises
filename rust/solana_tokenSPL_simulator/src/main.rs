pub mod error;
use crate::error::PackError;

use std::mem;

/// Trait pour la sérialisation binaire compatible Solana
pub trait BitPackable {
    const PACKED_SIZE: usize;

    fn pack(&self) -> Vec<u8>;
    fn pack_into(&self, dst: &mut [u8]) -> Result<(), PackError>;
    fn unpack(data: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized;
}

/// États possibles d'un token account (comme dans SPL Token)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AccountState {
    Uninitialized = 0,
    Initialized = 1,
    Frozen = 2,
}

impl TryFrom<u8> for AccountState {
    type Error = PackError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(AccountState::Uninitialized),
            1 => Ok(AccountState::Initialized),
            2 => Ok(AccountState::Frozen),
            _ => Err(PackError::InvalidState(value)),
        }
    }
}

/// Simulation d'une Pubkey Solana (32 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pubkey([u8; 32]);

impl Pubkey {
    pub fn default() -> Self {
        Self([8u8; 32])
    }

    pub const fn new_from_array(arr: [u8; 32]) -> Self {
        Self(arr)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub const ZERO: Self = Self([0; 32]);
}

/// Structure principale : Token Account (équivalent SPL Token)
/// Taille exacte : 165 bytes comme dans Solana
#[derive(Debug, Clone)]
pub struct TokenAccount {
    /// Mint du token (32 bytes)
    pub mint: Pubkey,
    /// Propriétaire du compte (32 bytes)
    pub owner: Pubkey,
    /// Nombre de tokens (8 bytes)
    pub amount: u64,
    /// Délégué optionnel (33 bytes : 1 flag + 32 pubkey)
    pub delegate: Option<Pubkey>,
    /// État du compte (1 byte)
    pub state: AccountState,
    /// Option de native token (9 bytes : 1 flag + 8 amount)
    pub is_native: Option<u64>,
    /// Montant délégué (8 bytes)
    pub delegated_amount: u64,
    /// Autorité de fermeture (33 bytes : 1 flag + 32 pubkey)
    pub close_authority: Option<Pubkey>,
}

impl TokenAccount {
    //MASK

    pub fn new(mint: Pubkey, owner: Pubkey) -> Self {
        Self {
            mint,
            owner,
            amount: 0,
            delegate: None,
            state: AccountState::Initialized,
            is_native: None,
            delegated_amount: 0,
            close_authority: None,
        }
    }

    /// Vérifie si le compte peut être utilisé
    pub fn is_frozen(&self) -> bool {
        self.state == AccountState::Frozen
    }

    /// Vérifie si une délégation est active
    pub fn has_active_delegation(&self) -> bool {
        self.delegate.is_some() && self.delegated_amount > 0
    }
}

// À IMPLÉMENTER : BitPackable pour TokenAccount
impl BitPackable for TokenAccount {
    // Taille exacte du SPL Token Program
    const PACKED_SIZE: usize = 165;

    fn pack(&self) -> Vec<u8> {
        let mut data = vec![0u8; Self::PACKED_SIZE];
        self.pack_into(&mut data).unwrap();
        data
    }

    fn pack_into(&self, dst: &mut [u8]) -> Result<(), PackError> {
        if dst.len() < Self::PACKED_SIZE {
            return Err(PackError::BufferTooSmall {
                expected: Self::PACKED_SIZE,
                actual: dst.len(),
            });
        }

        //===================================================================================START
        let mut offset = 0;
        //===================================================================================MINT
        dst[offset..offset + 32].copy_from_slice(self.mint.as_bytes());
        offset += 32;

        //===================================================================================OWNER
        dst[offset..offset + 32].copy_from_slice(self.owner.as_bytes());
        offset += 32;
        //===================================================================================AMOUNT
        dst[offset..offset + 8].copy_from_slice(&self.amount.to_le_bytes());
        offset += 8;
        //===================================================================================DELEGATED
        if let Some(delegate) = self.delegate {
            dst[offset] = 1;
            dst[offset + 1..offset + 33].copy_from_slice(delegate.as_bytes());
            offset += 33;
        } else {
            dst[offset] = 0;
            offset += 33;
        }
        //===================================================================================STATE
        dst[offset] = match self.state {
            AccountState::Uninitialized => 0,
            AccountState::Initialized => 1,
            AccountState::Frozen => 2,
        };
        offset += 1;
        //===================================================================================IS_NATIVE
        if let Some(native) = self.is_native {
            dst[offset] = 1;
            dst[offset + 1..offset + 9].copy_from_slice(&native.to_le_bytes());
        } else {
            dst[offset] = 0;
        }
        offset += 9;
        //===================================================================================DELEGATED_AMOUNT
        dst[offset..offset + 8].copy_from_slice(&self.delegated_amount.to_le_bytes());
        offset += 8;
        //===================================================================================CLOSE_AUTHORITY
        if let Some(authority) = self.close_authority {
            dst[offset] = 1;
            dst[offset + 1..offset + 33].clone_from_slice(authority.as_bytes());
            offset += 33;
        } else {
            dst[offset] = 0;
            offset += 33;
        };
        //===================================================================================END
        Ok(())

        // TODO: Implémentez la sérialisation binaire
        // Format : mint(32) + owner(32) + amount(8) + delegate_flag(1) + delegate(32) +
        //          state(1) + is_native_flag(1) + is_native(8) + delegated_amount(8) +
        //          close_authority_flag(1) + close_authority(32) = 165 bytes
    }

    fn unpack(data: &[u8]) -> Result<Self, PackError>
    where
        Self: Sized,
    {
        //===================================================================================START
        if data.len() < Self::PACKED_SIZE {
            return Err(PackError::BufferTooSmall {
                expected: Self::PACKED_SIZE,
                actual: data.len(),
            });
        }

        let mut offset = 0;
        //===================================================================================MINT +32
        let mint = data[offset..offset + 32]
            .try_into()
            .ok()
            .map(Pubkey::new_from_array)
            .unwrap_or_default();

        offset += 32;
        //===================================================================================OWNER +32
        let owner = data[offset..offset + 32]
            .try_into()
            .ok()
            .map(Pubkey::new_from_array)
            .unwrap_or_default();
        offset += 32;
        //===================================================================================AMOUNT +8

        let amount: u64 = u64::from_le_bytes(
            data[offset..offset + 8]
                .try_into()
                .map_err(|_| PackError::UnvalidLength)?,
        );
        offset += 8;
        //===================================================================================DELEGATE + 1 + 32

        let delegate = {
            if data[offset] == 1 {
                data[(offset + 1)..(offset + 33)]
                    .try_into()
                    .ok()
                    .map(Pubkey::new_from_array)
            } else {
                None
            }
        };
        offset += 33;
        //===================================================================================STATE +1

        let state = match data[offset] {
            0 => Ok(AccountState::Uninitialized),
            1 => Ok(AccountState::Initialized),
            2 => Ok(AccountState::Frozen),
            _ => Err(PackError::InvalidAlignment),
        }
        .map_err(|_| PackError::InvalidAlignment)?;
        offset += 1;
        //===================================================================================IS_NATIVE +1 +8

        let is_native = {
            if data[offset] == 1 {
                Some(u64::from_le_bytes(
                    data[offset + 1..offset + 9]
                        .try_into()
                        .map_err(|_| PackError::UnvalidLength)?,
                ))
            } else {
                None
            }
        };
        offset += 9;
        //===================================================================================DELEGATED_AMOUNT + 8

        let delegated_amount = u64::from_le_bytes(
            data[offset..offset + 8]
                .try_into()
                .map_err(|_| PackError::UnvalidLength)?,
        );
        // data[offset..offset + 8].into() as u64;
        offset += 8;
        //===================================================================================CLOSE_AUTHORITY + 1 + 33

        let close_authority = {
            if data[offset] == 1 {
                data[(offset + 1)..{ offset + 33 }]
                    .try_into()
                    .ok()
                    .map(Pubkey::new_from_array)
            } else {
                None
            }
        };
        //===================================================================================END

        Ok(Self {
            mint,
            owner,
            amount,
            delegate,
            state,
            is_native,
            delegated_amount,
            close_authority,
        })

        // TODO: Implémentez la désérialisation binaire
        // todo!("Implémentez unpack")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_account_size() {
        // Vérifiez que la taille est exactement 165 bytes
        println!("{}", std::mem::size_of::<TokenAccount>());

        assert_eq!(TokenAccount::PACKED_SIZE, 165);
    }

    #[test]
    fn test_pack_unpack_roundtrip() {
        let mut original = TokenAccount::new(
            Pubkey::new_from_array([1; 32]),
            Pubkey::new_from_array([2; 32]),
        );

        original.amount = 10;
        original.state = AccountState::Frozen;
        original.delegate = Some(Pubkey::new_from_array([64u8; 32]));
        original.is_native = Some(20);
        original.delegated_amount = 25548454616;

        let packed = original.pack();
        println!("{:?}", packed);
        let unpacked = TokenAccount::unpack(&packed).unwrap();
        println!("{:?}", unpacked);

        // TODO: Ajoutez les assertions pour vérifier l'égalité
    }

    #[test]
    fn test_delegation_logic() {
        let mut account = TokenAccount::new(
            Pubkey::new_from_array([1; 32]),
            Pubkey::new_from_array([2; 32]),
        );

        // Test sans délégation
        assert!(!account.has_active_delegation());

        // Test avec délégation
        account.delegate = Some(Pubkey::new_from_array([3; 32]));
        account.delegated_amount = 100;
        assert!(account.has_active_delegation());
    }
}

fn main() {
    println!("Simulateur SPL Token - Étape 1 complète");
    println!(
        "Taille du TokenAccount: {} bytes",
        TokenAccount::PACKED_SIZE
    );

    // Test basique
    let account = TokenAccount::new(
        Pubkey::new_from_array([1; 32]),
        Pubkey::new_from_array([2; 32]),
    );

    println!("Compte créé: {:?}", account);
}
