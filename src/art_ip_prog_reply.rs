use super::header;

pub struct ArtIpProgReply {
  header: header::art_header::ArtHeader,
  filler: [u8; 4],
  ip_address: [u8; 4],
  subnet_mask: [u8; 4],
  port: u16,
  status: u8,
  spare: [u8; 7]
}

impl ArtIpProgReply {
  pub fn new(ip_address: [u8; 4], subnet_mask: [u8; 4], port: u16, status: u8) -> ArtIpProgReply {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_IP_PROG);

    let filler: [u8; 4] = [0x00; 4];
    let spare: [u8; 7] = [0x00; 7];

    let art_ip_prog_reply = ArtIpProgReply { header, filler, ip_address, subnet_mask, port, status, spare };

    art_ip_prog_reply
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.extend(self.filler.iter().cloned());
    out.extend(self.ip_address.iter().cloned());
    out.extend(self.subnet_mask.iter().cloned());
    out.extend(self.port.to_be_bytes().iter().cloned());
    out.push(self.status);
    out.extend(self.spare.iter().cloned());

    out
  }
}
