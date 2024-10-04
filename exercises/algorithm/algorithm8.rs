/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        self.q1.enqueue(elem);
    }
    
    pub fn pop(&mut self) -> Result<T, &str> {
        // // 如果 q1 不为空，则将 q1 中的所有元素移动到 q2
        // if !self.q1.is_empty() {
        //     while let Ok(v1) = self.q1.dequeue() {
        //         self.q2.enqueue(v1);
        //     }
        //     // 从 q2 中弹出元素
        //     self.q2.dequeue().map_err(|_| "Stack is empty")
        // } else if !self.q2.is_empty() {
        //     // 如果 q1 为空，则将 q2 中的所有元素移动回 q1
        //     while let Ok(v2) = self.q2.dequeue() {
        //         self.q1.enqueue(v2);
        //     }
        //     // 从 q1 中弹出元素
        //     self.q1.dequeue().map_err(|_| "Stack is empty")
        // } else {
        //     // 如果两个队列都为空，则栈为空
        //     Err("Stack is empty")
        // }
        let (from, to) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else if !self.q2.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            return Err("Stack is empty");
        };

        // 将除最后一个元素外的所有元素移动到另一个队列
        while from.size() > 1 {
            if let Ok(item) = from.dequeue() {
                to.enqueue(item);
            }
        }

        // 返回最后一个元素
        from.dequeue().map_err(|_| "Unexpected error: Queue is empty")
    }

    pub fn is_empty(&self) -> bool {
		self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}