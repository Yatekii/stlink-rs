use stlink;

fn main() {
    let mut context = libusb::Context::new().unwrap();
    let usb_device = stlink::STLinkUSBDevice::get_all_plugged_devices(&mut context).unwrap().remove(0);
    let mut st_link = stlink::STLink::new(usb_device);
    st_link.open();
    let version = st_link.get_version();
    let vtg = st_link.get_target_voltage();
    println!("{:?} {:?}", version.ok(), vtg.ok());
}