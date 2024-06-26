// Copyright 2023 shadow3aaa@gitbub.com
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fs, path::PathBuf};

use anyhow::Result;

use super::file_handler::FileHandler;

#[derive(Debug)]
pub struct Info {
    pub policy: i32,
    path: PathBuf,
    pub freqs: Vec<isize>,
}

impl Info {
    pub fn new(path: PathBuf) -> Result<Self> {
        let policy = path.file_name().unwrap().to_str().unwrap()[6..].parse()?;

        let mut freqs: Vec<_> = fs::read_to_string(path.join("scaling_available_frequencies"))?
            .split_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();

        if let Ok(boost_freqs) = fs::read_to_string(path.join("scaling_boost_frequencies")) {
            let boost_freqs = boost_freqs
                .split_whitespace()
                .map(|f| f.parse::<isize>().unwrap());
            freqs.extend(boost_freqs);
        }

        freqs.sort_unstable();

        Ok(Self {
            policy,
            path,
            freqs,
        })
    }

    pub fn write_freq(&self, freq: isize, file_handler: &mut FileHandler) -> Result<()> {
        let freq = freq.to_string();
        let max_freq_path = self.max_freq_path();
        file_handler.write(max_freq_path, &freq)?;

        if self.policy != 0 {
            let min_freq_path = self.min_freq_path();
            file_handler.write(min_freq_path, &freq)?;
        }

        Ok(())
    }

    pub fn reset_freq(&self, file_handler: &mut FileHandler) -> Result<()> {
        let max_freq_path = self.max_freq_path();
        let min_freq_path = self.min_freq_path();

        file_handler.write(max_freq_path, self.freqs.last().unwrap().to_string())?;
        file_handler.write(min_freq_path, self.freqs.first().unwrap().to_string())?;

        Ok(())
    }

    fn max_freq_path(&self) -> PathBuf {
        self.path.join("scaling_max_freq")
    }

    fn min_freq_path(&self) -> PathBuf {
        self.path.join("scaling_min_freq")
    }
}
