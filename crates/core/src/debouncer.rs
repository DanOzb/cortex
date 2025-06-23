use std::collections::{HashMap, VecDeque};
use std::path::{PathBuf, Path};
use std::time::{Duration, Instant};

pub struct Debouncer{
    debounce_duration: Duration,
    index_times: HashMap<PathBuf, Instant>, 
    order: VecDeque<(PathBuf, Instant)>,
}

impl Debouncer {
    pub fn new(duration_secs: u64, duration_nanos: u32) -> Self {

        Self {
            debounce_duration: Duration::new(duration_secs, duration_nanos), 
            index_times: HashMap::new(), 
            order: VecDeque::new(),
        }
    }

    fn cleanup(&mut self) {
        let current_time = Instant::now();

        loop {
            let should_remove = match self.order.front() {
                Some((_, time)) => current_time.duration_since(*time) > self.debounce_duration,
                None => break,
            };
            
            if should_remove {
                if let Some((path, _)) = self.order.pop_front() {
                    self.index_times.remove(&path);
                }
            } else {
                break;
            }
        }
    }

    pub fn should_index<P: AsRef<Path>>(&mut self, path: P) -> bool {
        self.cleanup();

        let current_time = Instant::now();
        match self.index_times.get(path.as_ref()) {
            Some(last_time) if current_time.duration_since(*last_time) < self.debounce_duration => {
                false
            }
            _ => {
                let path_buf = path.as_ref().to_path_buf();
                self.index_times.insert(path_buf.clone(), current_time); 
                self.order.push_back((path_buf, current_time)); 
                true
            }
        }
    }

    pub fn time_left<P: AsRef<Path>>(&self, path: P) -> Duration {
    match self.index_times.get(path.as_ref()) {
        Some(last_time) => {
            let elapsed = last_time.elapsed();
            if elapsed >= self.debounce_duration {
                Duration::ZERO
            } else {
                self.debounce_duration - elapsed
            }
        }
        None => self.debounce_duration,
    }
}

}