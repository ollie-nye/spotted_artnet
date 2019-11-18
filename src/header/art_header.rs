pub struct ArtHeader {
  id: [u8; 8],
  opcode: u16,
  protocol_version: u16
}

impl ArtHeader {
  pub fn new(opcode: u16) -> ArtHeader {

    let mut id: [u8; 8] = [0x00; 8];
    id.copy_from_slice("Art-Net\0".as_bytes());

    let protocol_version: u16 = 0x000e;

    let header = ArtHeader { id, opcode, protocol_version };

    header
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend(self.id.iter().cloned());
    out.extend(self.opcode.to_be_bytes().iter().cloned());
    out.extend(self.protocol_version.to_be_bytes().iter().cloned());

    out
  }
}

pub struct ArtHeaderPartial {
  id: [u8; 8],
  opcode: u16
}

impl ArtHeaderPartial {
  pub fn new(opcode: u16) -> ArtHeaderPartial {

    let mut id: [u8; 8] = [0x00; 8];
    id.copy_from_slice("Art-Net\0".as_bytes());

    let header = ArtHeaderPartial { id, opcode };

    header
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();
    out.push(self.id[0]);
    out.push(self.id[1]);
    out.push(self.id[2]);
    out.push(self.id[3]);
    out.push(self.id[4]);
    out.push(self.id[5]);
    out.push(self.id[6]);
    out.push(self.id[7]);
    out.extend(self.opcode.to_be_bytes().iter().cloned());

    out
  }
}
