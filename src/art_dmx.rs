use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtDmx {
    header: ArtHeader,
    sequence: u8,
    physical: u8,
    sub_universe: u8,
    net: u8,
    length: u16,
    data: [u8; 512],
}

impl ArtDmx {
    pub fn new(
        sequence: u8,
        physical: u8,
        sub_universe: u8,
        net: u8,
        length: u16,
        data: [u8; 512],
    ) -> ArtDmx {
        let header = ArtHeader::new(opcode::OP_OUTPUT);

        ArtDmx {
            header,
            sequence,
            physical,
            sub_universe,
            net,
            length,
            data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.push(self.sequence);
        out.push(self.physical);
        out.push(self.sub_universe);
        out.push(self.net);
        out.extend(self.length.to_be_bytes().iter().cloned());
        out.extend(self.data.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtDmx {
        let sequence = data[0];
        let physical = data[1];
        let sub_universe = data[2];
        let net = data[3];

        let length: u16 = ((data[4] as u16) << 8) | data[5] as u16;
        
        let mut dmx_data: [u8; 512] = [0x00; 512];
        dmx_data.copy_from_slice(&data[6..518]);

        ArtDmx::new(sequence, physical, sub_universe, net, length, dmx_data)
    }
}
