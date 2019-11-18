use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtCommand {
    header: ArtHeader,
    esta_man: u16,
    length: u16,
    data: [u8; 512],
}

impl ArtCommand {
    pub fn new(esta_man: u16, length: u16, data: [u8; 512]) -> ArtCommand {
        let header = ArtHeader::new(opcode::OP_COMMAND);

        ArtCommand {
            header,
            esta_man,
            length,
            data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.extend(self.esta_man.to_be_bytes().iter().cloned());
        out.extend(self.length.to_be_bytes().iter().cloned());
        out.extend(self.data.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtCommand {
        let esta_man: u16 = ((data[0] as u16) << 8) | data[1] as u16;
        let length: u16 = ((data[2] as u16) << 8) | data[3] as u16;

        let mut cmd_data: [u8; 512] = [0x00; 512];
        cmd_data.copy_from_slice(&data[4..516]);

        ArtCommand::new(esta_man, length, cmd_data)
    }
}
