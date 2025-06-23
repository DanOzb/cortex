use std::path::{Path};
use std::time::Duration;
use crate::ignore_matcher::IgnoreMatcher; 
use crate::debouncer::Debouncer;
use crate::extension_filter::ExtensionFilter;

pub struct IndexDecider {
    ignore_matcher: IgnoreMatcher,
    extension_filter: ExtensionFilter,
    debouncer: Debouncer,
}

impl IndexDecider {
    pub fn new(ignore_matcher: IgnoreMatcher, extension_filter: ExtensionFilter, debouncer: Debouncer) -> Self {
        Self {
            ignore_matcher,
            extension_filter,
            debouncer,
        }
    }

    pub fn should_index<P: AsRef<Path>>(&mut self, path: P) -> bool {
        !self.ignore_matcher.is_ignored(path.as_ref()) 
        && self.extension_filter.is_supported(path.as_ref()) 
        && self.debouncer.should_index(path.as_ref())
    }

    pub fn debounce_duration_left<P: AsRef<Path>>(&self, path: P) -> Duration{
        self.debouncer.time_left(path)
    }
}
