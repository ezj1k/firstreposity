#[derive(Debug)]
struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    fn new() -> Self {
        MaxHeap { data: Vec::new() }
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * i + 2
    }

    fn insert(&mut self, value: i32) {
        if self.data.contains(&value) {
            println!("Valoarea deja exista");
            return;
        }

        self.data.push(value);
        self.heapify_up(self.data.len() - 1);
    }

    fn heapify_up(&mut self, mut i: usize) {
        while i > 0 && self.data[i] > self.data[Self::parent(i)] {
            self.data.swap(i, Self::parent(i));
            i = Self::parent(i);
        }
    }

    fn top(&self) -> Option<i32> {
        self.data.first().copied()
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let max = self.data[0];

        let last = self.data.pop().unwrap();
        if !self.data.is_empty() {
            self.data[0] = last;
            self.heapify_down(0);
        }
        Some(max)
    }

    fn heapify_down(&mut self, mut i: usize) {
        loop {
            let mut largest = i;
            let left = Self::left(i);
            let right = Self::right(i);

            if left < self.data.len() && self.data[left] > self.data[largest] {
                largest = left;
            }
            if right < self.data.len() && self.data[right] > self.data[largest] {
                largest = right;
            }

            if largest != i {
                self.data.swap(i, largest);
                i = largest;
            } else {
                break;
            }
        }
    }
}

fn main() {
    let mut h = MaxHeap::new();

    h.insert(10);
    h.insert(20);
    h.insert(5);
    h.insert(40);

    println!("Top: {:?}", h.top()); // 40

    println!("Pop: {:?}", h.pop()); // 40
    println!("Pop: {:?}", h.pop()); // 20

    println!("Top: {:?}", h.top()); // 10

    println!("{:?}", h); // vezi continutul
}
