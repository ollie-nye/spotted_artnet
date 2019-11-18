use super::header;

pub struct ArtIpProg {
  header: header::art_header::ArtHeader,
  filler1: u16,
  command: u8,
  filler2: u8,
  ip_address: [u8; 4],
  subnet_mask: [u8; 4],
  port: u16,
  spare: [u8; 8]
}

impl ArtIpProg {
  pub fn new(command: u8, ip_address: [u8; 4], subnet_mask: [u8; 4], port: u16) -> ArtIpProg {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_IP_PROG);

    let filler1 = 0x0000;
    let filler2 = 0x00;
    let spare: [u8; 8] = [0x00; 8];

    let art_ip_prog = ArtIpProg { header, filler1, command, filler2, ip_address, subnet_mask, port, spare };

    art_ip_prog
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.extend(self.filler1.to_be_bytes().iter().cloned());
    out.push(self.command);
    out.push(self.filler2);
    out.extend(self.ip_address.iter().cloned());
    out.extend(self.subnet_mask.iter().cloned());
    out.extend(self.port.to_be_bytes().iter().cloned());
    out.extend(self.spare.iter().cloned());

    out
  }
}
