/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start.take();
        while let Some(ptr) = current {
            unsafe {
                let boxed_node = Box::from_raw(ptr.as_ptr());

                current = boxed_node.next;
            }
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(mut list_a:LinkedList<T>,mut list_b:LinkedList<T>) -> Self
    where
        T: PartialOrd
	{
		let mut merged_list = LinkedList::new();

        let mut current_a = list_a.start.take();
        let mut current_b = list_b.start.take();

        while let (Some(mut n_a), Some(mut n_b)) =  (current_a, current_b) {
            unsafe{
                if n_a.as_ref().val <= n_b.as_ref().val {
                    current_a = n_a.as_ref().next;
                    current_b = Some(n_b);

                    merged_list.add_node(n_a);
                } else {
                    current_a = Some(n_a);
                    current_b = n_b.as_ref().next;

                    merged_list.add_node(n_b);
                }
            }
        }

        if let Some(mut remain_node) = current_a {
            if merged_list.end.is_some() {
                unsafe {
                    (*merged_list.end.unwrap().as_ptr()).next = Some(remain_node);
                }
            } else {
                merged_list.start = Some(remain_node);
            }
            merged_list.end = list_a.end; // 前文中let Some(mut remain_node) = current_a这一句不会获取list_a.end导致其所有权转移吗
        } else if let Some(mut remain_node) = current_b {
            if merged_list.end.is_some()  {
                unsafe {
                    (*merged_list.end.unwrap().as_ptr()).next = Some(remain_node);
                }
            } else {
                merged_list.start = Some(remain_node);
            }
            merged_list.end = list_b.end;
        }

        // update length
        merged_list.length  = list_a.length + list_b.length;

        list_a.start = None;
        list_a.end = None;
        list_b.start = None;
        list_b.end = None;

		merged_list
	}

    fn add_node(&mut self, mut node: NonNull<Node<T>>) {
        unsafe{
            node.as_mut().next = None;

        }

        match self.end {
            None => self.start =Some(node),
            Some(mut end_ptr) => unsafe {
                end_ptr.as_mut().next = Some(node);
            },
        }
        self.end = Some(node);
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{ 
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}