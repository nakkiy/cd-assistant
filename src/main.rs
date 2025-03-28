mod app;
mod tree;

use std::io;

fn main() -> io::Result<()> {
    let mut app = app::App::new()?;
    app.run()
}
