mod linked_loop;

use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::linked_loop::LinkedLoop;

fn main() {
    const DECRYPT_KEY: i64 = 811589153;
    let coordinates = [1000, 2000, 3000];

    // let data = vec![1, 2, -3, 3, -2, 0, 4];
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let data = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<i32>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    //part 1
    {
        // println!("input:\t\t\t{:?}", &data);
        let mut linked_loop = LinkedLoop::contugious(data.len());
        for (index, value) in data.iter().enumerate() {
            linked_loop.move_node(index, *value);
        }

        let mixed = linked_loop
            .iter_forwards(0)
            .map(|index| data[index])
            .collect::<Vec<_>>();
        // println!("output:\t\t\t{:?}", mixed);

        let index_of_0 = mixed
            .iter()
            .enumerate()
            .find_map(|(index, value)| (*value == 0).then_some(index))
            .unwrap();

        let target_sum = coordinates
            .iter()
            .map(|target| (index_of_0 + target).rem_euclid(data.len()))
            .map(|index| mixed[index])
            .sum::<i32>();

        println!("part 1: {}", target_sum);
    }

    //part 2
    {
        // println!("input:\t\t\t{:?}", &data);
        let modulus = data.len() as i32 - 1;
        let key = DECRYPT_KEY.rem_euclid(modulus as i64) as i32;

        let mut linked_loop = LinkedLoop::contugious(data.len());
        for _ in 0..10 {
            for (index, value) in data.iter().enumerate() {
                let value = (key * *value).rem_euclid(modulus);
                linked_loop.move_node(index, value);
            }
        }

        let mixed = linked_loop
            .iter_forwards(0)
            .map(|index| data[index])
            .collect::<Vec<_>>();
        // println!("output:\t\t\t{:?}", mixed);

        let index_of_0 = mixed
            .iter()
            .enumerate()
            .find_map(|(index, value)| (*value == 0).then_some(index))
            .unwrap();

        let target_sum = coordinates
            .into_iter()
            .map(|target| (index_of_0 + target).rem_euclid(data.len()))
            .map(|index| mixed[index])
            .sum::<i32>() as i64
            * DECRYPT_KEY;

        println!("part 2: {}", target_sum);
    }
}
