// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-nixos-configuration: nixos configuration component model.
//!
//! Real implementation: configurations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Configuration {
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
pub enum NixosConfigurationError {
    EmptyConfigurationName,
    DuplicateConfiguration(String),
    EmptyEventSubject,
}

impl fmt::Display for NixosConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyConfigurationName => write!(f, "configuration name must not be empty"),
            Self::DuplicateConfiguration(id) => write!(f, "duplicate configuration {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for NixosConfigurationError {}

pub struct NixosConfiguration {
    pub name: String,
    configurations: BTreeMap<String, Configuration>,
    events: BTreeMap<String, Event>,
}

impl NixosConfiguration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            configurations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_configuration(&mut self, item: Configuration) -> Result<(), NixosConfigurationError> {
        if item.name.trim().is_empty() {
            return Err(NixosConfigurationError::EmptyConfigurationName);
        }
        let id = item.id.clone();
        if self.configurations.contains_key(&id) {
            return Err(NixosConfigurationError::DuplicateConfiguration(id.clone()));
        }
        self.configurations.insert(id, item);
        Ok(())
    }

    pub fn configuration_count(&self) -> usize {
        self.configurations.len()
    }

    pub fn get_configuration(&self, id: &str) -> Option<&Configuration> {
        self.configurations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), NixosConfigurationError> {
        if event.subject.trim().is_empty() {
            return Err(NixosConfigurationError::EmptyEventSubject);
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

    fn sample_configuration(id: &str, name: &str) -> Configuration {
        Configuration {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = NixosConfiguration::new("NixosConfiguration");
        assert_eq!(m.name, "NixosConfiguration");
        assert_eq!(m.configuration_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_configuration() {
        let mut m = NixosConfiguration::new("NixosConfiguration");
        m.add_configuration(sample_configuration("a1", "Alpha")).unwrap();
        assert_eq!(m.configuration_count(), 1);
        assert!(m.get_configuration("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = NixosConfiguration::new("NixosConfiguration");
        assert!(m.add_configuration(sample_configuration("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = NixosConfiguration::new("NixosConfiguration");
        m.add_configuration(sample_configuration("x", "One")).unwrap();
        assert!(m.add_configuration(sample_configuration("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = NixosConfiguration::new("NixosConfiguration");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
