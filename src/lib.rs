pub mod header;

pub mod art_address;
pub mod art_command;
pub mod art_diag_data;
pub mod art_dmx;
pub mod art_input;
pub mod art_ip_prog;
pub mod art_ip_prog_reply;
pub mod art_poll;
pub mod art_poll_reply;
pub mod art_sync;
pub mod art_trigger;

use header::art_header::ArtHeader;
use header::opcode;

use art_address::ArtAddress;
use art_command::ArtCommand;
use art_diag_data::ArtDiagData;
use art_dmx::ArtDmx;
use art_input::ArtInput;
use art_ip_prog::ArtIpProg;
use art_ip_prog_reply::ArtIpProgReply;
use art_poll::ArtPoll;
use art_poll_reply::ArtPollReply;
use art_sync::ArtSync;
use art_trigger::ArtTrigger;

pub struct ArtPacket {
    header: header::art_header::ArtHeader,
    data: [u8; 65507 - 12],
}

impl ArtPacket {
    pub fn deserialize(content: &[u8]) -> ArtPacket {
        let id = &content[0..8];

        let mut header = ArtHeader::new(0x0000);

        let mut data = [0x00; 65507 - 12];

        if id == header::art_header::id() {
            let opcode: u16 = ((content[8] as u16) << 8) | content[9] as u16;

            header = ArtHeader::new(opcode);

            let trim_content = &content[12..];

            let mut i = 0;
            for cell in trim_content {
                data[i] = *cell;
                i += 1;
            }
        }

        ArtPacket { header, data }
    }

    pub fn is_art_packet(&self) -> bool {
        self.header.opcode() != 0x0000
    }

    pub fn is_address(&self) -> bool {
        self.header.opcode() == opcode::OP_ADDRESS
    }

    pub fn is_command(&self) -> bool {
        self.header.opcode() == opcode::OP_COMMAND
    }

    pub fn is_diag_data(&self) -> bool {
        self.header.opcode() == opcode::OP_DIAG_DATA
    }

    pub fn is_dmx(&self) -> bool {
        self.header.opcode() == opcode::OP_DMX
    }

    pub fn is_input(&self) -> bool {
        self.header.opcode() == opcode::OP_INPUT
    }

    pub fn is_ip_prog_reply(&self) -> bool {
        self.header.opcode() == opcode::OP_IP_PROG_REPLY
    }

    pub fn is_ip_prog(&self) -> bool {
        self.header.opcode() == opcode::OP_IP_PROG
    }

    pub fn is_poll_reply(&self) -> bool {
        self.header.opcode() == opcode::OP_POLL_REPLY
    }

    pub fn is_poll(&self) -> bool {
        self.header.opcode() == opcode::OP_POLL
    }

    pub fn is_sync(&self) -> bool {
        self.header.opcode() == opcode::OP_SYNC
    }

    pub fn is_trigger(&self) -> bool {
        self.header.opcode() == opcode::OP_TRIGGER
    }

    pub fn into_address(self) -> ArtAddress {
        ArtAddress::deserialize(&self.data)
    }

    pub fn into_command(self) -> ArtCommand {
        ArtCommand::deserialize(&self.data)
    }

    pub fn into_diag_data(self) -> ArtDiagData {
        ArtDiagData::deserialize(&self.data)
    }

    pub fn into_dmx(self) -> ArtDmx {
        ArtDmx::deserialize(&self.data)
    }

    pub fn into_input(self) -> ArtInput {
        ArtInput::deserialize(&self.data)
    }

    pub fn into_ip_prog_reply(self) -> ArtIpProgReply {
        ArtIpProgReply::deserialize(&self.data)
    }

    pub fn into_ip_prog(self) -> ArtIpProg {
        ArtIpProg::deserialize(&self.data)
    }

    pub fn into_poll_reply(self) -> ArtPollReply {
        ArtPollReply::deserialize(&self.data)
    }

    pub fn into_poll(self) -> ArtPoll {
        ArtPoll::deserialize(&self.data)
    }

    pub fn into_sync(self) -> ArtSync {
        ArtSync::deserialize(&self.data)
    }

    pub fn into_trigger(self) -> ArtTrigger {
        ArtTrigger::deserialize(&self.data)
    }
}

#[cfg(test)]
#[test]
fn test_new_is_art_packet() {
    let header = header::art_header::ArtHeader::new(0x0000);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_art_packet(), false);
}

#[test]
fn test_new_is_address() {
    let header = header::art_header::ArtHeader::new(0x0060);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_address(), true);
}

#[test]
fn test_new_is_command() {
    let header = header::art_header::ArtHeader::new(0x0024);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_command(), true);
}

#[test]
fn test_new_is_diag_data() {
    let header = header::art_header::ArtHeader::new(0x0023);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_diag_data(), true);
}

#[test]
fn test_new_is_dmx() {
    let header = header::art_header::ArtHeader::new(0x0050);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_dmx(), true);
}

#[test]
fn test_new_is_input() {
    let header = header::art_header::ArtHeader::new(0x0070);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_input(), true);
}

#[test]
fn test_new_is_ip_prog_reply() {
    let header = header::art_header::ArtHeader::new(0x00f9);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_ip_prog_reply(), true);
}

#[test]
fn test_new_is_ip_prog() {
    let header = header::art_header::ArtHeader::new(0x00f8);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_ip_prog(), true);
}

#[test]
fn test_new_is_poll_reply() {
    let header = header::art_header::ArtHeader::new(0x0021);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_poll_reply(), true);
}

#[test]
fn test_new_is_poll() {
    let header = header::art_header::ArtHeader::new(0x0020);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_poll(), true);
}

#[test]
fn test_new_is_sync() {
    let header = header::art_header::ArtHeader::new(0x0052);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_sync(), true);
}

#[test]
fn test_new_is_trigger() {
    let header = header::art_header::ArtHeader::new(0x0099);
    let bytes = header.serialize();
    let packet = ArtPacket::deserialize(&bytes);

    assert_eq!(packet.is_trigger(), true);
}
