// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-event-handler: mint event handler component model.
//!
//! Real implementation: handlers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Handler {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintEventHandlerError {
    EmptyHandlerName,
    DuplicateHandler(String),
    EmptyEventSubject,
}

impl fmt::Display for MintEventHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHandlerName => write!(f, "handler name must not be empty"),
            Self::DuplicateHandler(id) => write!(f, "duplicate handler {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintEventHandlerError {}

pub struct MintEventHandler {
    pub name: String,
    handlers: BTreeMap<String, Handler>,
    events: BTreeMap<String, Event>,
}

impl MintEventHandler {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            handlers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_handler(&mut self, item: Handler) -> Result<(), MintEventHandlerError> {
        if item.name.trim().is_empty() {
            return Err(MintEventHandlerError::EmptyHandlerName);
        }
        let id = item.id.clone();
        if self.handlers.contains_key(&id) {
            return Err(MintEventHandlerError::DuplicateHandler(id.clone()));
        }
        self.handlers.insert(id, item);
        Ok(())
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }

    pub fn get_handler(&self, id: &str) -> Option<&Handler> {
        self.handlers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintEventHandlerError> {
        if event.subject.trim().is_empty() {
            return Err(MintEventHandlerError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_handler(id: &str, name: &str) -> Handler {
        Handler {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintEventHandler::new("MintEventHandler");
        assert_eq!(m.name, "MintEventHandler");
        assert_eq!(m.handler_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_handler() {
        let mut m = MintEventHandler::new("MintEventHandler");
        m.add_handler(sample_handler("a1", "Alpha")).unwrap();
        assert_eq!(m.handler_count(), 1);
        assert!(m.get_handler("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintEventHandler::new("MintEventHandler");
        assert!(m.add_handler(sample_handler("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintEventHandler::new("MintEventHandler");
        m.add_handler(sample_handler("x", "One")).unwrap();
        assert!(m.add_handler(sample_handler("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintEventHandler::new("MintEventHandler");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
