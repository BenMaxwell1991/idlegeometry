use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::time::{Duration, Instant};

pub fn acquire_lock<'a, T>(lock: &'a RwLock<T>, name: &str) -> RwLockReadGuard<'a, T> {
    let timeout = Duration::from_secs(2);
    let start = Instant::now();

    while start.elapsed() < timeout {
        if let Ok(guard) = lock.try_read() {
            return guard;
        }
        thread::sleep(Duration::from_millis(5));
    }

    eprintln!("Failed to acquire read lock for {} within 2 seconds", name);
    panic!("Lock acquisition timed out");
}

pub fn acquire_lock_mut<'a, T>(lock: &'a RwLock<T>, name: &str) -> RwLockWriteGuard<'a, T> {
    let timeout = Duration::from_secs(2);
    let start = Instant::now();

    while start.elapsed() < timeout {
        if let Ok(guard) = lock.try_write() {
            return guard;
        }
        thread::sleep(Duration::from_millis(5));
    }

    eprintln!("Failed to acquire write lock for {} within 2 seconds", name);
    panic!("Lock acquisition timed out");
}
