use rand::Rng;

#[derive(Debug)]
struct BinaryHeap {
    heap: Vec<i32>,
    heap_size: usize,
}

impl BinaryHeap {
    fn new() -> BinaryHeap {
        let heap: Vec<i32> = vec![];
        let heap_size = 0;
        BinaryHeap { heap, heap_size }
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * i + 2
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn insert_val(&mut self, val: i32) {
        self.heap_size += 1;
        self.heap.push(val);
        let mut i = self.heap_size - 1;
        while i != 0 && self.heap[Self::parent(i)] > self.heap[i] {
            let tmp = self.heap[Self::parent(i)];
            self.heap[Self::parent(i)] = self.heap[i];
            self.heap[i] = tmp;
            i = Self::parent(i);
        }
    }

    fn heapify(&mut self, loc: usize) {
        let l = Self::left(loc);
        let r: usize = Self::right(loc);
        let mut smallest = loc;
        if l < self.heap_size && self.heap[l] < self.heap[smallest] {
            smallest = l;
        }
        if r < self.heap_size && self.heap[r] < self.heap[smallest] {
            smallest = r;
        }
        if smallest != loc {
            let tmp = self.heap[loc];
            self.heap[loc] = self.heap[smallest];
            self.heap[smallest] = tmp;
            self.heapify(smallest);
        }
    }

    fn extract_min(&mut self) -> i32 {
        if self.heap_size == 0 {
            return i32::MAX;
        }
        if self.heap_size == 1 {
            self.heap_size -= 1;
            return self.heap[0];
        }
        let root = self.heap[0];
        self.heap[0] = self.heap[self.heap_size - 1];
        self.heap_size -= 1;
        self.heapify(0);
        return root;
    }
}

fn main() {
    let mut h = BinaryHeap::new();
    for _i in 0..10 {
        let x = rand::thread_rng().gen_range(-1000..1000) as i32;
        h.insert_val(x);
    }
    println!("{:?}", h);
    let size = h.heap_size;
    for _j in 0..size {
        let z = h.extract_min();
        print!("{} ", z);
    }
    println!("");
    println!("{:?}", h);
}
