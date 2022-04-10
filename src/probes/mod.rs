use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

// TODO:
// 1. figure out appropriate structures and libraries for parsing service probe format into rust
// 2. write parsing logic, include PCRE, to get file contents into rust.
// NOTE: pay attention to RE compilation runtime performance.  see if lazy compile is possible.
// 3. wire parsed probes into main scanner logic
// probably need to add results housing structure here, replace current println approach
// 4. Start with hardcoded file to read in, but consider how to automatically find folder of probe files
// either pass in as CLI arg, check environment vars, or some other approach?
// 4. Right now the scanner only does TCP, so only TCP probes make sense.
// add parser support for all, but just short circuit non-tcp probes for now.

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Probe {
    protocol: String,
    totalwaitms: u32,
    tcpwrappedms: u32,
    version: String,
    author: String,
    #[serde(deserialize_with = "deserialize_rematch")]
    services: HashMap<String, ReMatch>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReMatch {
    name: String,
    reg: String, // This should be a regex.  Need to find a crate for PCRE
}

pub fn deserialize_rematch<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let _s = String::deserialize(deserializer)?;
    Ok("lorem ipsum".to_string())
}


#[allow(dead_code)]
mod deserialize_probe {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok("lorem ipsum".to_string())
    }
}