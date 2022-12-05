// Implement methods to track the max, min, mean, and mode
// https://www.interviewcake.com/question/javascript/temperature-tracker

#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

type OptionUsize = Option<usize>;

struct TempTracker {
    temperatures: Vec<usize>,
    max: OptionUsize,
    min: OptionUsize,
}

impl TempTracker {
    fn new() -> Self {
        TempTracker {
            temperatures: Vec::new(),
            max: None,
            min: None,
        }
    }

    fn insert(&mut self, temperature: usize) {
        self.temperatures.push(temperature);

        if self.max.is_none() || temperature > self.max.unwrap() {
            self.max = Some(temperature);
        }

        if self.min.is_none() || temperature < self.min.unwrap() {
            self.min = Some(temperature);
        }
    }

    fn get_max(&self) -> usize {
        match self.max {
            Some(max) => max,
            None => 0,
        }
    }

    fn get_min(&self) -> usize {
        match self.min {
            Some(min) => min,
            None => 0,
        }
    }

    fn get_mean(&self) -> usize {
        let sum: usize = self.temperatures.iter().sum();
        sum / self.temperatures.len()
    }

    fn get_mode(&self) -> usize {
        let frequencies = self
            .temperatures
            .iter()
            .fold(HashMap::new(), |mut freqs, value| {
                *freqs.entry(value).or_insert(0) += 1;
                freqs
            });

        frequencies
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(value, _)| *value)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_valid() {
        let mut t = TempTracker::new();

        t.insert(50);
        assert_eq!(t.get_max(), 50);
        assert_eq!(t.get_min(), 50);
        assert_eq!(t.get_mean(), 50);
        assert_eq!(t.get_mode(), 50);

        t.insert(80);
        assert_eq!(t.get_max(), 80);
        assert_eq!(t.get_min(), 50);
        assert_eq!(t.get_mean(), 65);
        assert_eq!(t.get_mode() == 50 || t.get_mode() == 80, true);

        t.insert(80);
        assert_eq!(t.get_max(), 80);
        assert_eq!(t.get_min(), 50);
        assert_eq!(t.get_mean(), 70);
        assert_eq!(t.get_mode(), 80);

        t.insert(30);
        assert_eq!(t.get_max(), 80);
        assert_eq!(t.get_min(), 30);
        assert_eq!(t.get_mean(), 60);
        assert_eq!(t.get_mode(), 80);
    }
}
