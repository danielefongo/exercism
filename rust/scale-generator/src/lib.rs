// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.

const SCALE_SHARP: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const SCALE_FLAT: [&str; 12] = [
    "F", "Gb", "G", "Ab", "A", "Bb", "B", "C", "Db", "D", "Eb", "E",
];

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub struct Scale {
    tonic: Tonic,
    intervals: Vec<Interval>,
}

#[derive(Debug)]
pub enum Tonic {
    Sharp(String),
    Flat(String),
}

impl Tonic {
    pub fn new(tonic: &str) -> Self {
        match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#"
            | "d#" => Tonic::Sharp(tonic.to_uppercase()),
            _ => Tonic::Flat(tonic.to_uppercase()),
        }
    }
}

#[derive(Debug)]
pub enum Interval {
    MinorSecond,
    MajorSecond,
    AugmentedSecond,
}

impl Interval {
    pub fn new(interval: char) -> Self {
        match interval {
            'm' => Interval::MinorSecond,
            'M' => Interval::MajorSecond,
            _ => Interval::AugmentedSecond,
        }
    }
    pub fn jumps(&self) -> usize {
        match self {
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::AugmentedSecond => 3,
        }
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let tonic = Tonic::new(tonic);
        let intervals: Vec<Interval> = intervals
            .chars()
            .map(|interval| Interval::new(interval))
            .collect();
        let scale = Scale { tonic, intervals };

        Ok(scale)
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        let (mut scale, note) = match &self.tonic {
            Tonic::Sharp(note) => (SCALE_SHARP.to_vec(), note),
            Tonic::Flat(note) => (SCALE_FLAT.to_vec(), note),
        };

        let scale_idx = scale
            .iter()
            .position(|element| element.to_uppercase() == *note)
            .unwrap();
        let mut result: Vec<String> = Vec::new();

        scale.rotate_left(scale_idx);
        result.push(scale.first().unwrap().to_string());

        self.intervals.iter().for_each(|interval| {
            scale.rotate_left(interval.jumps());
            result.push(scale.first().unwrap().to_string());
        });
        result
    }
}
