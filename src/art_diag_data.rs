use super::header;

pub struct ArtDiagData {
  header: header::art_header::ArtHeader,
  filler1: u8,
  priority: u8,
  filler2: [u8; 2],
  length: u16,
  data: [u8; 512]
}

impl ArtDiagData {
  pub fn new(priority: u8, length: u16, data: [u8; 512]) -> ArtDiagData {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_DIAG_DATA);

    let filler1 = 0x00;
    let filler2 = [0x00; 2];

    let art_diag_data = ArtDiagData { header, filler1, priority, filler2, length, data };

    art_diag_data
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.push(self.filler1);
    out.push(self.priority);
    out.extend(self.filler2.iter().cloned());
    out.extend(self.length.to_be_bytes().iter().cloned());
    out.extend(self.data.iter().cloned());

    out
  }
}
