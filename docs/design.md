# gameboy

## DMG

goal:
emulate a gameboy system.
start with the cpu and memory.

## cpu - Sharp LR35902
A custom 8-bit microprocessor, which is a hybrid between the Intel 8080 and the Zilog Z80.

Many instructions were modified/removed for this system, with some newly added ones as well.

The Gameboy's CPU has 6 registers of 16-bit addressable data. Some are specialized while others are used for general 16 bit operations.
- AF ~ The accumulator and Flags registers
- BC
- DE
- HL
- SP ~ The stack pointer
- PC ~ The program counter

Flags:
- bits 0-3 are unused
- 4 ~ Carry flag (C): Set if a carry occured from the last calculation, or if A is the smaller value when doing a compare
- 5 ~ Half-carry flag (H): Set if a carry occured from the lower nibble (4 bits) in the last math instruction
- 6 ~ Subtract flag (N): Set if a subtraction was performed in the last math instruction
- 7 ~ Zero flag (Z): Set if the result of a math operation is zero, or two values match when using `CP`

The clock frequency of this CPU is 4.194304 MHz
Machine cycles take 4 cpu cycles

I/O is memory mapped because of the limited needs in the Gameboy

The gameboy has a 16-bit address bus, 0x0000 - 0xFFFF
0000 - 3fff ~ 16Kb rom bank 00                 // From cartridge, usually a fixed bank
4000 - 7fff ~ 16Kb rom bank 01~NN              // Switchable bank via memory mapper
8000 - 9fff ~ 8kb video RAM
a000 - bfff ~ 8kb external ram                 // From cartridge, switchable bank if any
c000 - dfff ~ 8kb work RAM
e000 - fdff ~ Mirror of c000 - ddff (ECHO RAM)
fe00 - fe9f ~ Object attribute memory (OAM)
fea0 - feff ~ Not usable area                  // free area lets gooooo
ff00 - ff7f ~ I/O Registers
ff80 - fffe ~ High RAM
ffff - ffff ~ Interrupt enable register

core interrupts:
 - Devices can ask the cpu to interrupt, which would invoke an interrupt handler (a function pointer stored in the heap probably)
 - the handlers seem to be part of the game ROM, so we map them into memory 
 - the CPU can halt for particular devices and continue for others that it's not interested in
 - when one happens, the CPU disables further interrupts, the program counter is pushed to the stack, then the interrupt handler is called. when this is finished, the return address is popped back onto the program counter, and execution continues
 - !! some games may use nested interrupts. while an interrupt is being handled, they may be enabled again and invoked before the first finishes
 - interrupts have priorities because of this. with the lowest bit (v-blank) having the highest priority.
 - when an interrupt is requested, the cpu checks IE for the corresponding bit to decide whether to oblige to the request
 - FFFF – IE – Interrupt Enable (R/W)

  Bit 0: V-Blank  Interrupt Enable  (INT 40h)  (1=Enable)
  Bit 1: LCD STAT Interrupt Enable  (INT 48h)  (1=Enable)
  Bit 2: Timer    Interrupt Enable  (INT 50h)  (1=Enable)
  Bit 3: Serial   Interrupt Enable  (INT 58h)  (1=Enable)
  Bit 4: Joypad   Interrupt Enable  (INT 60h)  (1=Enable)

 - FF0F – IF – Interrupt Flag (R/W)

  Bit 0: V-Blank  Interrupt Request (INT 40h)  (1=Request)
  Bit 1: LCD STAT Interrupt Request (INT 48h)  (1=Request)
  Bit 2: Timer    Interrupt Request (INT 50h)  (1=Request)
  Bit 3: Serial   Interrupt Request (INT 58h)  (1=Request)
  Bit 4: Joypad   Interrupt Request (INT 60h)  (1=Request)

The main specs are:
 - CPU: 8-bit (Similar to the Z80 processor.)
 - Main RAM: 8K Byte
 - Video RAM: 8K Byte
 - Screen Size 2.6"
 - Resolution: 160x144 (20x18 tiles)
 - Max # of sprites: 40
 - Max # sprites/line: 10
 - Max sprite size: 8x16
 - Min sprite size: 8x8
 - Clock Speed: 4.194304 MHz 
   (4.295454 SGB, 4.194/8.388MHz GBC)
 - Horiz Sync: 9198 KHz (9420 KHz for SGB)
 - Vert Sync: 59.73 Hz (61.17 Hz for SGB)
 - Sound: 4 channels with stereo sound
 - Power: DC6V 0.7W (DC3V 0.7W for GB Pocket)
