//! TOML configuration fragments.

use serde::Deserialize;

/// Pinger config.
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ConfigFragment {
    pub(crate) collecting: Option<CollectingFragment>,
    pub(crate) reporting: Option<ReportingFragment>,
}

/// Collecting config group.
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct CollectingFragment {
    /// Collection level, may be `"minimal"` or `"full"` (default: "minimal").
    pub(crate) level: Option<String>,
}

/// Reporting config group.
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct ReportingFragment {
    /// Reporting enablement flag (required).
    pub(crate) enabled: Option<bool>,
}

#[cfg(test)]
pub(crate) fn mock_config() -> ConfigFragment {
    use std::io::Read;

    let fp =
        std::fs::File::open("tests/minimal/fedora-coreos-pinger/config.d/10-default-enable.toml")
            .unwrap();
    let mut bufrd = std::io::BufReader::new(fp);
    let mut content = vec![];
    bufrd.read_to_end(&mut content).unwrap();
    toml::from_slice(&content).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_dist_config_default() {
        let cfg: ConfigFragment = mock_config();

        let expected = ConfigFragment {
            collecting: Some(CollectingFragment {
                level: Some("minimal".to_string()),
            }),
            reporting: Some(ReportingFragment {
                enabled: Some(true),
            }),
        };

        assert_eq!(cfg, expected);
    }
}
