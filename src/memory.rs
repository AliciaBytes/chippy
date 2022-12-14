const FONT_DATA: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Allocate a vector with 4096 bytes as ram for the emulator.
/// The font data is already written to the memory range 0x50 to 0x9F.
#[must_use]
pub fn allocate_ram() -> Vec<u8> {
    let mut memory = vec![0u8; 4096];

    write_font_to_ram(&mut memory[0x50..=0x9F]);

    memory
}

/// Write the standard font data to a memory range.
fn write_font_to_ram(memory: &mut [u8]) {
    assert!(
        memory.len() == FONT_DATA.len(),
        "Only received a slice of {} bytes, but need {} bytes to write the font to memory.",
        memory.len(),
        FONT_DATA.len()
    );

    memory.copy_from_slice(&FONT_DATA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_allocating_memory_works() {
        let _ram = allocate_ram();
    }
}
