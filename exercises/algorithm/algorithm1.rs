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
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	where
        T: Ord + Copy
	{
         //1 初始化结果
         let mut result = LinkedList::new();
         let mut a_ptr = list_a.start;
         let mut b_ptr = list_b.start;
         //2 合并链表
         while let(Some(a),Some(b)) = (a_ptr,b_ptr){
         let val_a = unsafe{&(*a.as_ptr()).val};
         let val_b = unsafe{&(*b.as_ptr()).val};
         // 比较当前节点
             if val_a <= val_b{
                 result.add(*val_a);
                 a_ptr= unsafe{(*a.as_ptr()).next};
             }else{
                 result.add(*val_b);
                 b_ptr = unsafe{(*b.as_ptr()).next};
             }
         }
         //3 处理剩余的链表
         while let Some(a) = a_ptr {
             result.add(unsafe{(*a.as_ptr()).val});
             a_ptr = unsafe{(*a.as_ptr()).next};
         }
         while let Some(b) = b_ptr {
             result.add(unsafe{(*b.as_ptr()).val});
             b_ptr = unsafe{(*b.as_ptr()).next};
         }
        
         result
	}

	
	// 使用递归思路实现链表合并
	pub fn merge_recursive(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
	where
        T: Ord + Copy
	{
        let mut result = LinkedList::new();
        
        // 递归合并的辅助函数
        fn merge_nodes<T: Ord + Copy>(
            result: &mut LinkedList<T>, 
            node_a: Option<NonNull<Node<T>>>, 
            node_b: Option<NonNull<Node<T>>>
        ) {
            match (node_a, node_b) {
                // 两个链表都为空，结束递归
                (None, None) => return,
                
                // 链表A为空，添加B中剩余节点
                (None, Some(ptr_b)) => {
                    let val_b = unsafe { &(*ptr_b.as_ptr()).val };
                    result.add(*val_b);
                    merge_nodes(result, None, unsafe { (*ptr_b.as_ptr()).next });
                },
                
                // 链表B为空，添加A中剩余节点
                (Some(ptr_a), None) => {
                    let val_a = unsafe { &(*ptr_a.as_ptr()).val };
                    result.add(*val_a);
                    merge_nodes(result, unsafe { (*ptr_a.as_ptr()).next }, None);
                },
                
                // 两个链表都不为空，比较当前节点并递归处理下一个节点
                (Some(ptr_a), Some(ptr_b)) => {
                    let val_a = unsafe { &(*ptr_a.as_ptr()).val };
                    let val_b = unsafe { &(*ptr_b.as_ptr()).val };
                    
                    if val_a <= val_b {
                        result.add(*val_a);
                        merge_nodes(result, unsafe { (*ptr_a.as_ptr()).next }, Some(ptr_b));
                    } else {
                        result.add(*val_b);
                        merge_nodes(result, Some(ptr_a), unsafe { (*ptr_b.as_ptr()).next });
                    }
                }
            }
        }
        
        // 调用递归辅助函数开始合并
        merge_nodes(&mut result, list_a.start, list_b.start);
        
        result
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

	#[test]
	fn test_merge_recursive() {
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
		let mut list_c = LinkedList::<i32>::merge_recursive(list_a,list_b);
		println!("merged List (recursive) is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}