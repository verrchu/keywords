mod command;

fn main() {
    let _ = env_logger::init();

    command::run();
}
