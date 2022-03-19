use std::{cmp::Ordering, collections::BTreeMap};

use crate::{
    args::Command,
    types::{Action, Funscript},
};

pub struct ActionData {
    pub pos: f64,
    pub ty: Option<String>,
}

pub type Actions = BTreeMap<i64, ActionData>;

fn discrete_differential(input: &Funscript) -> Actions {
    let mut result = BTreeMap::new();
    let mut prev = Action {
        at: 0,
        pos: 0.0,
        ty: None,
    };
    let first = input
        .actions
        .get(0)
        .expect("input must have at least one action");

    for it in &input.actions {
        if it.at != first.at {
            let value = f64::abs(it.pos - prev.pos / (it.at as f64 - prev.at as f64));
            result.insert(
                prev.at,
                ActionData {
                    pos: value,
                    ty: it.ty.clone(),
                },
            );
        }

        prev = it.clone();
    }

    result
}

pub struct Transformer {
    data: Actions,
}

impl Transformer {
    pub fn new(input: &Funscript) -> Self {
        let data = discrete_differential(input);

        Self { data }
    }

    pub fn transform(&mut self, command: Command) {
        match command {
            Command::Normalize => self.normalize(0.0, 100.0),
            Command::ScaleLinear { scale } => self.scale_linear(scale),
            Command::ScaleSqrt => self.scale_sqrt(),
            Command::Shorten { diff } => self.shorten(diff),
        }
    }

    fn normalize(&mut self, min_value: f64, max_value: f64) {
        let max = self
            .data
            .values()
            .map(|a| a.pos)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))
            .unwrap_or_default();
        log::info!(
            "normalizing to range {}-{}, max={}",
            min_value,
            max_value,
            max
        );
        if max != 0.0 {
            for value in self.data.values_mut() {
                let new_value = ((max_value * value.pos) / max) + min_value;
                value.pos = new_value;
            }
        }
    }

    fn scale_linear(&mut self, scale: f64) {
        log::info!("scaling linearly by scale {scale}");
        for value in self.data.values_mut() {
            value.pos *= scale;
        }
    }

    fn scale_sqrt(&mut self) {
        log::info!("scaling with square root");
        for value in self.data.values_mut() {
            value.pos = value.pos.sqrt();
        }
    }

    fn shorten(&mut self, diff: f64) {
        log::info!(
            "shortening from size {} by diffing by {}",
            self.data.len(),
            diff
        );
        let mut value = 0.0;
        self.data.retain(|_, ActionData { pos, .. }| {
            let do_retain = (value - *pos).abs() >= diff;
            value = *pos;
            do_retain
        });
        log::info!("new size: {}", self.data.len());
    }

    pub fn into_list(self) -> Vec<Action> {
        self.data
            .into_iter()
            .map(|(at, data)| Action {
                at,
                pos: data.pos.round(),
                ty: data.ty,
            })
            .collect()
    }
}
