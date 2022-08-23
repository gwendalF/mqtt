use mqtt::packet;

fn main() {
    let p = packet::packet::Packet::Connect;
    println!("P: {:?}", p);
}
