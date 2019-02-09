use stlink;
use dbg_probe;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("ST-Link CLI")
                        .version("0.1.0")
                        .author("Noah Hüsser. <yatekii@yatekii.ch>")
                        .about("Get info about the connected ST-Links")
                        .arg(
                            Arg::with_name("list")
                                .short("l")
                                .help("List all connected ST-Links")
                        )
                        .arg(
                            Arg::with_name("info")
                                .short("i")
                                .help("Gets infos about the selcted ST-Link")
                                .takes_value(true)
                        )
                        .subcommand(
                            SubCommand::with_name("list")
                                        .about("List all connected ST-Links")
                        )
                        .subcommand(
                            SubCommand::with_name("info")
                                        .about("Gets infos about the selcted ST-Link")
                                        .arg(
                                            Arg::with_name("n")
                                                .help("The number associated with the ST-Link to use")
                                                .required(true)
                                        )
                        )
                        .get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        list_connected_devices();
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        let number = matches.value_of("n").unwrap().parse::<u8>().unwrap();
        show_info_of_device(number);
    }
}

fn list_connected_devices() {
    let mut context = libusb::Context::new().unwrap();
    match stlink::get_all_plugged_devices(&mut context) {
        Ok(connected_stlinks) => {
            println!("The following devices were found:");
            connected_stlinks.iter().enumerate().for_each(|(num, link)| {
                println!("[{}]: PID = {}, version = {}", num, link.info.usb_pid, link.info.version_name);
            });
        },
        Err(e) => { println!("{}", e); }
    };
}

fn show_info_of_device(n: u8) {
    let mut context = libusb::Context::new().unwrap();
    let usb_device = stlink::get_all_plugged_devices(&mut context).unwrap().remove(0);
    let mut st_link = stlink::STLink::new(usb_device);
    st_link.open();
    let version = st_link.get_version();
    let vtg = st_link.get_target_voltage();
    println!("{:?} {:?}", version.ok(), vtg.ok());
    let res = st_link.set_swd_frequency(stlink::constants::SwdFrequencyToDelayCount::Hz4600000);
    println!("{:?}", res);
    let res = st_link.target_reset();
    println!("{:?}", res);
    let res = st_link.enter_debug(dbg_probe::protocol::WireProtocol::Swd);
    println!("{:?}", res);
    st_link.close();
}