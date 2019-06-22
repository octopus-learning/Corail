use std::fs;

use either::{Left, Right};
use itertools::Itertools;

fn main() {
    let filename = "data/banknote.csv";
    let dataframes = read_dataset(filename);
    let gini_score = gini_index(&dataframes, 2);
    println!("{}", gini_score);

    let q = Question::new(0, 2.0);
    let (left, right) = q.apply(dataframes);
    println!("{} {}", left.len(), right.len());
    let groups = vec![&left, &right];

    let score = gini(&groups, 2);
    println!("{}", score);
    
}

#[derive(Debug)]
struct DataFrame {
    data: [f32; 4],
    class: usize,
}

/*{
    variance: f32,
    skewness: f32,
    kurtosis: f32,
    entropy: f32,
    class: usize,
}*/

fn read_dataset(filename: &str) -> Vec<DataFrame> {
    let content = fs::read_to_string(filename).expect("IO Error");

    let mut ret : Vec<DataFrame> = Vec::new();
    for line in content.lines() {
        let splitted: Vec<&str> = line.split(",").collect();
        let variance: f32 = splitted[0].parse().unwrap();
        let skewness: f32 = splitted[1].parse().unwrap();
        let kurtosis: f32 = splitted[2].parse().unwrap();
        let entropy: f32 = splitted[3].parse().unwrap();
        let class: usize = splitted[4].parse().unwrap();
        let data = [variance, skewness, kurtosis, entropy];
        let dataframe = DataFrame {
            data,
            class,
        };
        ret.push(dataframe);
    }
    ret
}

fn gini(groups: &Vec<&Vec<DataFrame>>, nb_class: usize) -> f32 {
    let mut score = 0.0;
    for group in groups.iter() {
        score += gini_index(group, nb_class);
    }
    score
}

fn gini_index(classes: &Vec<DataFrame>, nb_class: usize) -> f32 {
    let mut hist = vec![0; nb_class];
    for item in classes.iter() {
        hist[item.class] += 1;
    }

    let mut gini: f32 = 1.0;
    let len = classes.len() as f32;
    for class in 0..nb_class {
        let proportion = hist[class] as f32 / len;
        gini -= proportion * proportion;
    }
    gini
}


fn mask<T>(data: Vec<T>, indices: &[usize]) -> (Vec<T>, Vec<T>) {
    data.into_iter().enumerate().partition_map(|(index, elem)| {
        if indices.contains(&index) {
            Left(elem)
        } else {
            Right(elem)
        }
    })
}


struct Question {
    id: usize,
    value: f32
}

impl Question {
    pub fn new(index: usize, value: f32) -> Question {
        Question {
            id: index,
            value
        }
    }

    pub fn apply(&self, data: Vec<DataFrame>) -> (Vec<DataFrame>, Vec<DataFrame>) {
        let mut left: Vec<usize> = Vec::new();
        for (i, item) in data.iter().enumerate() {
            if item.data[self.id] < self.value {
                left.push(i);
            }
        }

        let (left, right) = mask(data, left.as_slice());
        
        (left, right)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gini_0() {
        let example1 = DataFrame(0., 0., 0., 0., 0);
        let example2 = DataFrame(0., 0., 0., 0., 0);
        let data = vec![example1, example2];
        assert_eq!(gini_index(&data, 2), 0.0);
    }

    #[test]
    fn gini_05() {
        let example1 = DataFrame(0., 0., 0., 0., 0);
        let example2 = DataFrame(0., 0., 0., 0., 1);
        let data = vec![example1, example2];
        assert_eq!(gini_index(&data, 2), 0.5);
    }
}
