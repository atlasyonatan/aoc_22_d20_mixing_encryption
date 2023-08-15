use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let file_path = "../input.txt";
    let path = Path::new(file_path);
    let file = File::open(path).unwrap();
    let mut data = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse::<i32>())
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    // let mut data = vec![1, 2, -3, 3, -2, 0, 4];
    // println!("input:\t\t\t{:?}", &data);
    mix(&mut data);
    // println!("mixed:\t\t\t{:?}", &data);

    let index0 = data
        .iter()
        .enumerate()
        .find_map(|(index, value)| (*value == 0).then_some(index))
        .unwrap();

    let target_sum = [1000usize, 2000, 3000]
        .into_iter()
        .map(|target| (index0 + target).rem_euclid(data.len()))
        .map(|index| data[index])
        .sum::<i32>();

    println!("part 1: {}", target_sum);
}

fn mix(data: &mut [i32]) -> () {
    let mut linked_loop = contiguous_linked_loop(data.len());
    for (index, value) in data.iter().enumerate() {
        move_node(&mut linked_loop, index, *value);
    }

    let mut mixed = Vec::with_capacity(data.len());
    let mut next = 0;
    for _ in 0..data.len() {
        mixed.push(data[next]);
        next = step_farwards(&linked_loop, next, 1);
    }
    data.clone_from_slice(&mixed)
}

#[derive(Debug)]
struct Node<I> {
    previous: I,
    next: I,
}

type LinkedLoop = Vec<Node<usize>>;

fn contiguous_linked_loop(length: usize) -> LinkedLoop {
    let mut linked_loop = Vec::new();
    linked_loop.push(Node {
        previous: length - 1,
        next: 1,
    });
    for i in 1..=length - 2 {
        linked_loop.push(Node {
            previous: i - 1,
            next: i + 1,
        })
    }
    linked_loop.push(Node {
        previous: length - 2,
        next: 0,
    });
    linked_loop
}

fn step_farwards(linked_loop: &LinkedLoop, mut index: usize, amount: usize) -> usize {
    assert!((0..linked_loop.len()).contains(&index));
    for _ in 0..amount {
        index = linked_loop.get(index).unwrap().next;
    }
    index
}

fn step_backwards(linked_loop: &LinkedLoop, mut index: usize, amount: usize) -> usize {
    assert!((0..linked_loop.len()).contains(&index));
    for _ in 0..amount {
        index = linked_loop.get(index).unwrap().previous;
    }
    index
}

fn move_node(linked_loop: &mut LinkedLoop, index: usize, amount: i32) {
    assert!((0..linked_loop.len()).contains(&index));

    let amount = amount.rem_euclid(linked_loop.len() as i32 - 1);

    if amount == 0 {
        return;
    }

    let current_node = linked_loop.get(index).unwrap();
    let current_previous = current_node.previous;
    let current_next = current_node.next;

    let previous_node = linked_loop.get_mut(current_previous).unwrap();
    previous_node.next = current_next;
    let next_node = linked_loop.get_mut(current_next).unwrap();
    next_node.previous = current_previous;

    let (new_previous, new_next) = match amount.cmp(&0) {
        std::cmp::Ordering::Equal => panic!(),
        std::cmp::Ordering::Less => {
            let new_next = step_backwards(&linked_loop, index, amount.abs() as usize);
            let new_previous = step_backwards(&linked_loop, new_next, 1);
            (new_previous, new_next)
        }
        std::cmp::Ordering::Greater => {
            let new_previous = step_farwards(&linked_loop, index, amount.abs() as usize);
            let new_next = step_farwards(&linked_loop, new_previous, 1);
            (new_previous, new_next)
        }
    };

    let current_node = linked_loop.get_mut(index).unwrap();
    current_node.previous = new_previous;
    current_node.next = new_next;

    let new_previous_node = linked_loop.get_mut(new_previous).unwrap();
    new_previous_node.next = index;

    let new_next_node = linked_loop.get_mut(new_next).unwrap();
    new_next_node.previous = index;
}
