use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use butane_core::db::{connect, ConnectionSpec};
use butane_test_helper::{pg_tmp_server_create, PgServerOptions};

fn main() {
    // Start the PostgreSQL server
    let server = pg_tmp_server_create(PgServerOptions {
        port: Some(5432),
        ..Default::default()
    })
    .unwrap();
    // Print the connection string
    println!(
        "Running temporary PostgreSQL server\nUnix socket dir: {}\nUser: postgres\nPort: 5432",
        server.sockdir.path().display()
    );

    let connection_spec = ConnectionSpec::new("pg", format!(
        "host={} user=postgres port=5432",
        server.sockdir.path().display()
    ));

    let new_dbname = format!("butane_tmp_{}", uuid::Uuid::new_v4().simple());

    let conn = connect(&connection_spec).unwrap();
    conn.execute(format!("CREATE DATABASE {new_dbname};"))
        .unwrap();

    println!("Temporary database created: {new_dbname}");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Exiting...");
}
