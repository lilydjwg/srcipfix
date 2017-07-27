extern crate nfqueue;
extern crate pnet;

use pnet::packet::ipv4::MutableIpv4Packet;

fn queue_callback(msg: &nfqueue::Message, _state: &mut ()) {
  let mut data = msg.get_payload().to_owned();
  {
    let mut pkt = MutableIpv4Packet::new(&mut data).unwrap();
    pkt.set_source("202.118.17.142".parse().unwrap());
  }

  msg.set_verdict_full(nfqueue::Verdict::Accept, 0, &data);
  print!(".");
}

fn main() {
  let mut q = nfqueue::Queue::new(());
  println!("nfqueue example program: print packets metadata and accept packets");

  q.open();
  q.create_queue(0, queue_callback);
  q.set_mode(nfqueue::CopyMode::CopyPacket, 0xffff);

  q.run_loop();
  q.close();
}
