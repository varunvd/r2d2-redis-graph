mod common;

use redisgraph::Graph;
use serial_test::serial;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};

use common::*;

#[test]
#[serial]
fn test_open_delete() {
    let mut conn = get_connection();

    let graph = Graph::open(&mut conn, "test_open_delete_graph".to_string()).unwrap();
    graph.delete().unwrap();
}

#[test]
#[serial]
fn test_open_delete_r2d2_connection_pool() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder()
        .max_size(20)
        .build(manager)
        .unwrap();
    let  mut conn = pool.get().unwrap();
    let graph = Graph::open(&mut conn, "test_open_delete_graph_with_connection_pool".to_string()).unwrap();
    graph.delete().unwrap();
}
