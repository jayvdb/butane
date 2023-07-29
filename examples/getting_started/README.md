# butane `getting_started` example

To use this example, install the CLI using `cargo install -p butane_cli` in the project root,
and then run these commands in this directory:

1. Initialise a Sqlite database using

   ```console
   $ butane init sqlite db.sqlite
   something
   ```

2. Migrate the new sqlite database using

   ```console
   $ butane migrate
   something
   ```

3. Run the commands, such as:

   ```console
   $ ../target/debug/show_posts
   something
   ```

See [getting-started.md](https://github.com/Electron100/butane/blob/master/docs/getting-started.md)
for a detailed walkthrough of this example.
