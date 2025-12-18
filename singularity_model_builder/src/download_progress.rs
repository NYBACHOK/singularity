use std::sync::{Arc, atomic::AtomicUsize};

const ITERATION_LOG: usize = 1000;

#[derive(Clone, Debug)]
pub struct DownloadProgressHandler<T, U> {
    current: Arc<AtomicUsize>,
    total: Arc<AtomicUsize>,
    iteration: Arc<AtomicUsize>,
    callback_progress: T,
    callback_finish: Option<U>,
}

#[derive(Clone, Debug)]
pub struct ProgressReport {
    pub downloaded: usize,
    pub total: usize,
}

impl<T, U> DownloadProgressHandler<T, U> {
    pub fn new(callback_progress: T, callback_finish: U) -> Self {
        Self {
            current: Arc::new(AtomicUsize::new(0)),
            total: Arc::new(AtomicUsize::new(0)),
            iteration: Arc::new(AtomicUsize::new(0)),
            callback_progress,
            callback_finish: Some(callback_finish),
        }
    }
}

impl<T: Fn(ProgressReport) + std::marker::Send, U: FnOnce() + std::marker::Send>
    hf_hub::api::tokio::Progress for DownloadProgressHandler<T, U>
{
    async fn init(&mut self, size: usize, _filename: &str) {
        self.total = Arc::new(AtomicUsize::new(size));
        self.current = Arc::new(AtomicUsize::new(0));
        self.iteration = Arc::new(AtomicUsize::new(0));
    }

    async fn update(&mut self, size: usize) {
        let downloaded = self
            .current
            .fetch_add(size, std::sync::atomic::Ordering::SeqCst);

        let total_size = self.total.load(std::sync::atomic::Ordering::SeqCst);

        // Print logs only for each 1000 iteration of download
        if self
            .iteration
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            % ITERATION_LOG
            == 0
        {
            (self.callback_progress)(ProgressReport {
                downloaded,
                total: total_size,
            })
        }
    }

    async fn finish(&mut self) {
        if let Some(callback) = self.callback_finish.take() {
            callback()
        }
    }
}
