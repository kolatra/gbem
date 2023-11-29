use super::*;
use crate::FlagBit::*;

#[test]
fn test_add() {
    let mut cpu = CPU::new();
    let instructions = get_instructions();
    let instruction = instructions
        .iter()
        .find(|i| i.mnemonic == "ADD A,B")
        .unwrap();

    // Test zero flag
    cpu.reg.a = 0;
    cpu.reg.b = 0;
    instruction.run(&mut cpu);

    assert!(cpu.is_set(Z));

    // Test half-carry flag
    cpu.reg.a = 62;
    cpu.reg.b = 34;
    instruction.run(&mut cpu);

    assert_eq!(cpu.reg.a, 96);
    assert!(cpu.is_set(H));

    // Test carry flag
    cpu.reg.a = 255;
    cpu.reg.b = 5;
    instruction.run(&mut cpu);

    assert!(cpu.is_set(C));
}

#[test]
fn test_sub() {
    let mut cpu = CPU::new();
    let instructions = get_instructions();
    let instruction = instructions
        .iter()
        .find(|i| i.mnemonic == "SUB A,B")
        .unwrap();

    // Test zero flag
    cpu.reg.a = 0;
    cpu.reg.b = 0;
    instruction.run(&mut cpu);

    assert!(cpu.is_set(Z));

    // Test half-carry flag
    cpu.reg.a = 62;
    cpu.reg.b = 34;
    instruction.run(&mut cpu);

    assert_eq!(cpu.reg.a, 28);
    assert!(cpu.is_set(H));

    // Test carry flag
    cpu.reg.a = 0;
    cpu.reg.b = 5;
    instruction.run(&mut cpu);

    assert!(cpu.is_set(C));
}

#[test]
fn test_mmu() {
    let mut mmu = MMU::default();

    mmu.write_word(0x0000, 0x0001);
    mmu.write_word(0x0002, 0x0203);

    mmu.write_word(0x8000, 0x0405);
    mmu.write_word(0x8002, 0x0607);

    mmu.write_word(0xC000, 0x0809);
    mmu.write_word(0xC002, 0x0A0B);

    assert_eq!(mmu.read_word(0x0000), 0x0001);
    assert_eq!(mmu.read_word(0x0002), 0x0203);

    assert_eq!(mmu.read_word(0x8000), 0x0405);
    assert_eq!(mmu.read_word(0x8002), 0x0607);

    assert_eq!(mmu.read_word(0xC000), 0x0809);
    assert_eq!(mmu.read_word(0xC002), 0x0A0B);
}