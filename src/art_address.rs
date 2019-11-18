use super::header;

pub struct ArtAddress {
  header: header::art_header::ArtHeader,
  net_switch: u8,
  bind_index: u8,
  short_name: [u8; 18],
  long_name: [u8; 64],
  sw_in: [u8; 4],
  sw_out: [u8; 4],
  sub_switch: u8,
  sw_video: u8,
  command: u8
}

impl ArtAddress {
  pub fn new(
    net_switch: u8,
    bind_index: u8,
    short_name: [u8; 18],
    long_name: [u8; 64],
    sw_in: [u8; 4],
    sw_out: [u8; 4],
    sub_switch: u8,
    sw_video: u8,
    command: u8
  ) -> ArtAddress {
    let header = header::art_header::ArtHeader::new(header::opcode::OP_ADDRESS);

    let art_address = ArtAddress {
      header,
      net_switch,
      bind_index,
      short_name,
      long_name,
      sw_in,
      sw_out,
      sub_switch,
      sw_video,
      command
    };

    art_address
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.push(self.net_switch);
    out.push(self.bind_index);
    out.extend(self.short_name.iter().cloned());
    out.extend(self.long_name.iter().cloned());
    out.extend(self.sw_in.iter().cloned());
    out.extend(self.sw_out.iter().cloned());
    out.push(self.sub_switch);
    out.push(self.sw_video);
    out.push(self.command);

    out
  }
}
