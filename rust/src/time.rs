// SPDX-License-Identifier: LGPL-3.0-or-later
#[derive(Debug)]
pub enum TimeScale {
    Regular,
    Fast,
    Superfast,
}

impl TimeScale {
    pub fn to_engine_time(&self) -> f64 {
        match self {
            TimeScale::Regular => 1.0,
            TimeScale::Fast => 5.0,
            TimeScale::Superfast => 10.0,
        }
    }
}
