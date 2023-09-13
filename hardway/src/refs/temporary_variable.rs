use std::cell::RefCell;
use std::rc::Rc;

// https://users.rust-lang.org/t/how-to-resolve-creates-a-temporary-variable-which-is-freed-while-still-in-use/51244/2

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
struct Node<T> {
    val: T,
    next: Link<T>,
}
struct List<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
}

use std::fmt;
// impl<T: fmt::Debug> fmt::Debug for List<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut temp = &self.head;
//         while let Some(r) = temp {
//             write!(f, "{:?} -> ", r.borrow().val);
//             // temp = &r.borrow().next; // ERROR
//             // temp = unsafe {& (*r.as_ptr()).next };
//             // The reason for this is that borrow() returns a guard with a destructor, 
//             // and you can only access references into the RefCell while that guard is still active. 
//             // So without cloning an Rc, you would need to keep all the guards from each iteration around, 
//             // but the loop destroys the previous guard when advancing to the next iteration.
//             // An Rc clone lets you access the next node without going through the guard, hence sidestepping the issue.
//             //
//             temp = &r.borrow().next.clone(); // OK
//         }
//         write!(f, "[]")
//     }
// }

impl<T: fmt::Debug> fmt::Debug for List<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let mut temp = self.head.clone();
      while let Some(r) = temp {
          write!(f, "{:?} -> ", r.borrow().val);
          temp = r.borrow().next.clone();
      }
      write!(f, "[]")
  }
}
