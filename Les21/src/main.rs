
use std::{thread, time::Duration};
/*
enum Poll<T>{
    Ready(T),
    Pending,
}

trait Future{
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

struct MyFuture{
    done:bool,
}

impl Future for MyFuture{
    type Output = ();
    fn poll(&mut self, _wake: fn()) -> Poll<Self::Output> {
        if self.done {
            Poll::Ready(())
        }else{
            println!("I'm an async function.");
            self.done = true;
            Poll::Pending
        }
    }
}

fn my_function() -> impl Future<Output = ()>{
    MyFuture {done:false}
}
*/

async fn my_function(i:u8){
    println!("[{i}] Start.");
    let s1:String = read_from_database().await;
    println!("First Result: {}", s1);
    println!("[{i}] End.")
}

async fn read_from_database() -> String{
    thread::sleep(Duration::from_secs(2));
    "DB Result".to_string()
}

async fn my_func1(){
    for i in 0..3 {
        println!("Func1 step {}", i);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
async fn my_func2(){
    for i in 0..3{
        println!("Func2 step {}", i);
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() {
    // Async and Await
    /*
    let mut fuc = my_function();
    
    loop {
        match fuc.poll(||{}) {
            Poll::Ready(_) => break,
            Poll::Pending => {}
        }
    }
    */

    // let func = my_function().await;
    // func

    // Tokio Task
    /*
    let mut handles = Vec::new();

    for _i in 0..2{
        let handle = tokio::spawn( async move {
            my_function(_i).await
        });
        handles.push(handle);
    }

    println!("{:?}", handles);
    */

    /*
    let handle1 = tokio::spawn(my_func1());
    let handle2 = tokio::spawn(my_func2());
    handle1.await.unwrap();
    handle2.await.unwrap();
    */

    // tokio::join!(my_func1(), my_func2());
}
