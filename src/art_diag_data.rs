use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtDiagData {
    header: ArtHeader,
    filler1: u8,
    priority: u8,
    filler2: [u8; 2],
    length: u16,
    data: [u8; 512],
}

impl ArtDiagData {
    pub fn new(priority: u8, length: u16, data: [u8; 512]) -> ArtDiagData {
        let header = ArtHeader::new(opcode::OP_DIAG_DATA);

        let filler1 = 0x00;
        let filler2 = [0x00; 2];

        ArtDiagData {
            header,
            filler1,
            priority,
            filler2,
            length,
            data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.push(self.filler1);
        out.push(self.priority);
        out.extend(self.filler2.iter().cloned());
        out.extend(self.length.to_be_bytes().iter().cloned());
        out.extend(self.data.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtDiagData {
        let priority = data[1];
        
        let length: u16 = ((data[4] as u16) << 8) | data[5] as u16;

        let mut diag_data: [u8; 512] = [0x00; 512];
        diag_data.copy_from_slice(&data[4..516]);

        ArtDiagData::new(priority, length, diag_data)
    }
}
