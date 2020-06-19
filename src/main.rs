use seahorse::{App, Command, Context};
use std::env;
use std;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .command(node_info_command());

    app.run(args);
}

fn default_action(c: &Context) {
    println!("Hello, {:?}", c.args);
}

fn node_info_action(c: &Context) {
    println!("node_info_action called");
    get_node_info();
}

fn node_info_command() -> Command {
    Command::new("node_info")
        .alias("ni")
        .usage("cli node_info")
        .action(node_info_action)
}

pub async fn get_node_info() -> std::io::Result<()> {
    println!("get_node_info called");
    let node_info = iota::Client::get_node_info().await;
    println!("node_info, {:?}", node_info);
    Ok(())
}
