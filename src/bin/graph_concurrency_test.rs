use std::thread;
use std::sync::{Arc, Mutex};
use redisgraph::Graph;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use std::borrow::{Borrow, BorrowMut};


fn main() {
    let manager  = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = Arc::new(
        Mutex::new(
            r2d2::Pool::builder()
                .max_size(20)
                .build(manager)
                .unwrap()
        )
    );

    //Create pools for two threads
    let pool_update_3 = pool.clone();
    let pool_update_5 = pool.clone();
    let result_pool = pool.clone();

    let update_node_data_to_3 = thread::spawn( move || {
        let mut grab_pool_3 = pool_update_3.lock().unwrap();
        let  mut conn_3 = grab_pool_3.get().unwrap();
        let mut graph_3 = Graph::open(&mut conn_3, "concurrency_update".to_string()).unwrap();
        graph_3.mutate(format!("MATCH (p:Person {{name: 'bla'}}) SET p.age = {}", 3.to_string()).as_str());
    });

    let update_node_data_to_5 = thread::spawn( move || {
        let mut grab_pool_5 = pool_update_5.lock().unwrap();
        let  mut conn_5 = grab_pool_5.get().unwrap();
        let mut graph_5 = Graph::open(&mut conn_5, "concurrency_update".to_string()).unwrap();
        graph_5.mutate(format!("MATCH (p:Person {{name: 'bla'}}) SET p.age = {}", 5.to_string()).as_str());
    });


    println!("Completed main block waiting for other threads now");

    update_node_data_to_3.join().unwrap();
    update_node_data_to_5.join().unwrap();

    let mut grab_pool = result_pool.lock().unwrap();
    let  mut conn = grab_pool.get().unwrap();
    let mut graph = Graph::open(&mut conn, "concurrency_update".to_string()).unwrap();
    println!("Finished waiting for other threads");
    let age: u32 = graph.query("MATCH (p:Person {name: 'bla'}) RETURN p.age").unwrap();
    println!("Result age - {}", age.to_string());
    assert!(age== 5);
}

