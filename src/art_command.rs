use super::header;

pub struct ArtCommand {
  header: header::art_header::ArtHeader,
  esta_man: u16,
  length: u16,
  data: [u8; 512]
}

impl ArtCommand {
  pub fn new(esta_man: u16, length: u16, data: [u8; 512]) -> ArtCommand {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_COMMAND);

    let art_command = ArtCommand { header, esta_man, length, data };

    art_command
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.extend(self.esta_man.to_be_bytes().iter().cloned());
    out.extend(self.length.to_be_bytes().iter().cloned());
    out.extend(self.data.iter().cloned());

    out
  }
}
