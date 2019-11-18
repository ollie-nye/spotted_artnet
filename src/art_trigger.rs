use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtTrigger {
    header: ArtHeader,
    filler: u16,
    oem_code: u16,
    key: u8,
    sub_key: u8,
    data: [u8; 512],
}

impl ArtTrigger {
    pub fn new(oem_code: u16, key: u8, sub_key: u8, data: [u8; 512]) -> ArtTrigger {
        let header = ArtHeader::new(opcode::OP_TRIGGER);

        let filler = 0x0000;

        ArtTrigger {
            header,
            filler,
            oem_code,
            key,
            sub_key,
            data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.extend(self.filler.to_be_bytes().iter().cloned());
        out.extend(self.oem_code.to_be_bytes().iter().cloned());
        out.push(self.key);
        out.push(self.sub_key);
        out.extend(self.data.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtTrigger {
        let oem_code: u16 = ((data[2] as u16) << 8) | data[3] as u16;

        let key = data[4];
        let sub_key = data[5];

        let mut trigger_data: [u8; 512] = [0x00; 512];
        trigger_data.copy_from_slice(&data[6..518]);

        ArtTrigger::new(oem_code, key, sub_key, trigger_data)
    }
}
