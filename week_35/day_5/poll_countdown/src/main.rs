use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::{mem, pin};

#[pin_project]
struct Countdown {
    remaining: u8,
}

impl Countdown {
    fn new(start: u8) -> Self {
        Self { remaining: start }
    }
}

impl Future for Countdown {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let pin_val = self.project();

        if *pin_val.remaining == 0 {
            println!("Countdown complete!");
            Poll::Ready(())
        } else {
            println!("Countdown: {}", pin_val.remaining);
            *pin_val.remaining -= 1;

            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

// Squelette du mini-exécuteur (fourni)
fn main() {
    // 1. Instanciez votre Future
    let countdown_future = Countdown::new(3);
    // Le `pin_mut!` est nécessaire pour pouvoir appeler `poll`
    futures::pin_mut!(countdown_future);

    // 2. Créez un Waker factice qui ne fait rien
    fn dummy_waker() -> Waker {
        static VTABLE: RawWakerVTable = RawWakerVTable::new(
            |_| unsafe { RawWaker::new(mem::transmute(core::ptr::null::<u64>()), &VTABLE) },
            |_| {},
            |_| {},
            |_| {},
        );
        unsafe {
            Waker::from_raw(RawWaker::new(
                mem::transmute(core::ptr::null::<u64>()),
                &VTABLE,
            ))
        }
    }
    let waker = dummy_waker();

    // 3. Boucle de polling manuelle
    loop {
        println!("Executor: Polling the future...");
        let mut context = Context::from_waker(&waker);
        match countdown_future.as_mut().poll(&mut context) {
            Poll::Ready(_) => {
                println!("Executor: Future is complete.");
                break;
            }
            Poll::Pending => {
                println!("Executor: Future is pending, will poll again.");
                // Dans un vrai exécuteur, on attendrait ici un `wake()`
                // avant de remettre le future dans la queue de polling.
            }
        }
    }
}
