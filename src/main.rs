use pnet::datalink::{
    NetworkInterface,
    Channel::Ethernet,
};

fn main() {
    let interface_name = "wlan0";
    let interface = get_network_interface(interface_name).unwrap();

    let (_, mut rx) = match pnet::datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to create channel: {e}", ),
    };

    println!("Listening on interface: {}", interface.name);

    loop {
        match rx.next() {
            Ok(packet) => {
                println!("Received packet: {packet:?}");
            }
            Err(e) => {
                eprintln!("Error receiving packet: {e}");
                break;
            }
        }
    }
}

pub fn get_network_interface(interface_name: &str) -> Result<NetworkInterface, String> {
    // ネットワークインタフェースの一覧を取得し、名前が一致するものを返す
    for iface in pnet::datalink::interfaces() {
        if iface.name == interface_name {
            return Ok(iface);
        }
    }
    Err(String::from("failed to get network interface"))
}