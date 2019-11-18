use super::header;

pub struct ArtTrigger {
  header: header::art_header::ArtHeader,
  filler: u16,
  oem_code: u16,
  key: u8,
  sub_key: u8,
  data: [u8; 512]
}

impl ArtTrigger {
  pub fn new(oem_code: u16, key: u8, sub_key: u8, data: [u8; 512]) -> ArtTrigger {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_TRIGGER);

    let filler = 0x0000;

    let art_trigger = ArtTrigger { header, filler, oem_code, key, sub_key, data };

    art_trigger
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.extend(self.filler.to_be_bytes().iter().cloned());
    out.extend(self.oem_code.to_be_bytes().iter().cloned());
    out.push(self.key);
    out.push(self.sub_key);
    out.extend(self.data.iter().cloned());

    out
  }
}
