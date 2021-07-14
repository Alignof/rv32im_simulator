use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;

    match inst.opc {
        OP_LUI    => {
            cpu.reg[inst.rd.unwrap() as usize] = (inst.imm.unwrap() << 12) as i32;
        },
        OP_AUIPC  => {
            cpu.pc += inst.imm.unwrap() << 12;
        },
        OP_JAL    => {},
        OP_JALR   => {},
        OP_BEQ    => {},
        OP_BNE    => {},
        OP_BLT    => {},
        OP_BGE    => {},
        OP_BLTU   => {},
        OP_BGEU   => {},
        OP_LB     => {},
        OP_LH     => {},
        OP_LW     => {},
        OP_LBU    => {},
        OP_LHU    => {},
        OP_SB     => {},
        OP_SH     => {},
        OP_SW     => {},
        OP_ADDI   => {
            cpu.reg[inst.rd.unwrap() as usize] += inst.rs1.unwrap() as i32;
        },
        OP_SLTI   => {},
        OP_SLTIU  => {},
        OP_XORI   => {
            cpu.reg[inst.rd.unwrap() as usize] ^= inst.rs1.unwrap() as i32;
        },
        OP_ORI    => {
            cpu.reg[inst.rd.unwrap() as usize] |= inst.rs1.unwrap() as i32;
        },
        OP_ANDI   => {
            cpu.reg[inst.rd.unwrap() as usize] &= inst.rs1.unwrap() as i32;
        },
        OP_SLLI   => {},
        OP_SRLI   => {},
        OP_ADD    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] + cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SUB    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] - cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SLL    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] << cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SLT    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                (cpu.reg[inst.rs1.unwrap() as usize] < cpu.reg[inst.rs2.unwrap() as usize]) as i32;
        },
        OP_SLTU   => {
            cpu.reg[inst.rd.unwrap() as usize] =
                ((cpu.reg[inst.rs1.unwrap() as usize] as u32) < (cpu.reg[inst.rs2.unwrap() as usize] as u32)) as i32;
        },
        OP_XOR    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] ^ cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SRL    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] >> cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_SRA    => {},
        OP_OR     => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] | cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_AND    => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] & cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_FENCE  => {},
        OP_ECALL  => {},
        OP_EBREAK => {},
        _         => panic!("not a full instruction"),
    }

    cpu.pc += 4;
}