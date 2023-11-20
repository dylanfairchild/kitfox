use kitfoxit::{Config, Operation, ScanArgs};

fn main() {
    let config = Config {
        operation: Operation::Scan(ScanArgs {}),
    };
    kitfoxit::run(config);
}
