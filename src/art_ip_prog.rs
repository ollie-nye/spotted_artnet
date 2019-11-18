use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtIpProg {
    header: ArtHeader,
    filler1: u16,
    command: u8,
    filler2: u8,
    ip_address: [u8; 4],
    subnet_mask: [u8; 4],
    port: u16,
    spare: [u8; 8],
}

impl ArtIpProg {
    pub fn new(command: u8, ip_address: [u8; 4], subnet_mask: [u8; 4], port: u16) -> ArtIpProg {
        let header = ArtHeader::new(opcode::OP_IP_PROG);

        let filler1 = 0x0000;
        let filler2 = 0x00;
        let spare: [u8; 8] = [0x00; 8];

        ArtIpProg {
            header,
            filler1,
            command,
            filler2,
            ip_address,
            subnet_mask,
            port,
            spare,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.extend(self.filler1.to_be_bytes().iter().cloned());
        out.push(self.command);
        out.push(self.filler2);
        out.extend(self.ip_address.iter().cloned());
        out.extend(self.subnet_mask.iter().cloned());
        out.extend(self.port.to_be_bytes().iter().cloned());
        out.extend(self.spare.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtIpProg {
        let command = data[2];

        let mut ip_address: [u8; 4] = [0x00; 4];
        ip_address.copy_from_slice(&data[4..8]);

        let mut subnet_mask: [u8; 4] = [0x00; 4];
        subnet_mask.copy_from_slice(&data[8..12]);

        let port: u16 = ((data[12] as u16) << 8) | data[13] as u16;

        ArtIpProg::new(command, ip_address, subnet_mask, port)
    }
}
