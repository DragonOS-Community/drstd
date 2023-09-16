#![allow(deprecated)]

use super::*;
use crate::std::sync::Arc;
use crate::std::thread;
use crate::std::time::Duration;

#[test]
fn sleep() {
    let mutex = Arc::new(SpinMutex::<i32>::default());
    let mutex2 = mutex.clone();
    let guard = mutex.lock();
    let t1 = thread::spawn(move || {
        *mutex2.lock() = 1;
    });

    thread::sleep(Duration::from_millis(50));

    assert_eq!(*guard, 0);
    drop(guard);
    t1.join().unwrap();
    assert_eq!(*mutex.lock(), 1);
}
