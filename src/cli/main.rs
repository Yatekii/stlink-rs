use stlink;
use dbg_probe;
use std::process;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("ST-Link CLI")
                        .version("0.1.0")
                        .author("Noah Hüsser. <yatekii@yatekii.ch>")
                        .about("Get info about the connected ST-Links")
                        .subcommand(
                            SubCommand::with_name("list")
                                        .about("List all connected ST-Links")
                        )
                        .subcommand(
                            SubCommand::with_name("info")
                                        .about("Gets infos about the selected ST-Link")
                                        .arg(
                                            Arg::with_name("n")
                                                .help("The number associated with the ST-Link to use")
                                                .required(true)
                                        )
                        )
                        .subcommand(
                            SubCommand::with_name("reset")
                                        .about("Resets the target attached to the selected ST-Link")
                                        .arg(
                                            Arg::with_name("n")
                                                .help("The number associated with the ST-Link to use")
                                                .required(true)
                                        )
                                        .arg(
                                            Arg::with_name("assert")
                                                .help("Whether the reset pin should be asserted or deasserted. If left open, just pulse it.")
                                                .required(false)
                                        )
                        )
                        .get_matches();

    if let Some(_) = matches.subcommand_matches("list") {
        list_connected_devices();
    }

    if let Some(matches) = matches.subcommand_matches("info") {
        let number = matches.value_of("n").unwrap().parse::<u8>().unwrap();
        let _ = show_info_of_device(number);
    }

    if let Some(matches) = matches.subcommand_matches("reset") {
        let number = matches.value_of("n").unwrap().parse::<u8>().unwrap();
        let assert = matches.value_of("assert").map(|v| if v == "true" { true } else { false });
        let result = reset_target_of_device(number, assert);
        println!("{:?}", result);
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

#[derive(Debug)]
enum Error {
    USB(libusb::Error),
    DeviceNotFound,
    STLinkError(stlink::STLinkError)
}

fn show_info_of_device(n: u8) -> Result<(), Error> {
    let mut context = libusb::Context::new().or_else(|e| { println!("Failed to open an USB context."); Err(Error::USB(e)) })?;
    let mut connected_devices = stlink::get_all_plugged_devices(&mut context).or_else(|e| { println!("Failed to fetch plugged USB devices."); Err(Error::USB(e)) })?;
    if connected_devices.len() <= n as usize {
        println!("The device with the given number was not found.");
        Err(Error::DeviceNotFound)
    } else {
        Ok(())
    }?;
    let usb_device = connected_devices.remove(n as usize);
    let mut st_link = stlink::STLink::new(usb_device);
    println!("{:?}", st_link.open());
    let version = st_link.get_version();
    let vtg = st_link.get_target_voltage();
    println!("{:?} {:?}", version.ok(), vtg.ok());
    let res = st_link.set_swd_frequency(stlink::constants::SwdFrequencyToDelayCount::Hz4600000);
    println!("{:?}", res);
    let res = st_link.target_reset();
    println!("{:?}", res);
    let res = st_link.enter_debug(dbg_probe::protocol::WireProtocol::Swd);
    println!("{:?}", res);
    let res = st_link.drive_nreset(false);
    println!("{:?}", res);
    let res = st_link.close();
    println!("{:?}", res);
    Ok(())
}

fn reset_target_of_device(n: u8, assert: Option<bool>) -> Result<(), Error> {
    let mut context = libusb::Context::new().or_else(|e| { println!("Failed to open an USB context."); Err(Error::USB(e)) })?;
    let mut connected_devices = stlink::get_all_plugged_devices(&mut context).or_else(|e| { println!("Failed to fetch plugged USB devices."); Err(Error::USB(e)) })?;
    if connected_devices.len() <= n as usize {
        println!("The device with the given number was not found.");
        Err(Error::DeviceNotFound)
    } else {
        Ok(())
    }?;
    let usb_device = connected_devices.remove(n as usize);
    let mut st_link = stlink::STLink::new(usb_device);
    st_link.open().or_else(|e| Err(Error::STLinkError(e)))?;
    
    if let Some(assert) = assert {
        println!("{} target reset.", if assert { "Asserting" } else { "Deasserting" });
        st_link.drive_nreset(assert).or_else(|e| Err(Error::STLinkError(e)))?;
        println!("Target reset has been {}.", if assert { "asserted" } else { "deasserted" });
        st_link.close().or_else(|e| Err(Error::STLinkError(e)))?;
    } else {
        println!("Triggering target reset.");
        st_link.target_reset().or_else(|e| Err(Error::STLinkError(e)))?;
        println!("Target reset has been triggered.");
    }
    Ok(())
}