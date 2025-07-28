use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

struct Mutex<T> {
    data: UnsafeCell<T>,
    locked: AtomicBool,
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

unsafe impl<T: Send> Send for Mutex<T> {}

unsafe impl<T: Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        loop {
            // On attend que le verrou soit libre (false).
            // Si libre, on le verrouille (true), on definie l'Ordering sur Acquire, afin de
            // guarantir une intergrité des données et se prémunir des dataraces.
            // Sinon, on continue la spin-loop, en evitant de surcharger le CPU.
            match self
                .locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(_) => std::thread::yield_now(),
            }
        }
        MutexGuard { mutex: self }
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // L'obtention du MutexGuard uniquement via le lock de l'AtomicBool nous permet d'etre totalement certain
        // d'etre le seul thread a avoir acces a cette data sur la plage d utilisation du thread.
        // Nous respectons notre contrat avec le borrow_checker par la creation d'une reference depuis la data contenue
        // dans l'UnsafeCell.
        unsafe { &*self.mutex.data.get() }
    } //
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Meme principe que pour deref.
        // Il est de notre devoir de veiller a n'avoir qu'une seule &mut sur le meme scope.

        unsafe { &mut *self.mutex.data.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    // Le MutexGuard est drop -> Le travail a ete effectuee.
    // Nous pouvons ainsi liberer le verrou (false), et indiquer au CPU que nos données sont pretes pour
    // etre partagees avec les autres threads (Ordering::Release)
    fn drop(&mut self) {
        self.mutex
            .locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_mutex_contention() {
        println!("here");
        dbg!("here");
        let mutex = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        dbg!("here0");
        for _ in 0..10 {
            dbg!("here1");
            let mutex_clone = Arc::clone(&mutex);
            handles.push(thread::spawn(move || {
                dbg!("here2");
                for _ in 0..1000 {
                    let mut guard = mutex_clone.lock();
                    *guard += 1;
                }
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Accéder à la donnée finale pour vérification
        let final_guard = mutex.lock();
        assert_eq!(*final_guard, 10 * 1000);
    }
}
