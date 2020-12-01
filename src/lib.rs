#![no_std]

use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct Pattern {
    steps: SmallVec<[bool; 64]>,
    length: usize,
    cursor: usize,
}

impl Pattern {

    pub fn new(length: usize, pulses: usize, rotation: isize) -> Self {
        let mut pattern = Pattern::with_length(length);
        if pulses > 0 {
            pattern.pulses(pulses);
        }
        pattern.rotate(rotation);
        pattern
    }

    pub fn with_length(length: usize) -> Self {
        let mut steps = SmallVec::new();
        for _ in 0..length {
            steps.push(false);
        }
        Self {
            steps,
            length: length.clone(),
            cursor: 0,
        }
    }

    pub fn from_slice(slice: &[bool]) -> Self {
        Self {
            steps: SmallVec::from_slice(slice),
            length: slice.len(),
            cursor: 0,
        }
    }

    pub fn pulses(&mut self, pulses: usize) -> &mut Self {
        if pulses == 0 {
            return self
        }
        let length = self.length;
        let pulses = if pulses > length {
            length
        } else {
            pulses
        };
        self.steps.clear();
        let mut bucket: usize = 0;
        for _ in 0..length {
            bucket += pulses;
            if bucket >= length {
                bucket -= length;
                self.steps.push(true);
            } else {
                self.steps.push(false);
            }
        }
        if length > 0 {
            let offset = length / pulses - 1;
            self.steps.rotate_right(offset);
        }
        self
    }

    pub fn rotate(&mut self, rotation: isize) -> &mut Self {
        if rotation.is_positive() {
            self.steps.rotate_right(rotation as usize);
        } else if rotation.is_negative() {
            self.steps.rotate_left(rotation.abs() as usize);
        }
        self
    }

    pub fn as_slice(&self) -> &[bool] {
        self.steps.as_slice()
    }

    pub fn step(&self, step: usize) -> Option<bool> {
        if step < self.steps.len() {
            Some(self.steps[step])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }
}

impl Iterator for Pattern {
    type Item = bool;
    fn next(&mut self) -> Option<bool> { 
        if self.cursor < self.steps.len() {
            let current = self.cursor;
            self.cursor += 1;
            Some(self.steps[current])
        } else {
            self.cursor = 0;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let pattern = Pattern::new(4, 0, 0);
        assert_eq!([false, false, false, false], pattern.as_slice());
    }

    #[test]
    fn with_length() {
        let pattern = Pattern::with_length(8);
        assert_eq!(8, pattern.len());
    }

    #[test]
    fn with_zero_length() {
        let pattern = Pattern::with_length(0);
        assert_eq!(0, pattern.len());
    }

    #[test]
    fn from_slice() {
        let pattern = Pattern::from_slice(&[false, false, false, true]);
        assert_eq!([false, false, false, true], pattern.as_slice());
    }

    #[test]
    fn from_slice_zero_len() {
        let pattern = Pattern::from_slice(&[]);
        assert_eq!(0, pattern.len());
    }

    #[test]
    fn pulses() {
        let pattern = Pattern::new(4, 2, 0);
        assert_eq!([true, false, true, false], pattern.as_slice());
    }

    #[test]
    fn rotate() {
        let pattern = Pattern::new(4, 2, -1);
        assert_eq!([false, true, false, true], pattern.as_slice());
    }

    #[test]
    fn step() {
        let pattern = Pattern::new(4, 2, 0);
        assert_eq!(Some(true), pattern.step(0));
        assert_eq!(Some(false), pattern.step(1));
        assert_eq!(None, pattern.step(4));
    }

    #[test]
    fn complex() {
        let mut pattern = Pattern::with_length(3);
        pattern.pulses(1);
        assert_eq!([false, true, false], pattern.as_slice());
    }

    #[test]
    fn new_with_rotate() {
        let mut pattern = Pattern::new(3, 1, 0);
        pattern.rotate(1);
        assert_eq!([false, false, true], pattern.as_slice());
    }

    #[test]
    fn new_with_pulses() {
        let mut pattern = Pattern::new(3, 1, 0);
        pattern.pulses(2);
        assert_eq!([false, true, true], pattern.as_slice());
    }

    #[test]
    fn bjorklund_example() {
        let pattern = Pattern::new(13, 5, -3);
        assert_eq!(
            [true, false, false, true, false, true, false, false, true, false, true, false, false],
            pattern.as_slice()
        );
    }

    #[test]
    fn ruchenitza() {
        let pattern = Pattern::new(7, 3, -3);
        assert_eq!(
            [true, false, true, false, true, false, false],
            pattern.as_slice()
        );
    }

    #[test]
    fn york_samai() {
        let pattern = Pattern::new(6, 5, 1);
        assert_eq!(
            [true, false, true, true, true, true],
            pattern.as_slice()
        );
    }

    #[test]
    fn cumbia() {
        let pattern = Pattern::new(4, 3, 1);
        assert_eq!(
            [true, false, true, true],
            pattern.as_slice()
        );
    }

    #[test]
    fn khafif_e_ramal() {
        let pattern = Pattern::new(5, 2, -3);
        assert_eq!(
            [true, false, true, false, false],
            pattern.as_slice()
        );
    }

    #[test]
    fn agsag_samai() {
        let pattern = Pattern::new(9, 5, 1);
        assert_eq!(
            [true, false, true, false, true, false, true, false, true],
            pattern.as_slice()
        );
    }

    #[test]
    fn venda() {
        let pattern = Pattern::new(12, 5, 0);
        assert_eq!(
            [true, false, false, true, false, true, false, false, true, false, true, false],
            pattern.as_slice()
        );
    }

    #[test]
    fn bendir() {
        let pattern = Pattern::new(8, 7, 1);
        assert_eq!(
            [true, false, true, true, true, true, true, true],
            pattern.as_slice()
        );
    }

    #[test]
    fn overflow() {
        let pattern = Pattern::new(8, 9, 0);
        assert_eq!(
            [true, true, true, true, true, true, true, true],
            pattern.as_slice()
        );
    }
}