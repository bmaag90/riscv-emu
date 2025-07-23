# RISC-V Instruction Set Implementation Progress

## Arithmetic Instructions
- [ ] `add rd, rs1, rs2` - Add (rd = rs1 + rs2)
- [x] `addi rd, rs1, imm` - Add Immediate (rd = rs1 + imm)
- [ ] `neg rd, rs2` - Negate (rd = -rs2)
- [ ] `sub rd, rs1, rs2` - Subtract (rd = rs1 - rs2)
- [ ] `mul rd, rs1, rs2` - Multiply (rd = (rs1 * rs2)[31:0])
- [ ] `mulh rd, rs1, rs2` - Multiply High (rd = (rs1 * rs2)[63:32])
- [ ] `mulhu rd, rs1, rs2` - Multiply High Unsigned
- [ ] `mulhsu rd, rs1, rs2` - Multiply High Signed Unsigned
- [ ] `div rd, rs1, rs2` - Divide (rd = rs1 / rs2)
- [ ] `rem rd, rs1, rs2` - Remainder (rd = rs1 % rs2)

## Bitwise Logic
- [ ] `and rd, rs1, rs2` - AND (rd = rs1 & rs2)
- [x] `andi rd, rs1, imm` - AND Immediate
- [ ] `not rd, rs1` - NOT (rd = ~rs1)
- [ ] `or rd, rs1, rs2` - OR (rd = rs1 | rs2)
- [x] `ori rd, rs1, imm` - OR Immediate
- [ ] `xor rd, rs1, rs2` - XOR (rd = rs1 ^ rs2)
- [x] `xori rd, rs1, imm` - XOR Immediate

## Shifts
- [ ] `sll rd, rs1, rs2` - Shift Left Logical
- [x] `slli rd, rs1, imm` - Shift Left Logical Immediate
- [ ] `srl rd, rs1, rs2` - Shift Right Logical
- [x] `srli rd, rs1, imm` - Shift Right Logical Immediate
- [ ] `sra rd, rs1, rs2` - Shift Right Arithmetic
- [x] `srai rd, rs1, imm` - Shift Right Arithmetic Immediate

## Load/Store
- [ ] `lw rd, imm(rs1)` - Load Word
- [ ] `lh rd, imm(rs1)` - Load Half
- [ ] `lhu rd, imm(rs1)` - Load Half Unsigned
- [ ] `lb rd, imm(rs1)` - Load Byte
- [ ] `lbu rd, imm(rs1)` - Load Byte Unsigned
- [ ] `sw rs2, imm(rs1)` - Store Word
- [ ] `sh rs2, imm(rs1)` - Store Half
- [ ] `sb rs2, imm(rs1)` - Store Byte
- [x] `lui rd, imm20` - Load upper immediate

## Branch Instructions
- [ ] `beq rs1, rs2, imm` - Branch Equal
- [ ] `bne rs1, rs2, imm` - Branch Not Equal
- [ ] `blt rs1, rs2, imm` - Branch Less Than
- [ ] `bge rs1, rs2, imm` - Branch Greater or Equal
- [ ] `bltu rs1, rs2, imm` - Branch Less Than Unsigned
- [ ] `bgeu rs1, rs2, imm` - Branch Greater or Equal Unsigned

## Set Instructions
- [ ] `slt rd, rs1, rs2` - Set Less Than
- [x] `slti rd, rs1, imm` - Set Less Than Immediate
- [ ] `sltu rd, rs1, rs2` - Set Less Than Unsigned
- [x] `sltiu rd, rs1, imm` - Set Less Than Immediate Unsigned

## Jump Instructions
- [ ] `jal rd, imm` - Jump and Link
- [ ] `jalr rd, rs1, imm` - Jump and Link Register

## System Instructions
- [ ] `ecall` - Environment Call
- [ ] `ebreak` - Environment Break
- [ ] `fence` - Memory and I/O Fence

## Pseudo Instructions
- [ ] `li rd, imm` - Load Immediate
- [ ] `mv rd, rs1` - Copy Register
- [ ] `nop` - No Operation
- [ ] `j imm` - Jump
- [ ] `ret` - Return from Function
- [ ] `call symbol` - Call Function

## Misc
- [ ] `auipc rd, imm`, AUIPC - add upper immediate to pc
