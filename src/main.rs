use std::io::{self, Write};

use pnet_packet::Packet;
use pnet_packet::ipv4::MutableIpv4Packet;
use pnet_packet::udp::MutableUdpPacket;

fn queue_callback(msg: &nfqueue::Message, _state: &mut ()) {
  let mut data = msg.get_payload().to_owned();
  let udp_offset = {
    let ip_len = data.len();
    let mut pkt = MutableIpv4Packet::new(&mut data).unwrap();
    pkt.set_source("202.118.17.142".parse().unwrap());
    ip_len - pkt.payload().len()
  };
  {
    let mut pkt = MutableUdpPacket::new(&mut data[udp_offset..]).unwrap();
    pkt.set_checksum(0);
  }

  msg.set_verdict_full(nfqueue::Verdict::Accept, 0, &data);
  print!(".");
  io::stdout().flush().unwrap();
}

fn main() {
  let mut q = nfqueue::Queue::new(());

  q.open();
  q.create_queue(1, queue_callback);
  q.set_mode(nfqueue::CopyMode::CopyPacket, 0xffff);

  q.run_loop();
  q.close();
}
