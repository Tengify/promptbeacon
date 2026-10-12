use std::{
    sync::{Arc, Condvar, Mutex},
    thread::JoinHandle,
    time::Duration,
};
#[derive(Default)]
pub struct Stop {
    flag: Mutex<bool>,
    wake: Condvar,
}
impl Stop {
    pub fn requested(&self) -> bool {
        *self.flag.lock().unwrap_or_else(|e| e.into_inner())
    }
    pub fn wait(&self, duration: Duration) -> bool {
        let flag = self.flag.lock().unwrap_or_else(|e| e.into_inner());
        let (flag, _) = self
            .wake
            .wait_timeout_while(flag, duration, |stop| !*stop)
            .unwrap_or_else(|e| e.into_inner());
        *flag
    }
}
pub struct Worker {
    stop: Arc<Stop>,
    handle: Option<JoinHandle<()>>,
}
impl Worker {
    pub fn spawn(run: impl FnOnce(Arc<Stop>) + Send + 'static) -> std::io::Result<Self> {
        let stop = Arc::new(Stop::default());
        let child_stop = stop.clone();
        let handle = std::thread::Builder::new()
            .name("promptbeacon-monitor".into())
            .spawn(move || run(child_stop))?;
        Ok(Self {
            stop,
            handle: Some(handle),
        })
    }
    pub fn stop(&mut self) {
        *self.stop.flag.lock().unwrap_or_else(|e| e.into_inner()) = true;
        self.stop.wake.notify_all();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.stop();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    #[test]
    fn shutdown_interrupts_a_long_wait_and_drops_thread_owned_resources() {
        struct Resource(Arc<AtomicBool>);
        impl Drop for Resource {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }
        let dropped = Arc::new(AtomicBool::new(false));
        let resource = Resource(dropped.clone());
        let mut worker = Worker::spawn(move |stop| {
            let _resource = resource;
            while !stop.wait(Duration::from_secs(60)) {}
        })
        .unwrap();
        worker.stop();
        assert!(dropped.load(Ordering::SeqCst));
        worker.stop(); // Idempotent.
    }
}
