use std::collections::HashSet;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Export {
    buckets: HashSet<String>,
    entries: Vec<ExportEntry>
}

impl Export {
    pub fn new() -> Export {
        Export {
            buckets: HashSet::new(),
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, name: &str, version: Option<&str>, bucket: Option<&str>) {
        if let Some(bucket) = bucket {
            if !self.buckets.contains(bucket) {
                self.buckets.insert(bucket.to_owned());
            }
        }
        self.entries.push(ExportEntry::new(name, version))
    }

    pub fn buckets(&self) -> impl Iterator<Item = &str> {
        self.buckets.iter().map(|s| s.as_str())
    }

    pub fn buckets_vec(&self) -> Vec<String> {
        let mut v = Vec::with_capacity(self.buckets.len());
        for val in self.buckets.iter() {
            v.push(val.clone())
        }
        v
    }

    pub fn entries(&self) -> &[ExportEntry] {
        &self.entries
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportEntry {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    version: Option<String>,
}

impl ExportEntry {
    fn new(name: &str, version: Option<&str>) -> ExportEntry {
        ExportEntry {
            name: name.to_owned(),
            version: version.map(|s| s.to_owned())
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

