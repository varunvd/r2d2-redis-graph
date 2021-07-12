# r2d2-redisgraph-rs

`r2d2-redisgraph-rs` is an extension of [redisgraph](https://github.com/malte-v/redisgraph-rs.git). The original crate does not support using connections from [r2d2](https://github.com/malte-v/redisgraph-rs.git) connection pool to establish a connection with redis graph database. This problem is solved in `r2d2-redisgraph-rs`. A r2d2 connection pool can be created with mulitple threads, the thread from the connection pool can be used to establish a connection with redis graph database resulting in having mulitple connections to the database in a single application

If you want to use this crate, add this to your Cargo.toml:

```ini
[dependencies]
redis = "0.15.1"
r2d2redisgraph = { path = "/path/to/clone/repo" }
r2d2_redis = "0.14.0"
```

**Warning**: This library has not been thoroughly tested yet and some features are still missing.
Expect bugs and breaking changes.

## Resources

- RedisGraph documentation: [redisgraph.io][]

## Example

First, run RedisGraph on your machine using

```sh
$ docker run --name redisgraph-test -d --rm -p 6379:6379 redislabs/redisgraph
```

Then, try out this code:

```rust
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use r2d2redisgraph::{Graph, RedisGraphResult};

fn main() -> RedisGraphResult<()> {
    //Create a connection manager
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();

    // Create a pool from the connection manager
    let pool = r2d2::Pool::builder()
        .max_size(20)
        .build(manager)
        .unwrap();

    //Create connection from pool
    let  mut connection = pool.get().unwrap();

    //Create a graph struct using the connection
    let mut graph = Graph::open(&mut connection, "MotoGP".to_string())?;

    // Create six nodes (three riders, three teams) and three relationships between them.
    graph.mutate("CREATE (:Rider {name: 'Valentino Rossi', birth_year: 1979})-[:rides]->(:Team {name: 'Yamaha'}), \
        (:Rider {name:'Dani Pedrosa', birth_year: 1985, height: 1.58})-[:rides]->(:Team {name: 'Honda'}), \
        (:Rider {name:'Andrea Dovizioso', birth_year: 1986, height: 1.67})-[:rides]->(:Team {name: 'Ducati'})")?;

    // Get the names and birth years of all riders in team Yamaha.
    let results: Vec<(String, u32)> = graph.query("MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, r.birth_year")?;
    // Since we know just one rider in our graph rides for team Yamaha,
    // we can also write this and only get the first record:
    let (name, birth_year): (String, u32) = graph.query("MATCH (r:Rider)-[:rides]->(t:Team) WHERE t.name = 'Yamaha' RETURN r.name, r.birth_year")?;
    // Let's now get all the data about the riders we have.
    // Be aware of that we only know the height of some riders, and therefore we use an `Option`:
    let results: Vec<(String, u32, Option<f32>)> = graph.query("MATCH (r:Rider) RETURN r.name, r.birth_year, r.height")?;

    // That was just a demo; we don't need this graph anymore. Let's delete it from the database:
    graph.delete()?;

    Ok(())
}
```

[redisgraph.io]:https://redisgraph.io
[docs.rs/redisgraph]:https://docs.rs/redisgraph
