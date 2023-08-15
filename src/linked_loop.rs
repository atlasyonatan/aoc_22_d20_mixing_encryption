#[derive(Debug)]
pub struct Node<I> {
    previous: I,
    next: I,
}

pub struct LinkedLoop(Vec<Node<usize>>);

impl LinkedLoop {
    pub fn contugious(length: usize) -> LinkedLoop {
        let mut nodes = Vec::with_capacity(length);
        nodes.push(Node {
            previous: length - 1,
            next: 1,
        });
        for i in 1..=length - 2 {
            nodes.push(Node {
                previous: i - 1,
                next: i + 1,
            });
        }
        nodes.push(Node {
            previous: length - 2,
            next: 0,
        });
        Self(nodes)
    }

    pub fn step_farwards(&self, mut index: usize, amount: usize) -> Option<usize> {
        self.0.get(index)?;
        for _ in 0..amount {
            index = self.0.get(index).unwrap().next;
        }
        Some(index)
    }

    pub fn step_backwards(&self, mut index: usize, amount: usize) -> Option<usize> {
        self.0.get(index)?;
        for _ in 0..amount {
            index = self.0.get(index).unwrap().previous;
        }
        Some(index)
    }

    pub fn move_node(&mut self, index: usize, amount: i32) -> Option<()> {
        self.0.get(index)?;

        let amount = amount.rem_euclid(self.0.len() as i32 - 1);

        if amount == 0 {
            return Some(());
        }

        let current_node = self.0.get(index).unwrap();
        let current_previous = current_node.previous;
        let current_next = current_node.next;

        let previous_node = self.0.get_mut(current_previous).unwrap();
        previous_node.next = current_next;
        let next_node = self.0.get_mut(current_next).unwrap();
        next_node.previous = current_previous;

        let (new_previous, new_next) = match amount.cmp(&0) {
            std::cmp::Ordering::Equal => panic!(),
            std::cmp::Ordering::Less => {
                let new_next = self.step_backwards(index, amount.abs() as usize).unwrap();
                let new_previous = self.step_backwards(new_next, 1).unwrap();
                (new_previous, new_next)
            }
            std::cmp::Ordering::Greater => {
                let new_previous = self.step_farwards(index, amount.abs() as usize).unwrap();
                let new_next = self.step_farwards(new_previous, 1).unwrap();
                (new_previous, new_next)
            }
        };

        let current_node = self.0.get_mut(index).unwrap();
        current_node.previous = new_previous;
        current_node.next = new_next;

        let new_previous_node = self.0.get_mut(new_previous).unwrap();
        new_previous_node.next = index;

        let new_next_node = self.0.get_mut(new_next).unwrap();
        new_next_node.previous = index;
        Some(())
    }

    pub fn iter_forwards<'a>(&'a self, start: usize) -> impl Iterator<Item = usize> + 'a {
        (0..self.0.len())
            .into_iter()
            .scan(start, |current_index, _| {
                let index = *current_index;
                *current_index = self.0.get(index).unwrap().next;
                Some(index)
            })
    }

    pub fn iter_backwards<'a>(&'a self, start: usize) -> impl Iterator<Item = usize> + 'a {
        (0..self.0.len())
            .into_iter()
            .scan(start, |current_index, _| {
                let index = *current_index;
                *current_index = self.0.get(index).unwrap().previous;
                Some(index)
            })
    }
}
