use dbus::{arg, BusType, Connection, Message, SignalArgs};

#[derive(Debug, Default)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties:
        ::std::collections::HashMap<String, arg::Variant<Box<arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
    fn get(&mut self, i: &mut arg::Iter) -> Result<(), arg::TypeMismatchError> {
        self.interface_name = i.read()?;
        self.changed_properties = i.read()?;
        self.invalidated_properties = i.read()?;
        Ok(())
    }
}

fn process_msg(msg: Message) {
    if let Some(sig) = OrgFreedesktopDBusPropertiesPropertiesChanged::from_message(&msg) {
        let path = msg.path().unwrap_or_default();
        if !path.starts_with("/org/bluez/hci0/dev_") {
            return;
        }
        if sig.interface_name != "org.bluez.Device1" {
            return;
        }
        if let Some(arg::Variant(val)) = sig.changed_properties.get("Connected") {
            if let Some(x) = val.as_u64() {
                println!("{}", if x == 0 { "Disconnected" } else { "Connected" });
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::get_private(BusType::System)?;
    conn.add_match(&OrgFreedesktopDBusPropertiesPropertiesChanged::match_str(
        None, None,
    ))?;

    loop {
        for msg in conn.incoming(60 * 60 * 1000) {
            process_msg(msg);
        }
        println!("Got nothing for an hour, loop");
    }
}
