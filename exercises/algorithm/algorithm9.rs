/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T:Default + Ord + std::clone::Clone> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        if self.count == 0 {
           // self.items.push(value.clone());
        }

        self.count += 1;
        self.items.push(value);
        
        
        let mut idx: usize  = self.count;
        loop {
            if idx  <= 1   {
                break;
            }
            if  (self.comparator)(&self.items[idx ], &self.items[idx / 2  ]) {
                
                self.items.swap(idx, idx / 2);
                idx = idx / 2;
            }
            else  {
                break;
            }
        }
        
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        let mut x = 0;
		let mut ans =&self.items[1];
        let mut ans_i =  0;
        for i  in &self.items   {
            x += 1;
            if ans > i {
                ans_i = x;
                ans= i;
            }
        }
        ans_i
    }
}

impl<T: Default + Ord +  std::clone::Clone> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T:Default + Ord + std::clone::Clone +std::fmt::Display> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.len() == 0 {
            return None;
        }
        for i in 1..(self.count+1) {
            print!("{}  ", self.items[i]);
        }
        print!("\n");

        let mut x = self.items[1].clone();
        
        self.items.swap(1,self.count);
        

        self.items.pop();
        self.count -= 1;

        for i in 1..(self.count+1) {
            print!("{}  ", self.items[i]);
        }
        print!("---\n");
        
        
       // self.items.swap(1,  self.count);
        let mut idx = 1;

        loop {
        
            let mut i = idx;
            let mut L = idx * 2 ;
            let mut R =  idx * 2  + 1;
            if L <= self.len() &&  (self.comparator)(&self.items[i], &self.items[L])==false {
                i = idx * 2 ;
            }
            if  R  <= self.len() && (self.comparator)(&self.items[i], &self.items[R])==false {
                i = R;
            }
            if i == idx  {
                break;
            }

            self.items.swap(idx, i);
            idx = i;
        }
        return Some(x);
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T:std::clone::Clone>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T:std::clone::Clone>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}