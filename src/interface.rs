    use std::fs::File;
    use std::io::prelude::*;
    use std::io;
    use crate::errors;

    pub struct PapirusDisplay {
        interface: String,
    }

    impl PapirusDisplay {
        pub fn new(display_path: &str) -> PapirusDisplay {

            PapirusDisplay {
                interface: String::from(display_path),
            }
        }

        pub fn full_update(&self) {
            const UPDATE_COMMAND: &str = "U";
            self.execute_command(UPDATE_COMMAND);
        }

        pub fn partial_update(&self) {
            const PARTIAL_UPDATE_COMMAND: &str = "P";
            self.execute_command(PARTIAL_UPDATE_COMMAND);
        }

        pub fn fast_update(&self) {
            const FAST_UPDATE_COMMAND: &str = "F";
            self.execute_command(FAST_UPDATE_COMMAND);

        }

        pub fn clear(&self) {
            const CLEAR_COMMAND: &str = "C";
            self.execute_command(CLEAR_COMMAND);
        }

        fn execute_command(&self, command: &str) {
            const COMMAND_FILE: &str = "command";
            let command_path = format!("{}/{}", self.interface, COMMAND_FILE);
            let mut papirus_interface = open_write_interface(&command_path).unwrap();
            papirus_interface.write(command.as_bytes()).unwrap();
            papirus_interface.flush().unwrap();
        }

        pub fn write_data(&self, data: Vec<u8>) {
            const DISPLAY_FILE: &str = "BE/display";
            let display_path = format!("{}/{}", self.interface, DISPLAY_FILE);
            let mut papirus_interface = open_write_interface(&display_path).unwrap();
            papirus_interface.write(&data).unwrap();
            papirus_interface.flush().unwrap();
        }

        pub fn get_version(&self) -> String {
            const VERSION_FILE: &str = "version";
            let version = self.read_from_interface(VERSION_FILE);
            String::from_utf8(version).unwrap()
        }

        pub fn get_current_display(self) -> Vec<u8> {
            const CURRENT_DISPLAY_FILE: &str = "current";
            self.read_from_interface(CURRENT_DISPLAY_FILE)
        }

        pub fn get_display_state(self) -> Result<String, errors::Error> {
            const STATE_FILE: &str = "error";
            let state = self.read_from_interface(STATE_FILE);
            let state = String::from_utf8(state).unwrap();
            match &state as &str {
                "Ok" => Ok(String::from("Ok")),
                "Unsupported COG" => Err(errors::Error::new(errors::ErrorKind::DisplayUnsupportedCog)),
                "Panel broken" => Err(errors::Error::new(errors::ErrorKind::DisplayPanelBroken)),
                "DC Failed" => Err(errors::Error::new(errors::ErrorKind::DisplayDcFailed)),
                "Unknown" => Err(errors::Error::new(errors::ErrorKind::DisplayUnknown)),
                _ => Err(errors::Error::new(errors::ErrorKind::UnexpectedError(state))),
            }
        }

        fn read_from_interface(&self, interface: &str) -> Vec<u8> {
            let read_path = format!("{}/{}", self.interface, interface);
            let mut papirus_interface = File::open(read_path).unwrap();
            let mut content = Vec::new();
            papirus_interface.read_to_end(&mut content).unwrap();
            content
        }
    }

    impl Default for PapirusDisplay {
        fn default() -> PapirusDisplay {
            PapirusDisplay {
                interface: String::from("/dev/epd"),
            }
        }
    }

fn open_write_interface(interface: &str) -> Result<File, io::Error> {
    use std::fs::OpenOptions;
    OpenOptions::new()
        .write(true)
        .create_new(false)
        .open(interface)
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::interface::PapirusDisplay;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    #[serial]
    fn full_update_writes_u() {
        let display = PapirusDisplay::new("./test/dev/epd");
        display.full_update();
        let mut written = String::new();
        {
            File::open("./test/dev/epd/command").unwrap().read_to_string(&mut written).unwrap();
        }
        assert_eq!("U", written);
    }

    #[test]
    #[serial]
    fn partial_update_writes_p() {
        let display = PapirusDisplay::new("./test/dev/epd");
        display.partial_update();
        let mut written = String::new();
        {
            File::open("./test/dev/epd/command").unwrap().read_to_string(&mut written).unwrap();
        }
        assert_eq!("P", written);
    }

    #[test]
    #[serial]
    fn fast_update_writes_f() {
        let display = PapirusDisplay::new("./test/dev/epd");
        display.fast_update();
        let mut written = String::new();
        {
            File::open("./test/dev/epd/command").unwrap().read_to_string(&mut written).unwrap();
        }
        assert_eq!("F", written);
    }

    #[test]
    #[serial]
    fn clear_writes_c() {
        let display = PapirusDisplay::new("./test/dev/epd");
        display.clear();
        let mut written = String::new();
        {
            File::open("./test/dev/epd/command").unwrap().read_to_string(&mut written).unwrap();
        }
        assert_eq!("C", written);
    }

    #[test]
    #[serial]
    fn write_data_contains_data_afterwards() {
        let display = PapirusDisplay::new("./test/dev/epd");
        let data = vec![1, 2, 3];
        display.write_data(data);
        let mut buffer = Vec::new();
        File::open("./test/dev/epd/BE/display").unwrap().read_to_end(&mut buffer).unwrap();
        assert_eq!(vec![1, 2, 3], buffer);
    }
}
