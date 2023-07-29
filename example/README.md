# butane example

To use this example, install the CLI using `cargo install -p butane_cli` in the project root,
and then run these commands in this directory:

1. Initialise a Sqlite database using `$ butane init sqlite db.sqlite`
2. Initialise the migrations using `$ butane makemigration initial`
3. Migrate the new sqlite database using `$ butane migrate`
4. Run the example `$ ../target/debug/example`

Any use of `cargo` to build/run this project will likely delete &
recreate the `example/.butane` directory, and the above steps will
need to be repeated.
