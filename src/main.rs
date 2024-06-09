mod interface;
mod reader;
mod command;
mod parser;

use input::Libinput;
use crate::interface::Interface;
use crate::reader::Reader;


fn main() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    let mut reader = Reader::new(input);
    reader.start_loop();
}

