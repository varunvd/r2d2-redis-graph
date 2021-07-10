use redis::{Client, Connection};
use redisgraph::graph::Graph;
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use r2d2_redis::r2d2::PooledConnection;

pub fn get_connection() -> PooledConnection<RedisConnectionManager> {
    let manager  = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder()
        .max_size(20)
        .build(manager)
        .unwrap();
    let  mut conn = pool.get().unwrap();
    return conn;
}

#[allow(dead_code)]
pub fn with_graph<F: FnOnce(&mut Graph)>(action: F) {
    let mut conn = get_connection();
    let mut graph = Graph::open(&mut conn, "test_graph".to_string()).unwrap();

    action(&mut graph);

    graph.delete().unwrap();
}
