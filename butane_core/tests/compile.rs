//! Compilation tests.
 
use butane_core::db::{BackendConnection, Connection};
use butane_core::migrations::{self, Migration, Migrations};

fn migrate<M: Migration>(connection: &mut Connection, migrations: Box<dyn Migrations<M = M>>) {
    // Migrate forward.
    let to_apply = migrations.unapplied_migrations(connection).unwrap();
    for migration in &to_apply {
        migration
            .apply(connection)
            .unwrap_or_else(|err| panic!("migration {} failed: {err}", migration.name()));
        println!("Applied {}", migration.name());
    }
}
