use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtInput {
    header: ArtHeader,
    filler: u8,
    bind_index: u8,
    number_ports: u16,
    input: [u8; 4],
}

impl ArtInput {
    pub fn new(bind_index: u8, number_ports: u16, input: [u8; 4]) -> ArtInput {
        let header = ArtHeader::new(opcode::OP_INPUT);

        let filler = 0x00;

        ArtInput {
            header,
            filler,
            bind_index,
            number_ports,
            input,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.push(self.filler);
        out.push(self.bind_index);
        out.extend(self.number_ports.to_be_bytes().iter().cloned());
        out.extend(self.input.iter().cloned());

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtInput {
        let bind_index = data[1];
        
        let number_ports: u16 = ((data[2] as u16) << 8) | data[3] as u16;

        let mut input: [u8; 4] = [0x00; 4];
        input.copy_from_slice(&data[4..8]);

        ArtInput::new(bind_index, number_ports, input)
    }
}
