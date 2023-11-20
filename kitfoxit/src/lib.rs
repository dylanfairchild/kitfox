use kitfoxi::linux::nvme;

pub struct ScanArgs {}
pub struct IdentifyArgs {}

pub enum Operation {
    Scan(ScanArgs),
    Identify(IdentifyArgs),
}

pub struct Config {
    pub operation: Operation,
}

pub fn scan(args: ScanArgs) {
    let recipes = nvme::scan();
    println!("kitfoxi Recipes: {:?}", recipes);

    for recipe in recipes {
        println!("Device Name: {:?}", recipe.path);
        let device = recipe.make_device();
        println!("Device: {:?}", device);
        let buffer = device.identify_controller();
        println!("Serial: {:?}", buffer.serial().unwrap());
        println!("Model: {:?}", buffer.model().unwrap());
    }
}

pub fn identify(args: IdentifyArgs) {}

pub fn run(config: Config) {
    match config.operation {
        Operation::Scan(args) => scan(args),
        Operation::Identify(args) => identify(args),
    }
}
