use std::{fs::File, io::Read};

use rand::{thread_rng, prelude::SliceRandom};

use crate::pipeline::Node;

pub struct FileLoader {
    files: Vec<String>,
    load_order: Vec<usize>, // A full vector of indexes for every example, shuffled on reset
    currently_loaded_index: usize // The last example we loaded as an index of the load_order vector (starts at 0)
}

impl FileLoader {
    pub fn new(files: Vec<String>) -> Self {
        FileLoader {
            files,
            load_order: vec![],
            currently_loaded_index: 0
        }
    }
}

impl Node for FileLoader {
    type Input = ();
    type Output = Vec<u8>;

    fn process(&mut self, input: Vec<Self::Input>) -> Vec<Self::Output> {
        let mut read_data = vec![];
        for index in self.load_order[self.currently_loaded_index..input.len()].iter() {
            let mut data = Vec::new();
            let mut f = File::open(&self.files[*index]).expect("FileLoader failed to load file!");
            f.read_to_end(&mut data).expect("Failed to read file!");
            read_data.push(data);
        }
        read_data
    }

    fn reset(&mut self) {
        self.load_order.shuffle(&mut thread_rng());
        self.currently_loaded_index = 0;
    }

    fn data_remaining(&self) -> usize {
        self.load_order.len() - self.currently_loaded_index
    }
}