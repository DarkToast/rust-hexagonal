use domain::description::Description;
use domain::device::Device;
use domain::failure::Failure::{WrongFormat, TooLong};
use domain::kind::Kind;
use domain::name::Name;

#[cfg(test)]
pub mod tests;
pub mod domain;

fn main() {
    let name = Name::new("MyPrettyDevice");
    let description = Description::new("My Description");
    let kind = vec![Kind::Temperature, Kind::Humidity];

    let result = Device::from(name, description, kind);

    match result {
        Ok(device) => {
            println!("Device: ");
            println!("  Name: {}", device.name.raw());
            println!("  Description: {}", device.description.raw());
            println!("  Kinds: {:#?}", device.kinds);
        },
        Err(failures) => for failure in failures {
            match failure {
                WrongFormat { field, message } => println!("Wrong format in field {} with message '{}'", field, message),
                TooLong { field, message } => println!("Too long field {} with message '{}'", field, message),
            }
        }
    }
    //device.id;
}
