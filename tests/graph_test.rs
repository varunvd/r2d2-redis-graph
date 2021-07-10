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

