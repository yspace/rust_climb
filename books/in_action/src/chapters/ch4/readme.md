## copy

Formally, primitive types are said to possess copy semantics, whereas all other types have move semantics. Unfortunately, for learners of Rust, that special case looks like the default case because beginners typically encounter primitive types first.

### Resolving ownership issues
Four general strategies can help with ownership issues:
 Use references where full ownership is not required.
 Duplicate the value.
 Refactor code to reduce the number of long-lived objects.
 Wrap your data in a type designed to assist with movement issues.

###
Types can opt into two modes of duplication: cloning and copying. Each mode is provided by a trait. Cloning is defined by std::clone::Clone, and the copying mode is defined by std::marker::Copy. Copy acts implicitly. Whenever ownership would otherwise be moved to an inner scope, the value is duplicated instead. (The bits of object a are replicated to create object b.) Clone acts explicitly. Types that implement Clone have a .clone() method that is permitted to do whatever it needs to do to create a new value. 

### 内部可修改
Rc<T> does not allow mutation. To permit that, we need to wrap our wrapper. Rc<RefCell<T>> is a type that can be used to perform interior mutability, first intro- duced at the end of of chapter 3 in section 3.4.1. An object that has interior mutability presents an immutable façade while internal values are being modified.


NOTE Rc<T> is not thread-safe. In multithreaded code, it’s much better to replace Rc<T> with Arc<T> and Rc<RefCell<T>> with Arc<Mutex<T>>. Arc stands for atomic reference counter.