use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtAddress {
    header: ArtHeader,
    net_switch: u8,
    bind_index: u8,
    short_name: [u8; 18],
    long_name: [u8; 64],
    sw_in: [u8; 4],
    sw_out: [u8; 4],
    sub_switch: u8,
    sw_video: u8,
    command: u8,
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
        command: u8,
    ) -> ArtAddress {
        let header = ArtHeader::new(opcode::OP_ADDRESS);

        ArtAddress {
            header,
            net_switch,
            bind_index,
            short_name,
            long_name,
            sw_in,
            sw_out,
            sub_switch,
            sw_video,
            command,
        }
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

    pub fn deserialize(data: &[u8]) -> ArtAddress {
        let net_switch = data[0];
        let bind_index = data[1];

        let mut short_name: [u8; 18] = [0x00; 18];
        short_name.copy_from_slice(&data[2..20]);

        let mut long_name: [u8; 64] = [0x00; 64];
        long_name.copy_from_slice(&data[20..84]);

        let mut sw_in: [u8; 4] = [0x00; 4];
        sw_in.copy_from_slice(&data[84..88]);

        let mut sw_out: [u8; 4] = [0x00; 4];
        sw_out.copy_from_slice(&data[88..92]);

        let sub_switch = data[92];
        let sw_video = data[93];
        let command = data[94];

        ArtAddress::new(
            net_switch, bind_index, short_name, long_name, sw_in, sw_out, sub_switch, sw_video,
            command,
        )
    }
}
