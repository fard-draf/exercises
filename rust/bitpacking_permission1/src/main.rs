pub mod error;
use std::fmt;

use crate::error::PermError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Permissions {
    bits: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Permission {
    Read = 1 << 0,     // bit 0
    Write = 1 << 1,    // bit 1
    Execute = 1 << 2,  // bit 2
    Moderate = 1 << 3, // bit 3
    Admin = 1 << 4,    // bit 4
    Ban = 1 << 5,      // bit 5
}

impl fmt::Display for Permissions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(
            read -> {},
            write -> {},
            execute -> {},
            moderate -> {},
            admin -> {},
            ban -> {} 
            )",
            (self.bits & Self::MASK),
            ((self.bits >> 1) & Self::MASK),
            ((self.bits >> 2) & Self::MASK),
            ((self.bits >> 3) & Self::MASK),
            ((self.bits >> 4) & Self::MASK),
            ((self.bits >> 5) & Self::MASK),
        )
    }
}

impl Permissions {
    //MASK
    const MASK: u64 = 0x1; // 1 bit a 1;

    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn default() -> Self {
        Self { bits: 3 }
    }

    pub fn from_bits(bits: u64) -> Self {
        Self { bits }
    }

    pub fn add_permission(&mut self, perm: Permission) -> Result<(), PermError> {
        self.bits |= perm as u64;
        self.validate()?;
        Ok(())
    }

    pub fn remove_permission(&mut self, perm: Permission) -> Result<(), PermError> {
        self.bits &= !(perm as u64);
        self.validate()?;
        Ok(())
    }

    pub fn have_permission(&self, perm: Permission) -> Result<bool, PermError> {
        Ok((((self.bits >> (perm as u64)) & Self::MASK) as u8) == 1)
    }

    pub fn clear_all(&mut self) -> Result<(), PermError> {
        self.bits &= !(u64::MAX);
        Ok(())
    }

    //============================================================ADVANCED OPERATOION

    pub fn add_multiples(&mut self, perm: &[Permission]) -> Result<(), PermError> {
        perm.iter().for_each(|perm| {
            self.bits |= *perm as u64;
        });

        Ok(())
    }

    pub fn active_permissions(&self) -> Vec<Permission> {
        let mut vec_perm = Vec::with_capacity(6);

        let permissions = [
            (1, Permission::Read),
            (1 << 1, Permission::Write),
            (1 << 2, Permission::Execute),
            (1 << 3, Permission::Moderate),
            (1 << 4, Permission::Admin),
            (1 << 5, Permission::Ban),
        ];

        for (bit, perm) in permissions.iter() {
            if (self.bits & bit) != 0 {
                vec_perm.push(*perm);
            }
        }

        vec_perm
    }

    pub fn has_all(&self, required: &[Permission]) -> bool {
        required.iter().all(|perm| (self.bits & *perm as u64) != 0)
    }

    pub fn has_any(&self, options: &[Permission]) -> bool {
        options.iter().any(|perm| (self.bits & *perm as u64) != 0)
    }

    //================================================================VALIDATION

    pub fn validate(&mut self) -> Result<(), PermError> {
        if ((self.bits >> 4) & Self::MASK) == 1 {
            self.bits |= (Permission::Read as u64) | (Permission::Write as u64);
        };
        Ok(())
    }

    pub fn from_string(&mut self, s: &str) -> Result<Self, PermError> {
        let chunck = s.split(",").take(6).collect::<Vec<_>>();

        for element in chunck {
            match element.trim().to_lowercase().as_str() {
                "read" => Self::add_permission(self, Permission::Read)?,
                "write" => Self::add_permission(self, Permission::Write)?,
                "execute" => Self::add_permission(self, Permission::Execute)?,
                "moderate" => Self::add_permission(self, Permission::Moderate)?,
                "admin" => Self::add_permission(self, Permission::Admin)?,
                "ban" => Self::add_permission(self, Permission::Ban)?,
                _ => Err(PermError::ConflictingPermissions)?,
            };
        }

        Ok(Self { bits: self.bits })
    }

    //==================================================================AFFICHAGE
}

fn main() -> Result<(), PermError> {
    let mut perm = Permissions::new();

    println!("{:06b}", perm.bits);
    perm.add_multiples(&[Permission::Execute])?;

    println!("{:06b}", perm.bits);

    perm.add_permission(Permission::Admin)?;
    println!("{:06b}", perm.bits);

    // perm.remove_permission(Permission::Execute)?;
    println!("{:06b}", perm.bits);

    let actives = perm.active_permissions();
    println!("{:?}", actives);

    let has_all = perm.has_all(&[Permission::Admin, Permission::Ban]);
    println!("{}", has_all);

    perm.remove_permission(Permission::Admin)?;
    println!("after unadmin {:06b}", perm.bits);

    let has_any = perm.has_any(&[Permission::Admin]);
    println!("{}", has_any);

    let bits = perm.from_string("ban, admin, execute")?;
    println!("{:06b}", bits.bits);

    println!("{}", perm);

    let default_user_perm = Permissions::default();
    println!("{}", default_user_perm);
    Ok(())
}
