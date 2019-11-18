use super::header;

pub struct ArtInput {
  header: header::art_header::ArtHeader,
  filler:u8,
  bind_index: u8,
  number_ports: u16,
  input: [u8; 4]
}

impl ArtInput {
  pub fn new(bind_index: u8, number_ports: u16, input: [u8; 4]) -> ArtInput {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_INPUT);

    let filler = 0x00;

    let art_input = ArtInput { header, filler, bind_index, number_ports, input };

    art_input
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.push(self.filler);
    out.push(self.bind_index);
    out.extend(self.number_ports.to_be_bytes().iter().cloned());
    out.extend(self.input.iter().cloned());

    out
  }
}
