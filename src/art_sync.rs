use super::header;

pub struct ArtSync {
  header: header::art_header::ArtHeader,
  aux1: u8,
  aux2: u8
}

impl ArtSync {
  pub fn new(aux1: u8, aux2: u8) -> ArtSync {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_SYNC);

    let art_sync = ArtSync { header, aux1, aux2 };

    art_sync
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.push(self.aux1);
    out.push(self.aux2);

    out
  }
}
