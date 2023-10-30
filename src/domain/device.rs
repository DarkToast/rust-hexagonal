use uuid::Uuid;

use super::{name::Name, description::Description, kind::Kind, failure::Failure};

pub struct Device {
    pub id: Uuid,
    pub name: Name,
    pub description: Description,
    pub kinds: Vec<Kind>,
}

impl Device {
    pub fn new(name: Name, description: Description, kinds: Vec<Kind>) -> Device {
        let uuid: Uuid = Uuid::new_v4();
        Device {
            id: uuid,
            name,
            description,
            kinds,
        }
    }

    pub fn from(
        name: Result<Name, Failure>,
        descripton: Result<Description, Failure>,
        kinds: Vec<Kind>,
    ) -> Result<Device, Vec<Failure>> {
        fn error<T>(result: Result<T, Failure>) -> Failure {
            result.err().unwrap()
        }

        // Match the three input arguments. If all of them are
        // successful, then return a new device. Otherwise, return
        // a list of failures.
        match (name, descripton) {
            (Ok(name), Ok(description)) => Ok(Device::new(name, description, kinds)),
            failure => {
                let mut failures = Vec::new();

                if failure.0.is_err() {
                    failures.push(error(failure.0));
                }

                if failure.1.is_err() {
                    failures.push(error(failure.1));
                }

                Err(failures)
            }
        }
    }
}
