struct Nodo<T: Clone + Ord>{
    children: Vec<Nodo<T>>,
    val: T,
}

#[warn(unused)]
impl<T: Clone + Ord> Nodo<T> {
    pub fn new(val: T) -> Self{
        Self{
            val: val,
            children: Vec::new(),
        }
    }
}


#[warn(unused)]
struct HeapFib<T: Clone + Ord> {
    heap: Vec<Nodo<T>>,
    min_pointer: usize,

}

#[warn(unused)]
impl<T: Clone + Ord> HeapFib<T> {
    pub fn merge() {
        todo!();
    }

    pub fn peek_min(&self) {
        todo!();
    }

    pub fn pop_min(&mut self) {
        todo!();
    }

    pub fn push(&mut self, el: T) {
        self.heap.push(Nodo::new(el));

    }

    pub fn decrease_key(&mut self) {
        todo!();
    }

    pub fn delete(&mut self){
        todo!();
    }




}


fn main() {

}
