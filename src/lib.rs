pub struct SparkLine {
    data: Vec<f32>,
    width: u32,
}

impl SparkLine {
    pub fn new(data: &[f32]) -> SparkLine {
        SparkLine {
            data: data.to_vec(),
            width: data.len() as u32,
        }
    }

    pub fn render(&self) {
        let min = self.data.iter().reduce(|a, b| if a < b { a } else { b });
        let max = self.data.iter().reduce(|a, b| if a > b { a } else { b });
        if let Some(min) = min {
            if let Some(max) = max {
                let characters = "▁▂▃▄▅▆▇█";
                let num_chars = characters.chars().count();
                for i in 0..self.width {
                    let index = i as f32 / self.width as f32 * self.data.len() as f32;
                    let n = self.data[index as usize];
                    let span = max - min;
                    let rel = (n - min) / span;
                    let c_index = rel * (num_chars - 1) as f32;
                    let c = characters.chars().nth(c_index as usize).unwrap();
                    print!("{}", c);
                }
                println!();
            }
        }
    }
}
