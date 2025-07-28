use std::{
    cell::{Cell, UnsafeCell},
    ops::{Deref, DerefMut},
};

struct LogicCell<T> {
    data: UnsafeCell<T>,
    counter: Cell<isize>,
}

//==================================================================================
pub struct Ref<'a, T> {
    cell: &'a LogicCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.cell.data.get() }
    }
    // L'acces a Deref se fait via la methode borrow, qui retourne une Ref<T>. On passe donc d'abord par une condition qui nous garanti
    // que self.counter != -1, donc qu'il n'y a pas de reference mutable en cours; il y a deux possibilités : soit aucune reference, self.counter == 0,
    // soit des references immuables, self.counter >= 1. Dans ces deux cas, il est totalement safe de créer une réference immuable de self.data.
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        self.cell.counter.update(|x| x - 1isize);
    }
}
//==================================================================================
pub struct RefMut<'a, T> {
    cell: &'a LogicCell<T>,
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.cell.data.get() }
    }
    // De meme que pour l'implementation Deref pour Ref<T>. Cette implementation permet d'utiliser le trait enfant
    // de Deref qui est DerefMut.
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.cell.data.get() } // idem pour borrow_mut et &
    }
    // L'acces a DerefMut se fait via borrow_mut. Borrow_mut possede une condition tres stricte: elle ne peut fonctionner
    // uniquement si self.counter == 0. Cela nous garanti qu il n y a ni reference mutable, ni reference immuable. La
    // creation d une ref mut est donc totalement safe et controlee via cette methode.
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        self.cell.counter.set(0);
    }
}

//==================================================================================
impl<T> LogicCell<T> {
    fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
            counter: Cell::new(0),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        if self.counter.get() == -1isize {
            core::panic!("Mutable Borrowing is pending");
        } else {
            self.counter.update(|x| x + 1isize);
            Ref { cell: self }
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        if self.counter.get() != 0isize {
            core::panic!("References are pending");
        } else {
            self.counter.set(-1isize);
            RefMut { cell: self }
        }
    }
}

//==================================================================================
fn main() {}
//==================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_borrow() {
        let cell = LogicCell::new(5);
        let borrowed = cell.borrow();
        assert_eq!(*borrowed, 5);
    }

    #[test]
    fn test_simple_borrow_mut() {
        let cell = LogicCell::new(String::from("hello"));
        let mut borrowed = cell.borrow_mut();
        *borrowed += " world";
        // Le drop de `borrowed` se fait ici.
    }

    // TODO: Ajouter les tests qui vérifient les panics.
    // Utilisez #[should_panic]
    #[test]
    #[should_panic]
    fn test_bad_counter_borrow() {
        let cell = LogicCell {
            data: UnsafeCell::new(5),
            counter: Cell::new(0),
        };
        let borrow_mut = cell.borrow_mut();
        let borrow = cell.borrow(); //should panic
    }

    #[test]
    #[should_panic]
    fn test_bad_counter_borrow_mut_1() {
        let cell = LogicCell {
            data: UnsafeCell::new(5),
            counter: Cell::new(0),
        };
        let borrow = cell.borrow();
        let borrow_mut = cell.borrow_mut(); //should panic
    }

    #[test]
    fn test_drop_counter() {
        let cell = LogicCell {
            data: UnsafeCell::new(5),
            counter: Cell::new(0),
        };

        let cell_assert = Cell::new(0);
        {
            {
                let borrow = cell.borrow();
                assert_eq!(cell.counter.get(), Cell::new(1).get());
            }
            let borrow_mut = cell.borrow_mut();
            assert_eq!(cell.counter.get(), Cell::new(-1).get());
        }
        assert_eq!(cell.counter.into_inner(), cell_assert.into_inner());
    }
}
