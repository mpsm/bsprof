use super::profile;
use serde::Serialize;
use std::fs::File;

#[derive(Serialize)]
pub struct Report {
    pub system_info: profile::info::SystemInfo,
    pub profile_settings: profile::ProfileSettings,
    pub profile_results: Vec<profile::ProfileResult>,
}

impl Report {
    pub fn new(
        system_info: &profile::info::SystemInfo,
        settings: &profile::ProfileSettings,
    ) -> Report {
        Report {
            system_info: (*system_info).clone(),
            profile_settings: *settings,
            profile_results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: profile::ProfileResult) {
        self.profile_results.push(result);
    }

    pub fn save(&self) {
        let file = File::create("report.json").unwrap();
        serde_json::to_writer_pretty(file, self).unwrap();
    }
}
