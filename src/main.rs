#[derive(Clone)]
struct Node<T: Clone + Copy + Ord>{
    parent: Option<Box<Self>>,
    degree: u8,
    children: Vec<Self>,
    val: T,
}


impl<T: Clone + Copy + Ord> Node<T> {
    pub fn new(val: T, parent: Option<Box<Self>>) -> Self{
        Self{
            parent: parent,
            val: val,
            children: Vec::new(),
            degree: 0,
        }
    }
}



struct HeapFib<T: Clone + Copy + Ord> {
    heap: Vec<Node<T>>,
    min_pointer: usize,
}

impl<T: Clone + Copy + Ord> HeapFib<T> {
    pub fn merge(&mut self, mut other: Self) {
        if self.heap.len() <= 0 || other.heap.len() <= 0{
            return;
        }

        let self_min_val = self.peek_min().unwrap();

        let other_min_val = other.peek_min().unwrap();

        let new_min_poiter = if self_min_val < other_min_val { self.min_pointer}
        else {
            self.heap.len() + other.min_pointer
        };
        self.heap.append(&mut other.heap);
        self.min_pointer = new_min_poiter;
    }

    pub fn peek_min(&self) -> Option<T>{
        self.heap.get(self.min_pointer).map(|node| node.val)
    }

    pub fn pop_min(&mut self) -> Option<T>{
        if self.heap.len() <= 0 { return None;}

        let mut node = self.heap[self.min_pointer].clone();

        let node_len = node.children.len()-1;
        self.heap.append(&mut node.children);
        self.heap.swap_remove(self.min_pointer);
        if self.heap.len() <= 0 {return Some(node.val);}

        for i in node_len..self.heap.len() {
            self.heap[i].parent = None;
        }
        todo!();

        Some(node.val);
    }

    pub fn push(&mut self, el: T) {
        self.heap.push(Node::new(el, None));
    }

    pub fn decrease_key(&mut self, el: T, new: T ) {
        todo!();
    }

    pub fn delete(&mut self, el: T){
    }

    fn heapify(&self) -> () {
        for i in 0..u8::MAX {
            let v_size: usize = self.heap.len();
            let count: usize = 0;
            let vec_node: Vec<&Node<T>> = Vec::new();

            for j in 0..self.heap.len(){
                if self.heap[j].degree == i {
                    vec_node.push(&mut self.heap[j]);
                }
            }

            HeapFib::link(vec_node);

            if (v_size == count) {break}
        }

    }

    fn link(vec: Vec<&Node<T>>){
        let i = 0;
        while i+1 <= vec.len() {
            if vec[i].val < vec[i+1].val {
                vec[i].children.push(vec[i+1]);
                vec[i+1].parent = vec[i];
            }
            else {
                vec[i+1].children.push(vec[i]);
                vec[i].parent = vec[i+1];
           }

           i+=2;
        }
    }

}


fn main() {

}
