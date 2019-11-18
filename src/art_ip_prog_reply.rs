use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtIpProgReply {
    header: ArtHeader,
    filler: [u8; 4],
    ip_address: [u8; 4],
    subnet_mask: [u8; 4],
    port: u16,
    status: u8,
    spare: [u8; 7],
}

impl ArtIpProgReply {
    pub fn new(ip_address: [u8; 4], subnet_mask: [u8; 4], port: u16, status: u8) -> ArtIpProgReply {
        let header = ArtHeader::new(opcode::OP_IP_PROG);

        let filler: [u8; 4] = [0x00; 4];
        let spare: [u8; 7] = [0x00; 7];

        ArtIpProgReply {
            header,
            filler,
            ip_address,
            subnet_mask,
            port,
            status,
            spare,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.extend(self.filler.iter().cloned());
        out.extend(self.ip_address.iter().cloned());
        out.extend(self.subnet_mask.iter().cloned());
        out.extend(self.port.to_be_bytes().iter().cloned());
        out.push(self.status);
        out.extend(self.spare.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtIpProgReply {
        let mut ip_address: [u8; 4] = [0x00; 4];
        ip_address.copy_from_slice(&data[4..8]);

        let mut subnet_mask: [u8; 4] = [0x00; 4];
        subnet_mask.copy_from_slice(&data[8..12]);

        let port: u16 = ((data[12] as u16) << 8) | data[13] as u16;

        let status = data[14];

        ArtIpProgReply::new(ip_address, subnet_mask, port, status)
    }
}
