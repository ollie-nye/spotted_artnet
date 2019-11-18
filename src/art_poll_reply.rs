use super::header::art_header::ArtHeaderPartial;
use super::header::opcode;

pub struct ArtPollReply {
    header: ArtHeaderPartial,
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
    filler: [u8; 26],
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
        status2: u8,
    ) -> ArtPollReply {
        let header = ArtHeaderPartial::new(opcode::OP_POLL_REPLY);

        let version_info = 0x0001;
        let ubea_version = 0x00;
        let sw_video = 0x00;
        let sw_macro = 0x00;
        let sw_remote = 0x00;
        let spare: [u8; 3] = [0x00; 3];
        let filler: [u8; 26] = [0x00; 26];

        ArtPollReply {
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
            filler,
        }
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

    pub fn deserialize(data: &[u8]) -> ArtPollReply {
        let mut ip_address: [u8; 4] = [0x00; 4];
        ip_address.copy_from_slice(&data[0..4]);

        let port: u16 = ((data[4] as u16) << 8) | data[5] as u16;

        let net_switch = data[6];
        let sub_switch = data[7];

        let oem: u16 = ((data[8] as u16) << 8) | data[9] as u16;

        let status1 = data[10];

        let esta_man: u16 = ((data[11] as u16) << 8) | data[12] as u16;

        let mut short_name: [u8; 18] = [0x00; 18];
        short_name.copy_from_slice(&data[13..31]);

        let mut long_name: [u8; 64] = [0x00; 64];
        long_name.copy_from_slice(&data[31..95]);

        let mut node_report: [u8; 64] = [0x00; 64];
        node_report.copy_from_slice(&data[95..159]);

        let number_ports: u16 = ((data[159] as u16) << 8) | data[160] as u16;

        let mut port_types: [u8; 4] = [0x00; 4];
        port_types.copy_from_slice(&data[161..165]);

        let mut good_input: [u8; 4] = [0x00; 4];
        good_input.copy_from_slice(&data[165..169]);

        let mut good_output: [u8; 4] = [0x00; 4];
        good_output.copy_from_slice(&data[169..173]);

        let mut sw_in: [u8; 4] = [0x00; 4];
        sw_in.copy_from_slice(&data[173..177]);

        let mut sw_out: [u8; 4] = [0x00; 4];
        sw_out.copy_from_slice(&data[177..181]);

        let style = data[181];

        let mut mac: [u8; 6] = [0x00; 6];
        mac.copy_from_slice(&data[182..188]);

        let mut bind_ip: [u8; 4] = [0x00; 4];
        bind_ip.copy_from_slice(&data[188..192]);

        let bind_index = data[192];
        let status2 = data[193];

        ArtPollReply::new(
            ip_address,
            port,
            net_switch,
            sub_switch,
            oem,
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
            style,
            mac,
            bind_ip,
            bind_index,
            status2
        )
    }
}
