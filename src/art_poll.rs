use super::header::art_header::ArtHeader;
use super::header::opcode;

pub struct ArtPoll {
    header: ArtHeader,
    talk_to_me: u8,
    priority: u8,
}

impl ArtPoll {
    pub fn new(talk_to_me: u8, priority: u8) -> ArtPoll {
        let header = ArtHeader::new(opcode::OP_POLL);

        ArtPoll {
            header,
            talk_to_me,
            priority,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut out = Vec::new();

        out.extend(self.header.serialize().iter().cloned());
        out.push(self.talk_to_me);
        out.push(self.priority);

        out
    }

    pub fn deserialize(data: &[u8]) -> ArtPoll {
        let talk_to_me = data[0];
        let priority = data[1];

        ArtPoll::new(talk_to_me, priority)
    }
}
