// Copyright 2022 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::collections::HashMap;

use peniko::{Color, ColorStop, ColorStops};

const N_SAMPLES: usize = 512;
const RETAINED_COUNT: usize = 64;

/// Data and dimensions for a set of resolved gradient ramps.
#[derive(Copy, Clone, Debug, Default)]
pub struct Ramps<'a> {
    pub data: &'a [Color],
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub(crate) struct RampCache {
    epoch: u64,
    map: HashMap<ColorStops, (u32, u64)>,
    data: Vec<Color>,
}

impl RampCache {
    pub fn maintain(&mut self) {
        self.epoch += 1;
        if self.map.len() > RETAINED_COUNT {
            self.map
                .retain(|_key, value| value.0 < RETAINED_COUNT as u32);
            self.data.truncate(RETAINED_COUNT * N_SAMPLES);
        }
    }

    pub fn add(&mut self, stops: &[ColorStop]) -> u32 {
        if let Some(entry) = self.map.get_mut(stops) {
            entry.1 = self.epoch;
            entry.0
        } else if self.map.len() < RETAINED_COUNT {
            let id = (self.data.len() / N_SAMPLES) as u32;
            self.data.extend(make_ramp(stops));
            self.map.insert(stops.into(), (id, self.epoch));
            id
        } else {
            let mut reuse = None;
            for (stops, (id, epoch)) in &self.map {
                if *epoch + 2 < self.epoch {
                    reuse = Some((stops.to_owned(), *id));
                    break;
                }
            }
            if let Some((old_stops, id)) = reuse {
                self.map.remove(&old_stops);
                let start = id as usize * N_SAMPLES;
                for (dst, src) in self.data[start..start + N_SAMPLES]
                    .iter_mut()
                    .zip(make_ramp(stops))
                {
                    *dst = src;
                }
                self.map.insert(stops.into(), (id, self.epoch));
                id
            } else {
                let id = (self.data.len() / N_SAMPLES) as u32;
                self.data.extend(make_ramp(stops));
                self.map.insert(stops.into(), (id, self.epoch));
                id
            }
        }
    }

    pub fn ramps(&self) -> Ramps {
        Ramps {
            data: &self.data,
            width: N_SAMPLES as u32,
            height: (self.data.len() / N_SAMPLES) as u32,
        }
    }
}

fn make_ramp(stops: &[ColorStop]) -> impl Iterator<Item = Color> + '_ {
    let mut last_u = 0.0;
    let mut last_c = stops[0].color;
    let mut this_u = last_u;
    let mut this_c = last_c;
    let mut j = 0;
    (0..N_SAMPLES).map(move |i| {
        let u = (i as f64) / (N_SAMPLES - 1) as f64;
        while u > this_u {
            last_u = this_u;
            last_c = this_c;
            if let Some(s) = stops.get(j + 1) {
                this_u = s.offset as f64;
                this_c = s.color;
                j += 1;
            } else {
                break;
            }
        }
        let du = this_u - last_u;
        if du < 1e-9 {
            this_c
        } else {
            let t = (u - last_u) / du;
            last_c.lerp(this_c, t as f32)
        }
        .premultiply()
    })
}
