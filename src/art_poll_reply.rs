use super::header;

pub struct ArtPollReply {
  header: header::art_header::ArtHeaderPartial,
  ip_address: [u8; 4],
  port: u16,
  version_info: u16,
  net_switch: u8,
  sub_switch: u8,
  oem: u16,
  ubea_version: u8,
  status1: u8,
  esta_man: u16,
  short_name: [u8; 18],
  long_name: [u8; 64],
  node_report: [u8; 64],
  number_ports: u16,
  port_types: [u8; 4],
  good_input: [u8; 4],
  good_output: [u8; 4],
  sw_in: [u8; 4],
  sw_out: [u8; 4],
  sw_video: u8,
  sw_macro: u8,
  sw_remote: u8,
  spare: [u8; 3],
  style: u8,
  mac: [u8; 6],
  bind_ip: [u8; 4],
  bind_index: u8,
  status2: u8,
  filler: [u8; 26]
}

impl ArtPollReply {
  pub fn new(
    ip_address: [u8; 4],
    port: u16,
    net_switch: u8,
    sub_switch: u8,
    oem: u16,
    status1: u8,
    esta_man: u16,
    short_name: [u8; 18],
    long_name: [u8; 64],
    node_report: [u8; 64],
    number_ports: u16,
    port_types: [u8; 4],
    good_input: [u8; 4],
    good_output: [u8; 4],
    sw_in: [u8; 4],
    sw_out: [u8; 4],
    style: u8,
    mac: [u8; 6],
    bind_ip: [u8; 4],
    bind_index: u8,
    status2: u8
  ) -> ArtPollReply {
    let header = header::art_header::ArtHeaderPartial::new(header::opcode::OP_POLL_REPLY);

    let version_info = 0x0001;
    let ubea_version = 0x00;
    let sw_video = 0x00;
    let sw_macro = 0x00;
    let sw_remote = 0x00;
    let spare: [u8; 3] = [0x00; 3];
    let filler: [u8; 26] = [0x00; 26];

    let art_poll_reply = ArtPollReply {
      header,
      ip_address,
      port,
      version_info,
      net_switch,
      sub_switch,
      oem,
      ubea_version,
      status1,
      esta_man,
      short_name,
      long_name,
      node_report,
      number_ports,
      port_types,
      good_input,
      good_output,
      sw_in,
      sw_out,
      sw_video,
      sw_macro,
      sw_remote,
      spare,
      style,
      mac,
      bind_ip,
      bind_index,
      status2,
      filler
    };

    art_poll_reply
  }

  pub fn serialize(&self) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend(self.header.serialize().iter().cloned());
    out.extend(self.ip_address.iter().cloned());
    out.extend(self.port.to_be_bytes().iter().cloned());
    out.extend(self.version_info.to_be_bytes().iter().cloned());
    out.push(self.net_switch);
    out.push(self.sub_switch);
    out.extend(self.oem.to_be_bytes().iter().cloned());
    out.push(self.ubea_version);
    out.push(self.status1);
    out.extend(self.esta_man.to_be_bytes().iter().cloned());
    out.extend(self.short_name.iter().cloned());
    out.extend(self.long_name.iter().cloned());
    out.extend(self.node_report.iter().cloned());
    out.extend(self.number_ports.to_be_bytes().iter().cloned());
    out.extend(self.port_types.iter().cloned());
    out.extend(self.good_input.iter().cloned());
    out.extend(self.good_output.iter().cloned());
    out.extend(self.sw_in.iter().cloned());
    out.extend(self.sw_out.iter().cloned());
    out.push(self.sw_video);
    out.push(self.sw_macro);
    out.push(self.sw_remote);
    out.extend(self.spare.iter().cloned());
    out.push(self.style);
    out.extend(self.mac.iter().cloned());
    out.extend(self.bind_ip.iter().cloned());
    out.push(self.bind_index);
    out.push(self.status2);
    out.extend(self.filler.iter().cloned());

    out
  }
}
