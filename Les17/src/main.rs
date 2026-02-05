use std::{cell::{Cell, RefCell}, rc::Rc, sync::{Arc, Mutex, RwLock}};


fn main() {
    // Smart Pointers

    // Box<T>   => Heap allocation, Single Ownership, Dereference
    let b:Box<i32> = Box::new(5);
    println!("Box: {}", *b);
    let mut b2:Box<i32> = b;
    *b2 = 55;
    println!("Box2: {}", b2);
    // println!("Box: {}", b);

    // Rc<T>    => Heap allocation, Shared Ownership, Reference Counting, Dereference
    let r:Rc<i32> =  Rc::new(10);
    let r2:Rc<i32> = Rc::clone(&r);
    let r3:Rc<i32> = r;
    // println!("R1: {}", *r);
    println!("Rc2: {}", *r2);
    println!("Rc3: {}", *r3);
    println!("Count Rc2: {}", Rc::strong_count(&r2));
    let r2:Rc<i32> = Rc::new(111);
    println!("Rc2: {}, Count Rc2: {}", *r2, Rc::strong_count(&r2));

    // Arc<T>   =>  Heap allocation, Shared Ownership, Reference Counting, Dereference, Thread-safe
    let a:Arc<i32> = Arc::new(11);
    let a2:Arc<i32> = Arc::clone(&a);
    let a:Arc<i32> = Arc::new(333);
    println!("Arc1: {}, Count: {}", *a, Arc::strong_count(&a));
    println!("Arc2: {}, Count: {}", &a2, Arc::strong_count(&a2));

    // RefCell<T>   =>  Interior mutability, Change when it is Immutable borrow, Runtime borrow check
    // U can use with Rc to use Shared ownership, Reference Counting and Dereference
    let rfc:RefCell<i32> = RefCell::new(32);
    println!("ReffCell1: {}", rfc.borrow());
    *rfc.borrow_mut() = 15;
    println!("ReffCell2: {}", rfc.borrow());
    // let mut_ref2 = data.borrow_mut(); // It panics and RuntimeCheck

    // Cell<T>  =>  Interior mutability, Copy types and fast changes, Works on Stack 
    // U can use with Rc to use Shared ownership, Reference Counting and Dereference
    let cell:Cell<i32> = Cell::new(23);
    cell.set(55);
    println!("Value: {}", cell.get());

    // Mutex<T> =>  Interior mutability, Locked Access, Dereference, Thread Safe
    let mutex:Mutex<i32> = Mutex::new(100);
    {
        let mut val = mutex.lock().unwrap();
        *val = 120;
        println!("Value: {}", val);
    }
    println!("Mutex: {}", mutex.lock().unwrap());

    // RwLock<T>    => Interior mutability, Multiple Reader, Single Writer, Thread-Safe
    // U use it with Arc<T> in practice
    let rw:RwLock<i32> = RwLock::new(999);
    {
        let r1 = rw.read().unwrap();
        let r2 = rw.read().unwrap();
        println!("Read1: {}, Read2: {}", r1, r2);
    }

    {
        let mut w1 = rw.write().unwrap();
        //let mut w2 = rw.write().unwrap();
        *w1 = 1;
        //*w2 = 2;
        println!("Write1: {}", *w1);
    }
}
