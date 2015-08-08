// This file is part of Mooneye GB.
// Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use std::collections::VecDeque;
use time::precise_time_s;

use emulation::MachineCycles;

const HISTORY_SIZE: usize = 64;
const EXPECTED_CYCLES_PER_SECOND: f64 = 4194304.0;

/// A cycles-per-second counter
pub struct PerfCounter {
  history: VecDeque<f64>,
  last_time: f64
}

impl PerfCounter {
  pub fn new() -> PerfCounter {
    PerfCounter {
      history: VecDeque::with_capacity(HISTORY_SIZE),
      last_time: precise_time_s()
    }
  }
  pub fn update(&mut self, cycles: MachineCycles) {
    let time = precise_time_s();
    let cycles_per_s = cycles.as_clock_cycles() as f64 / (time - self.last_time);

    self.make_room_for_new_element();
    self.history.push_front(cycles_per_s);

    self.last_time = time;
  }
  /// Returns the estimated relative speed in percentages compared to
  /// a real device
  pub fn get_relative_speed(&self) -> f64 {
    let mut avg_cps = 0.0;
    for history_cps in self.history.iter() {
      avg_cps += *history_cps;
    }
    avg_cps /= self.history.len() as f64;

    (avg_cps * 100.0) / EXPECTED_CYCLES_PER_SECOND
  }
  fn make_room_for_new_element(&mut self) {
    if self.history.len() >= HISTORY_SIZE {
      let _ = self.history.pop_back();
    }
  }
}
