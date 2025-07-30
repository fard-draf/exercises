use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug)]
struct TheFuture {
    val: u8,
    ptr_onme: *const u8,
    counter_poll: u8,
    _market: PhantomData<*const ()>,
}

impl TheFuture {
    fn new() -> Self {
        Self {
            val: 43,
            ptr_onme: std::ptr::null(),
            counter_poll: 0,
            _market: PhantomData,
        }
    }
}

impl Future for TheFuture {
    type Output = u8;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        
        if this.counter_poll == 0 {
            this.ptr_onme = &this.val as *const u8;
            println!("[PTR_ONME]: {:p}", this.ptr_onme);
            this.counter_poll += 1;
            cx.waker().wake_by_ref(); 
            return Poll::Pending;
        }

        if this.counter_poll < 5 {
            println!("[PTR_ONME {}]: {:p}", this.counter_poll, this.ptr_onme);
            this.counter_poll += 1;
            this.val += 10;
            cx.waker().wake_by_ref(); 
            return Poll::Pending;
        } else {
            println!("Value reached!");
           return  Poll::Ready(this.val);
        }
    }
}

#[tokio::main]
async fn main() {
    let my_pinned_future = TheFuture::new();
    let res = my_pinned_future.await;
    println!("[RES]: {:?}", res);
}
