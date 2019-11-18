use super::header;

pub struct ArtDmx {
  header: header::art_header::ArtHeader,
  sequence: u8,
  physical: u8,
  sub_universe: u8,
  net: u8,
  length: u16,
  data: [u8; 512]
}

impl ArtDmx {
  pub fn new(sequence: u8, physical: u8, sub_universe: u8, net: u8, length: u16, data: [u8; 512]) -> ArtDmx {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_OUTPUT);

    let art_dmx = ArtDmx { header, sequence, physical, sub_universe, net, length, data };

    art_dmx
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.push(self.sequence);
    out.push(self.physical);
    out.push(self.sub_universe);
    out.push(self.net);
    out.extend(self.length.to_be_bytes().iter().cloned());
    out.extend(self.data.iter().cloned());

    out
  }
}
