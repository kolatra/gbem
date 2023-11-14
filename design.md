# gameboy

## cpu - Sharp LR35902
A custom microprocessor, which is a hybrid between the Intel 8080 and the Zilog Z80. 

I/O is memory mapped because of the limited needs in the Gameboy

Hardware interrupts:
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
