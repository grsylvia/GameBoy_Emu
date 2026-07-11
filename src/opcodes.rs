use super::{GameBoy, Flag};

type OpFn = fn(&mut GameBoy);

pub(super) static OPCODE_TABLE: [OpFn; 256] = [
    // 0x0_
    GameBoy::op_nop,            // 0x00 NOP
    GameBoy::op_ld_bc_d16,      // 0x01 LD BC, d16
    GameBoy::op_ld_mem_bc_a,    // 0x02 LD (BC), A
    GameBoy::op_inc_bc,         // 0x03 INC BC
    GameBoy::op_inc_b,          // 0x04 INC B
    GameBoy::op_dec_b,          // 0x05 DEC B
    GameBoy::op_ld_b_d8,        // 0x06 LD B, d8
    GameBoy::op_rlca,           // 0x07 RLCA
    GameBoy::op_ld_mem_a16_sp,  // 0x08 LD (a16), SP
    GameBoy::op_add_hl_bc,      // 0x09 ADD HL, BC
    GameBoy::op_ld_a_mem_bc,    // 0x0A LD A, (BC)
    GameBoy::op_dec_bc,         // 0x0B DEC BC
    GameBoy::op_inc_c,          // 0x0C INC C
    GameBoy::op_dec_c,          // 0x0D DEC C
    GameBoy::op_ld_c_d8,        // 0x0E LD C, d8
    GameBoy::op_rrca,           // 0x0F RRCA

    // 0x1_
    GameBoy::op_stop,           // 0x10 STOP 0
    GameBoy::op_ld_de_d16,      // 0x11 LD DE, d16
    GameBoy::op_ld_mem_de_a,    // 0x12 LD (DE), A
    GameBoy::op_inc_de,         // 0x13 INC DE
    GameBoy::op_inc_d,          // 0x14 INC D
    GameBoy::op_dec_d,          // 0x15 DEC D
    GameBoy::op_ld_d_d8,        // 0x16 LD D, d8
    GameBoy::op_rla,            // 0x17 RLA
    GameBoy::op_jr_r8,          // 0x18 JR r8
    GameBoy::op_add_hl_de,      // 0x19 ADD HL, DE
    GameBoy::op_ld_a_mem_de,    // 0x1A LD A, (DE)
    GameBoy::op_dec_de,         // 0x1B DEC DE
    GameBoy::op_inc_e,          // 0x1C INC E
    GameBoy::op_dec_e,          // 0x1D DEC E
    GameBoy::op_ld_e_d8,        // 0x1E LD E, d8
    GameBoy::op_rra,            // 0x1F RRA

    // 0x2_
    GameBoy::op_jr_nz_r8,       // 0x20 JR NZ, r8
    GameBoy::op_ld_hl_d16,      // 0x21 LD HL, d16
    GameBoy::op_ld_mem_hli_a,   // 0x22 LD (HL+), A
    GameBoy::op_inc_hl,         // 0x23 INC HL
    GameBoy::op_inc_h,          // 0x24 INC H
    GameBoy::op_dec_h,          // 0x25 DEC H
    GameBoy::op_ld_h_d8,        // 0x26 LD H, d8
    GameBoy::op_daa,            // 0x27 DAA
    GameBoy::op_jr_z_r8,        // 0x28 JR Z, r8
    GameBoy::op_add_hl_hl,      // 0x29 ADD HL, HL
    GameBoy::op_ld_a_mem_hli,   // 0x2A LD A, (HL+)
    GameBoy::op_dec_hl,         // 0x2B DEC HL
    GameBoy::op_inc_l,          // 0x2C INC L
    GameBoy::op_dec_l,          // 0x2D DEC L
    GameBoy::op_ld_l_d8,        // 0x2E LD L, d8
    GameBoy::op_cpl,            // 0x2F CPL

    // 0x3_
    GameBoy::op_jr_nc_r8,       // 0x30 JR NC, r8
    GameBoy::op_ld_sp_d16,      // 0x31 LD SP, d16
    GameBoy::op_ld_mem_hld_a,   // 0x32 LD (HL-), A
    GameBoy::op_inc_sp,         // 0x33 INC SP
    GameBoy::op_inc_mem_hl,     // 0x34 INC (HL)
    GameBoy::op_dec_mem_hl,     // 0x35 DEC (HL)
    GameBoy::op_ld_mem_hl_d8,   // 0x36 LD (HL), d8
    GameBoy::op_scf,            // 0x37 SCF
    GameBoy::op_jr_c_r8,        // 0x38 JR C, r8
    GameBoy::op_add_hl_sp,      // 0x39 ADD HL, SP
    GameBoy::op_ld_a_mem_hld,   // 0x3A LD A, (HL-)
    GameBoy::op_dec_sp,         // 0x3B DEC SP
    GameBoy::op_inc_a,          // 0x3C INC A
    GameBoy::op_dec_a,          // 0x3D DEC A
    GameBoy::op_ld_a_d8,        // 0x3E LD A, d8
    GameBoy::op_ccf,            // 0x3F CCF

    // 0x4_
    GameBoy::op_ld_b_b,         // 0x40 LD B, B
    GameBoy::op_ld_b_c,         // 0x41 LD B, C
    GameBoy::op_ld_b_d,         // 0x42 LD B, D
    GameBoy::op_ld_b_e,         // 0x43 LD B, E
    GameBoy::op_ld_b_h,         // 0x44 LD B, H
    GameBoy::op_ld_b_l,         // 0x45 LD B, L
    GameBoy::op_ld_b_mem_hl,    // 0x46 LD B, (HL)
    GameBoy::op_ld_b_a,         // 0x47 LD B, A
    GameBoy::op_ld_c_b,         // 0x48 LD C, B
    GameBoy::op_ld_c_c,         // 0x49 LD C, C
    GameBoy::op_ld_c_d,         // 0x4A LD C, D
    GameBoy::op_ld_c_e,         // 0x4B LD C, E
    GameBoy::op_ld_c_h,         // 0x4C LD C, H
    GameBoy::op_ld_c_l,         // 0x4D LD C, L
    GameBoy::op_ld_c_mem_hl,    // 0x4E LD C, (HL)
    GameBoy::op_ld_c_a,         // 0x4F LD C, A

    // 0x5_
    GameBoy::op_ld_d_b,         // 0x50 LD D, B
    GameBoy::op_ld_d_c,         // 0x51 LD D, C
    GameBoy::op_ld_d_d,         // 0x52 LD D, D
    GameBoy::op_ld_d_e,         // 0x53 LD D, E
    GameBoy::op_ld_d_h,         // 0x54 LD D, H
    GameBoy::op_ld_d_l,         // 0x55 LD D, L
    GameBoy::op_ld_d_mem_hl,    // 0x56 LD D, (HL)
    GameBoy::op_ld_d_a,         // 0x57 LD D, A
    GameBoy::op_ld_e_b,         // 0x58 LD E, B
    GameBoy::op_ld_e_c,         // 0x59 LD E, C
    GameBoy::op_ld_e_d,         // 0x5A LD E, D
    GameBoy::op_ld_e_e,         // 0x5B LD E, E
    GameBoy::op_ld_e_h,         // 0x5C LD E, H
    GameBoy::op_ld_e_l,         // 0x5D LD E, L
    GameBoy::op_ld_e_mem_hl,    // 0x5E LD E, (HL)
    GameBoy::op_ld_e_a,         // 0x5F LD E, A

    // 0x6_
    GameBoy::op_ld_h_b,         // 0x60 LD H, B
    GameBoy::op_ld_h_c,         // 0x61 LD H, C
    GameBoy::op_ld_h_d,         // 0x62 LD H, D
    GameBoy::op_ld_h_e,         // 0x63 LD H, E
    GameBoy::op_ld_h_h,         // 0x64 LD H, H
    GameBoy::op_ld_h_l,         // 0x65 LD H, L
    GameBoy::op_ld_h_mem_hl,    // 0x66 LD H, (HL)
    GameBoy::op_ld_h_a,         // 0x67 LD H, A
    GameBoy::op_ld_l_b,         // 0x68 LD L, B
    GameBoy::op_ld_l_c,         // 0x69 LD L, C
    GameBoy::op_ld_l_d,         // 0x6A LD L, D
    GameBoy::op_ld_l_e,         // 0x6B LD L, E
    GameBoy::op_ld_l_h,         // 0x6C LD L, H
    GameBoy::op_ld_l_l,         // 0x6D LD L, L
    GameBoy::op_ld_l_mem_hl,    // 0x6E LD L, (HL)
    GameBoy::op_ld_l_a,         // 0x6F LD L, A

    // 0x7_
    GameBoy::op_ld_mem_hl_b,    // 0x70 LD (HL), B
    GameBoy::op_ld_mem_hl_c,    // 0x71 LD (HL), C
    GameBoy::op_ld_mem_hl_d,    // 0x72 LD (HL), D
    GameBoy::op_ld_mem_hl_e,    // 0x73 LD (HL), E
    GameBoy::op_ld_mem_hl_h,    // 0x74 LD (HL), H
    GameBoy::op_ld_mem_hl_l,    // 0x75 LD (HL), L
    GameBoy::op_halt,           // 0x76 HALT
    GameBoy::op_ld_mem_hl_a,    // 0x77 LD (HL), A
    GameBoy::op_ld_a_b,         // 0x78 LD A, B
    GameBoy::op_ld_a_c,         // 0x79 LD A, C
    GameBoy::op_ld_a_d,         // 0x7A LD A, D
    GameBoy::op_ld_a_e,         // 0x7B LD A, E
    GameBoy::op_ld_a_h,         // 0x7C LD A, H
    GameBoy::op_ld_a_l,         // 0x7D LD A, L
    GameBoy::op_ld_a_mem_hl,    // 0x7E LD A, (HL)
    GameBoy::op_ld_a_a,         // 0x7F LD A, A

    // 0x8_
    GameBoy::op_add_a_b,        // 0x80 ADD A, B
    GameBoy::op_add_a_c,        // 0x81 ADD A, C
    GameBoy::op_add_a_d,        // 0x82 ADD A, D
    GameBoy::op_add_a_e,        // 0x83 ADD A, E
    GameBoy::op_add_a_h,        // 0x84 ADD A, H
    GameBoy::op_add_a_l,        // 0x85 ADD A, L
    GameBoy::op_add_a_mem_hl,   // 0x86 ADD A, (HL)
    GameBoy::op_add_a_a,        // 0x87 ADD A, A
    GameBoy::op_adc_a_b,        // 0x88 ADC A, B
    GameBoy::op_adc_a_c,        // 0x89 ADC A, C
    GameBoy::op_adc_a_d,        // 0x8A ADC A, D
    GameBoy::op_adc_a_e,        // 0x8B ADC A, E
    GameBoy::op_adc_a_h,        // 0x8C ADC A, H
    GameBoy::op_adc_a_l,        // 0x8D ADC A, L
    GameBoy::op_adc_a_mem_hl,   // 0x8E ADC A, (HL)
    GameBoy::op_adc_a_a,        // 0x8F ADC A, A

    // 0x9_
    GameBoy::op_sub_b,          // 0x90 SUB B
    GameBoy::op_sub_c,          // 0x91 SUB C
    GameBoy::op_sub_d,          // 0x92 SUB D
    GameBoy::op_sub_e,          // 0x93 SUB E
    GameBoy::op_sub_h,          // 0x94 SUB H
    GameBoy::op_sub_l,          // 0x95 SUB L
    GameBoy::op_sub_mem_hl,     // 0x96 SUB (HL)
    GameBoy::op_sub_a,          // 0x97 SUB A
    GameBoy::op_sbc_a_b,        // 0x98 SBC A, B
    GameBoy::op_sbc_a_c,        // 0x99 SBC A, C
    GameBoy::op_sbc_a_d,        // 0x9A SBC A, D
    GameBoy::op_sbc_a_e,        // 0x9B SBC A, E
    GameBoy::op_sbc_a_h,        // 0x9C SBC A, H
    GameBoy::op_sbc_a_l,        // 0x9D SBC A, L
    GameBoy::op_sbc_a_mem_hl,   // 0x9E SBC A, (HL)
    GameBoy::op_sbc_a_a,        // 0x9F SBC A, A

    // 0xA_
    GameBoy::op_and_b,          // 0xA0 AND B
    GameBoy::op_and_c,          // 0xA1 AND C
    GameBoy::op_and_d,          // 0xA2 AND D
    GameBoy::op_and_e,          // 0xA3 AND E
    GameBoy::op_and_h,          // 0xA4 AND H
    GameBoy::op_and_l,          // 0xA5 AND L
    GameBoy::op_and_mem_hl,     // 0xA6 AND (HL)
    GameBoy::op_and_a,          // 0xA7 AND A
    GameBoy::op_xor_b,          // 0xA8 XOR B
    GameBoy::op_xor_c,          // 0xA9 XOR C
    GameBoy::op_xor_d,          // 0xAA XOR D
    GameBoy::op_xor_e,          // 0xAB XOR E
    GameBoy::op_xor_h,          // 0xAC XOR H
    GameBoy::op_xor_l,          // 0xAD XOR L
    GameBoy::op_xor_mem_hl,     // 0xAE XOR (HL)
    GameBoy::op_xor_a,          // 0xAF XOR A

    // 0xB_
    GameBoy::op_or_b,           // 0xB0 OR B
    GameBoy::op_or_c,           // 0xB1 OR C
    GameBoy::op_or_d,           // 0xB2 OR D
    GameBoy::op_or_e,           // 0xB3 OR E
    GameBoy::op_or_h,           // 0xB4 OR H
    GameBoy::op_or_l,           // 0xB5 OR L
    GameBoy::op_or_mem_hl,      // 0xB6 OR (HL)
    GameBoy::op_or_a,           // 0xB7 OR A
    GameBoy::op_cp_b,           // 0xB8 CP B
    GameBoy::op_cp_c,           // 0xB9 CP C
    GameBoy::op_cp_d,           // 0xBA CP D
    GameBoy::op_cp_e,           // 0xBB CP E
    GameBoy::op_cp_h,           // 0xBC CP H
    GameBoy::op_cp_l,           // 0xBD CP L
    GameBoy::op_cp_mem_hl,      // 0xBE CP (HL)
    GameBoy::op_cp_a,           // 0xBF CP A

    // 0xC_
    GameBoy::op_ret_nz,         // 0xC0 RET NZ
    GameBoy::op_pop_bc,         // 0xC1 POP BC
    GameBoy::op_jp_nz_a16,      // 0xC2 JP NZ, a16
    GameBoy::op_jp_a16,         // 0xC3 JP a16
    GameBoy::op_call_nz_a16,    // 0xC4 CALL NZ, a16
    GameBoy::op_push_bc,        // 0xC5 PUSH BC
    GameBoy::op_add_a_d8,       // 0xC6 ADD A, d8
    GameBoy::op_rst_00h,        // 0xC7 RST 00H
    GameBoy::op_ret_z,          // 0xC8 RET Z
    GameBoy::op_ret,            // 0xC9 RET
    GameBoy::op_jp_z_a16,       // 0xCA JP Z, a16
    GameBoy::op_prefix_cb,      // 0xCB PREFIX CB
    GameBoy::op_call_z_a16,     // 0xCC CALL Z, a16
    GameBoy::op_call_a16,       // 0xCD CALL a16
    GameBoy::op_adc_a_d8,       // 0xCE ADC A, d8
    GameBoy::op_rst_08h,        // 0xCF RST 08H

    // 0xD_
    GameBoy::op_ret_nc,         // 0xD0 RET NC
    GameBoy::op_pop_de,         // 0xD1 POP DE
    GameBoy::op_jp_nc_a16,      // 0xD2 JP NC, a16
    GameBoy::op_undefined,      // 0xD3 (undefined)
    GameBoy::op_call_nc_a16,    // 0xD4 CALL NC, a16
    GameBoy::op_push_de,        // 0xD5 PUSH DE
    GameBoy::op_sub_d8,         // 0xD6 SUB d8
    GameBoy::op_rst_10h,        // 0xD7 RST 10H
    GameBoy::op_ret_c,          // 0xD8 RET C
    GameBoy::op_reti,           // 0xD9 RETI
    GameBoy::op_jp_c_a16,       // 0xDA JP C, a16
    GameBoy::op_undefined,      // 0xDB (undefined)
    GameBoy::op_call_c_a16,     // 0xDC CALL C, a16
    GameBoy::op_undefined,      // 0xDD (undefined)
    GameBoy::op_sbc_a_d8,       // 0xDE SBC A, d8
    GameBoy::op_rst_18h,        // 0xDF RST 18H

    // 0xE_
    GameBoy::op_ldh_mem_a8_a,   // 0xE0 LDH (a8), A
    GameBoy::op_pop_hl,         // 0xE1 POP HL
    GameBoy::op_ld_mem_c_a,     // 0xE2 LD (C), A
    GameBoy::op_undefined,      // 0xE3 (undefined)
    GameBoy::op_undefined,      // 0xE4 (undefined)
    GameBoy::op_push_hl,        // 0xE5 PUSH HL
    GameBoy::op_and_d8,         // 0xE6 AND d8
    GameBoy::op_rst_20h,        // 0xE7 RST 20H
    GameBoy::op_add_sp_r8,      // 0xE8 ADD SP, r8
    GameBoy::op_jp_hl,          // 0xE9 JP (HL)
    GameBoy::op_ld_mem_a16_a,   // 0xEA LD (a16), A
    GameBoy::op_undefined,      // 0xEB (undefined)
    GameBoy::op_undefined,      // 0xEC (undefined)
    GameBoy::op_undefined,      // 0xED (undefined)
    GameBoy::op_xor_d8,         // 0xEE XOR d8
    GameBoy::op_rst_28h,        // 0xEF RST 28H

    // 0xF_
    GameBoy::op_ldh_a_mem_a8,   // 0xF0 LDH A, (a8)
    GameBoy::op_pop_af,         // 0xF1 POP AF
    GameBoy::op_ld_a_mem_c,     // 0xF2 LD A, (C)
    GameBoy::op_di,             // 0xF3 DI
    GameBoy::op_undefined,      // 0xF4 (undefined)
    GameBoy::op_push_af,        // 0xF5 PUSH AF
    GameBoy::op_or_d8,          // 0xF6 OR d8
    GameBoy::op_rst_30h,        // 0xF7 RST 30H
    GameBoy::op_ld_hl_sp_r8,    // 0xF8 LD HL, SP+r8
    GameBoy::op_ld_sp_hl,       // 0xF9 LD SP, HL
    GameBoy::op_ld_a_mem_a16,   // 0xFA LD A, (a16)
    GameBoy::op_ei,             // 0xFB EI
    GameBoy::op_undefined,      // 0xFC (undefined)
    GameBoy::op_undefined,      // 0xFD (undefined)
    GameBoy::op_cp_d8,          // 0xFE CP d8
    GameBoy::op_rst_38h,        // 0xFF RST 38H
];

// Human-readable mnemonic for each opcode, indexed by opcode byte
pub(super) static MNEMONIC_TABLE: [&str; 256] = [
    // 0x0_
    "NOP",              // 0x00
    "LD BC, d16",       // 0x01
    "LD (BC), A",       // 0x02
    "INC BC",           // 0x03
    "INC B",            // 0x04
    "DEC B",            // 0x05
    "LD B, d8",         // 0x06
    "RLCA",             // 0x07
    "LD (a16), SP",     // 0x08
    "ADD HL, BC",       // 0x09
    "LD A, (BC)",       // 0x0A
    "DEC BC",           // 0x0B
    "INC C",            // 0x0C
    "DEC C",            // 0x0D
    "LD C, d8",         // 0x0E
    "RRCA",             // 0x0F

    // 0x1_
    "STOP 0",           // 0x10
    "LD DE, d16",       // 0x11
    "LD (DE), A",       // 0x12
    "INC DE",           // 0x13
    "INC D",            // 0x14
    "DEC D",            // 0x15
    "LD D, d8",         // 0x16
    "RLA",              // 0x17
    "JR r8",            // 0x18
    "ADD HL, DE",       // 0x19
    "LD A, (DE)",       // 0x1A
    "DEC DE",           // 0x1B
    "INC E",            // 0x1C
    "DEC E",            // 0x1D
    "LD E, d8",         // 0x1E
    "RRA",              // 0x1F

    // 0x2_
    "JR NZ, r8",        // 0x20
    "LD HL, d16",       // 0x21
    "LD (HL+), A",      // 0x22
    "INC HL",           // 0x23
    "INC H",            // 0x24
    "DEC H",            // 0x25
    "LD H, d8",         // 0x26
    "DAA",              // 0x27
    "JR Z, r8",         // 0x28
    "ADD HL, HL",       // 0x29
    "LD A, (HL+)",      // 0x2A
    "DEC HL",           // 0x2B
    "INC L",            // 0x2C
    "DEC L",            // 0x2D
    "LD L, d8",         // 0x2E
    "CPL",              // 0x2F

    // 0x3_
    "JR NC, r8",        // 0x30
    "LD SP, d16",       // 0x31
    "LD (HL-), A",      // 0x32
    "INC SP",           // 0x33
    "INC (HL)",         // 0x34
    "DEC (HL)",         // 0x35
    "LD (HL), d8",      // 0x36
    "SCF",              // 0x37
    "JR C, r8",         // 0x38
    "ADD HL, SP",       // 0x39
    "LD A, (HL-)",      // 0x3A
    "DEC SP",           // 0x3B
    "INC A",            // 0x3C
    "DEC A",            // 0x3D
    "LD A, d8",         // 0x3E
    "CCF",              // 0x3F

    // 0x4_
    "LD B, B",          // 0x40
    "LD B, C",          // 0x41
    "LD B, D",          // 0x42
    "LD B, E",          // 0x43
    "LD B, H",          // 0x44
    "LD B, L",          // 0x45
    "LD B, (HL)",       // 0x46
    "LD B, A",          // 0x47
    "LD C, B",          // 0x48
    "LD C, C",          // 0x49
    "LD C, D",          // 0x4A
    "LD C, E",          // 0x4B
    "LD C, H",          // 0x4C
    "LD C, L",          // 0x4D
    "LD C, (HL)",       // 0x4E
    "LD C, A",          // 0x4F

    // 0x5_
    "LD D, B",          // 0x50
    "LD D, C",          // 0x51
    "LD D, D",          // 0x52
    "LD D, E",          // 0x53
    "LD D, H",          // 0x54
    "LD D, L",          // 0x55
    "LD D, (HL)",       // 0x56
    "LD D, A",          // 0x57
    "LD E, B",          // 0x58
    "LD E, C",          // 0x59
    "LD E, D",          // 0x5A
    "LD E, E",          // 0x5B
    "LD E, H",          // 0x5C
    "LD E, L",          // 0x5D
    "LD E, (HL)",       // 0x5E
    "LD E, A",          // 0x5F

    // 0x6_
    "LD H, B",          // 0x60
    "LD H, C",          // 0x61
    "LD H, D",          // 0x62
    "LD H, E",          // 0x63
    "LD H, H",          // 0x64
    "LD H, L",          // 0x65
    "LD H, (HL)",       // 0x66
    "LD H, A",          // 0x67
    "LD L, B",          // 0x68
    "LD L, C",          // 0x69
    "LD L, D",          // 0x6A
    "LD L, E",          // 0x6B
    "LD L, H",          // 0x6C
    "LD L, L",          // 0x6D
    "LD L, (HL)",       // 0x6E
    "LD L, A",          // 0x6F

    // 0x7_
    "LD (HL), B",       // 0x70
    "LD (HL), C",       // 0x71
    "LD (HL), D",       // 0x72
    "LD (HL), E",       // 0x73
    "LD (HL), H",       // 0x74
    "LD (HL), L",       // 0x75
    "HALT",             // 0x76
    "LD (HL), A",       // 0x77
    "LD A, B",          // 0x78
    "LD A, C",          // 0x79
    "LD A, D",          // 0x7A
    "LD A, E",          // 0x7B
    "LD A, H",          // 0x7C
    "LD A, L",          // 0x7D
    "LD A, (HL)",       // 0x7E
    "LD A, A",          // 0x7F

    // 0x8_
    "ADD A, B",         // 0x80
    "ADD A, C",         // 0x81
    "ADD A, D",         // 0x82
    "ADD A, E",         // 0x83
    "ADD A, H",         // 0x84
    "ADD A, L",         // 0x85
    "ADD A, (HL)",      // 0x86
    "ADD A, A",         // 0x87
    "ADC A, B",         // 0x88
    "ADC A, C",         // 0x89
    "ADC A, D",         // 0x8A
    "ADC A, E",         // 0x8B
    "ADC A, H",         // 0x8C
    "ADC A, L",         // 0x8D
    "ADC A, (HL)",      // 0x8E
    "ADC A, A",         // 0x8F

    // 0x9_
    "SUB B",            // 0x90
    "SUB C",            // 0x91
    "SUB D",            // 0x92
    "SUB E",            // 0x93
    "SUB H",            // 0x94
    "SUB L",            // 0x95
    "SUB (HL)",         // 0x96
    "SUB A",            // 0x97
    "SBC A, B",         // 0x98
    "SBC A, C",         // 0x99
    "SBC A, D",         // 0x9A
    "SBC A, E",         // 0x9B
    "SBC A, H",         // 0x9C
    "SBC A, L",         // 0x9D
    "SBC A, (HL)",      // 0x9E
    "SBC A, A",         // 0x9F

    // 0xA_
    "AND B",            // 0xA0
    "AND C",            // 0xA1
    "AND D",            // 0xA2
    "AND E",            // 0xA3
    "AND H",            // 0xA4
    "AND L",            // 0xA5
    "AND (HL)",         // 0xA6
    "AND A",            // 0xA7
    "XOR B",            // 0xA8
    "XOR C",            // 0xA9
    "XOR D",            // 0xAA
    "XOR E",            // 0xAB
    "XOR H",            // 0xAC
    "XOR L",            // 0xAD
    "XOR (HL)",         // 0xAE
    "XOR A",            // 0xAF

    // 0xB_
    "OR B",             // 0xB0
    "OR C",             // 0xB1
    "OR D",             // 0xB2
    "OR E",             // 0xB3
    "OR H",             // 0xB4
    "OR L",             // 0xB5
    "OR (HL)",          // 0xB6
    "OR A",             // 0xB7
    "CP B",             // 0xB8
    "CP C",             // 0xB9
    "CP D",             // 0xBA
    "CP E",             // 0xBB
    "CP H",             // 0xBC
    "CP L",             // 0xBD
    "CP (HL)",          // 0xBE
    "CP A",             // 0xBF

    // 0xC_
    "RET NZ",           // 0xC0
    "POP BC",           // 0xC1
    "JP NZ, a16",       // 0xC2
    "JP a16",           // 0xC3
    "CALL NZ, a16",     // 0xC4
    "PUSH BC",          // 0xC5
    "ADD A, d8",        // 0xC6
    "RST 00H",          // 0xC7
    "RET Z",            // 0xC8
    "RET",              // 0xC9
    "JP Z, a16",        // 0xCA
    "PREFIX CB",        // 0xCB
    "CALL Z, a16",      // 0xCC
    "CALL a16",         // 0xCD
    "ADC A, d8",        // 0xCE
    "RST 08H",          // 0xCF

    // 0xD_
    "RET NC",           // 0xD0
    "POP DE",           // 0xD1
    "JP NC, a16",       // 0xD2
    "(undefined)",      // 0xD3
    "CALL NC, a16",     // 0xD4
    "PUSH DE",          // 0xD5
    "SUB d8",           // 0xD6
    "RST 10H",          // 0xD7
    "RET C",            // 0xD8
    "RETI",             // 0xD9
    "JP C, a16",        // 0xDA
    "(undefined)",      // 0xDB
    "CALL C, a16",      // 0xDC
    "(undefined)",      // 0xDD
    "SBC A, d8",        // 0xDE
    "RST 18H",          // 0xDF

    // 0xE_
    "LDH (a8), A",      // 0xE0
    "POP HL",           // 0xE1
    "LD (C), A",        // 0xE2
    "(undefined)",      // 0xE3
    "(undefined)",      // 0xE4
    "PUSH HL",          // 0xE5
    "AND d8",           // 0xE6
    "RST 20H",          // 0xE7
    "ADD SP, r8",       // 0xE8
    "JP (HL)",          // 0xE9
    "LD (a16), A",      // 0xEA
    "(undefined)",      // 0xEB
    "(undefined)",      // 0xEC
    "(undefined)",      // 0xED
    "XOR d8",           // 0xEE
    "RST 28H",          // 0xEF

    // 0xF_
    "LDH A, (a8)",      // 0xF0
    "POP AF",           // 0xF1
    "LD A, (C)",        // 0xF2
    "DI",               // 0xF3
    "(undefined)",      // 0xF4
    "PUSH AF",          // 0xF5
    "OR d8",            // 0xF6
    "RST 30H",          // 0xF7
    "LD HL, SP+r8",     // 0xF8
    "LD SP, HL",        // 0xF9
    "LD A, (a16)",      // 0xFA
    "EI",               // 0xFB
    "(undefined)",      // 0xFC
    "(undefined)",      // 0xFD
    "CP d8",            // 0xFE
    "RST 38H",          // 0xFF
];

impl GameBoy {
    fn op_undefined(&mut self) {}

    // 0x00
    // NOP
    // advances the program counter by 1 address
    fn op_nop(&mut self) {}

    // 0x01
    // LD BC, d16
    // loads two operands immediately after opcode into registers b and c
    fn op_ld_bc_d16(&mut self) {
        // stores values in memory as little endian (low byte first)
        // fetch low byte and increment program counter
        self.reg.c = self.fetch_byte();
        // fetch high byte and increment program counter
        self.reg.b = self.fetch_byte();
    }

    // 0x02
    // LD (BC), A
    // store contents of register A in memory location addressed by register pair BC
    fn op_ld_mem_bc_a(&mut self) {
        let high_byte: u16 = (self.reg.b as u16) << 8;
        let low_byte: u16 = self.reg.c as u16;

        let address: u16 = high_byte | low_byte;

        self.bus.write(address, self.reg.a);
    }

    // 0x03
    // INC BC
    // increment the contents of register pair BC
    fn op_inc_bc(&mut self) {
        let high_byte: u16 = (self.reg.b as u16) << 8;
        let low_byte: u16 = self.reg.c as u16;

        let bc_increm: u16 = (high_byte | low_byte).wrapping_add(0x0001);

        self.reg.b = (bc_increm >> 8) as u8;
        self.reg.c = (bc_increm & 0x00FF) as u8; 
    }

    // 0x04
    // INC B
    // increment register B by one
    fn op_inc_b(&mut self) {
        let half_carry = (self.reg.b & 0x0F) == 0x0F;

        // increment register b by 1
        self.reg.b = self.reg.b.wrapping_add(0x01);

        // if increment wraps register b back to 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.b == 0x00);
    
        // reset n (subtraction flag)
        self.reg.set_flag(Flag::N, false);

        // if value in register b overflows to high nibble, set H flag to 1
        // ex. 0x0F -> 0x10
        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x05
    // DEC B
    // decrement register B by one
    fn op_dec_b(&mut self) {
        // borrow from bit 4 occurs when the low nibble is 0
        let half_carry = (self.reg.b & 0x0F) == 0x00;

        // decrement register b by 1
        self.reg.b = self.reg.b.wrapping_sub(0x01);

        // if decrement results in 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.b == 0x00);

        // set n (subtraction flag)
        self.reg.set_flag(Flag::N, true);

        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x06
    // LD B, d8
    // load byte immediately after opcode into register B
    fn op_ld_b_d8(&mut self) {
        self.reg.b = self.fetch_byte();
    }

    // 0x07
    // RLCA (Rotate Left Circular, A)
    // rotate register to the left by 1 bit, reset Z, N, H, and set / reset C if there is a carry
    fn op_rlca(&mut self) {

        // reset flags Z, N, H
        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);

        // check if 1 bit is set in bit 7 of register a
        let carry: bool = (self.reg.a & 0b1000_0000) != 0;

        // set carry flag to old bit 7 (must clear it when bit 7 is 0)
        self.reg.set_flag(Flag::C, carry);

        // rotate register a 1 bit to the left
        self.reg.a = self.reg.a.rotate_left(1);
    }
    

    // 0x08
    // LD (a16), SP
    // store lower byte of stack pointer into address specified by the 16-bit operand
    fn op_ld_mem_a16_sp(&mut self) {
        // pull lower and higher bytes of stack pointer
        let lower_sp: u8 = (self.reg.sp & 0xFF) as u8;
        let high_sp: u8 = (self.reg.sp >> 8) as u8;

        let lower_immediate: u16 = self.fetch_byte() as u16;
        let high_immediate: u16 = self.fetch_byte() as u16;
        let address: u16 = (high_immediate << 8) | lower_immediate;

        // store lower byte of sp in immediate 16 bit operand
        self.bus.write(address, lower_sp);

        // store high byte of sp in a16 address + 1
        self.bus.write(address.wrapping_add(0x0001), high_sp);
    }

    // 0x09
    // ADD HL, BC
    // add contents of BC to HL, and store result in HL
    fn op_add_hl_bc(&mut self) {
        let bc: u16 = ((self.reg.b as u16) << 8) | (self.reg.c as u16);
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        // half carry set when the lower 12 bits carry into bit 12
        let half_carry: bool = (hl & 0x0FFF) + (bc & 0x0FFF) > 0x0FFF;
        // carry set when the full 16-bit addition overflows
        let carry: bool = (hl as u32) + (bc as u32) > 0xFFFF;
        let sum: u16 = bc.wrapping_add(hl);

        self.reg.h = (sum >> 8) as u8;
        self.reg.l = (sum & 0x00FF) as u8;

        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, half_carry);
        self.reg.set_flag(Flag::C, carry);

    }

    // 0x0A
    // LD A, (BC)
    // load u8 value from memory addressed by BC into register A
    fn op_ld_a_mem_bc(&mut self) {
        let bc: u16 = ((self.reg.b as u16) << 8) | (self.reg.c as u16);

        self.reg.a = self.bus.read(bc);
    }

    // 0x0B
    // DEC BC
    // decrement the contents of register pair BC
    fn op_dec_bc(&mut self) {
        let high_byte: u16 = (self.reg.b as u16) << 8;
        let low_byte: u16 = self.reg.c as u16;

        let bc_decrem: u16 = (high_byte | low_byte).wrapping_sub(0x0001);

        self.reg.b = (bc_decrem >> 8) as u8;
        self.reg.c = (bc_decrem & 0x00FF) as u8; 
    }

    // 0x0C
    // INC C
    // increments the contents of register c
    fn op_inc_c(&mut self) {
        let half_carry = (self.reg.c & 0x0F) == 0x0F;

        // increment register c by 1
        self.reg.c = self.reg.c.wrapping_add(0x01);

        // if increment wraps register c back to 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.c == 0x00);
    
        // reset n (subtraction flag)
        self.reg.set_flag(Flag::N, false);

        // if value in register c overflows to high nibble, set H flag to 1
        // ex. 0x0F -> 0x10
        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x0D
    // DEC C
    // decrements the contents of register c
    fn op_dec_c(&mut self) {
        // borrow from bit 4 occurs when the low nibble is 0
        let half_carry = (self.reg.c & 0x0F) == 0x00;

        // decrement register c by 1
        self.reg.c = self.reg.c.wrapping_sub(0x01);

        // if decrement results in 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.c == 0x00);

        // set n (subtraction flag)
        self.reg.set_flag(Flag::N, true);

        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x0E
    // LD C, d8
    // load immediate 8-bit operand into register C
    fn op_ld_c_d8(&mut self) {
        self.reg.c = self.fetch_byte();
    }

    // 0x0F
    // RRCA
    fn op_rrca(&mut self) {

        // reset flags Z, N, and H
        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);

        // set carry to value of first bit of register A
        let carry: bool = (self.reg.a & 0b0000_0001) != 0;
        self.reg.set_flag(Flag::C, carry);

        // rotate register a 1 bit to the right
        self.reg.a = self.reg.a.rotate_right(1);

    }

    // 0x10

    // STOP 0
    // enters standby until joypad input
    fn op_stop(&mut self) {
        // includes extra 0x00 padding byte
        self.fetch_byte();
        self.halt = true;
    }

    // 0x11
    // LD DE, d16
    fn op_ld_de_d16(&mut self) {
        self.reg.e = self.fetch_byte();
        self.reg.d = self.fetch_byte();
    }

    // 0x12
    // LD (DE), A
    fn op_ld_mem_de_a(&mut self) {
        let high_byte: u16 = (self.reg.d as u16) << 8;
        let low_byte: u16 = self.reg.e as u16;

        let address: u16 = high_byte | low_byte;

        self.bus.write(address, self.reg.a);
    }

    // 0x13
    // INC DE
    fn op_inc_de(&mut self) {
        let high_byte: u16 = (self.reg.d as u16) << 8;
        let low_byte: u16 = self.reg.e as u16;

        let de_increm: u16 = (high_byte | low_byte).wrapping_add(0x0001);

        self.reg.d = (de_increm >> 8) as u8;
        self.reg.e = (de_increm & 0x00FF) as u8; 
    }

    // 0x14
    // INC D
    // increments the contents of register c
    fn op_inc_d(&mut self) {
        let half_carry = (self.reg.d & 0x0F) == 0x0F;

        // increment register d by 1
        self.reg.d = self.reg.d.wrapping_add(0x01);

        // if increment wraps register d back to 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.d == 0x00);

        // reset n (subtraction flag)
        self.reg.set_flag(Flag::N, false);

        // if value in register d overflows to high nibble, set H flag to 1
        // ex. 0x0F -> 0x10
        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x15
    // DEC D
    // decrements the contents of register c
    fn op_dec_d(&mut self) {
        // borrow from bit 4 occurs when the low nibble is 0
        let half_carry = (self.reg.d & 0x0F) == 0x00;

        // decrement register d by 1
        self.reg.d = self.reg.d.wrapping_sub(0x01);

        // if decrement results in 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.d == 0x00);

        // set n (subtraction flag)
        self.reg.set_flag(Flag::N, true);

        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x16
    // LD D, d8
    fn op_ld_d_d8(&mut self) {
        self.reg.d = self.fetch_byte();
    }

    // 0x17
    // RLA
    fn op_rla(&mut self) {}

    // 0x18
    // JR r8
    fn op_jr_r8(&mut self) {}

    // 0x19
    // ADD HL, DE
    // add contents of DE to HL, and store result in HL
    fn op_add_hl_de(&mut self) {
        let de: u16 = ((self.reg.d as u16) << 8) | (self.reg.e as u16);
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        // half carry set when the lower 12 bits carry into bit 12
        let half_carry: bool = (hl & 0x0FFF) + (de & 0x0FFF) > 0x0FFF;
        // carry set when the full 16-bit addition overflows
        let carry: bool = (hl as u32) + (de as u32) > 0xFFFF;
        let sum: u16 = de.wrapping_add(hl);

        self.reg.h = (sum >> 8) as u8;
        self.reg.l = (sum & 0x00FF) as u8;

        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, half_carry);
        self.reg.set_flag(Flag::C, carry);
    }

    // 0x1A
    // LD A, (DE)
    // load u8 value from memory addressed by DE into register A
    fn op_ld_a_mem_de(&mut self) {
        let de: u16 = ((self.reg.d as u16) << 8) | (self.reg.e as u16);

        self.reg.a = self.bus.read(de);
    }

    // 0x1B
    // DEC DE
    // decrement the contents of register pair DE
    fn op_dec_de(&mut self) {
        let high_byte: u16 = (self.reg.d as u16) << 8;
        let low_byte: u16 = self.reg.e as u16;

        let de_decrem: u16 = (high_byte | low_byte).wrapping_sub(0x0001);

        self.reg.d = (de_decrem >> 8) as u8;
        self.reg.e = (de_decrem & 0x00FF) as u8;
    }

    // 0x1C
    // INC E
    fn op_inc_e(&mut self) {
        let half_carry = (self.reg.e & 0x0F) == 0x0F;

        // increment register e by 1
        self.reg.e = self.reg.e.wrapping_add(0x01);

        // if increment wraps register e back to 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.e == 0x00);

        // reset n (subtraction flag)
        self.reg.set_flag(Flag::N, false);

        // if value in register e overflows to high nibble, set H flag to 1
        // ex. 0x0F -> 0x10
        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x1D
    // DEC E
    fn op_dec_e(&mut self) {
        // borrow from bit 4 occurs when the low nibble is 0
        let half_carry = (self.reg.e & 0x0F) == 0x00;

        // decrement register e by 1
        self.reg.e = self.reg.e.wrapping_sub(0x01);

        // if decrement results in 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.e == 0x00);

        // set n (subtraction flag)
        self.reg.set_flag(Flag::N, true);

        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x1E
    // LD E, d8
    fn op_ld_e_d8(&mut self) {
        self.reg.e = self.fetch_byte();
    }

    // 0x1F
    // RRA
    fn op_rra(&mut self) {}

    // 0x20
    // JR NZ, r8
    fn op_jr_nz_r8(&mut self) {}

    // 0x21
    // LD HL, d16
    fn op_ld_hl_d16(&mut self) {
        self.reg.l = self.fetch_byte();
        self.reg.h = self.fetch_byte();
    }

    // 0x22
    // LD (HL+), A
    // store contents of register A at memory addressed by HL, then increment HL
    fn op_ld_mem_hli_a(&mut self) {
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        self.bus.write(hl, self.reg.a);

        // increment HL and store back into the register pair
        let hl_increm: u16 = hl.wrapping_add(0x0001);
        self.reg.h = (hl_increm >> 8) as u8;
        self.reg.l = (hl_increm & 0x00FF) as u8;
    }

    // 0x23
    // INC HL
    // increment the contents of register pair HL
    fn op_inc_hl(&mut self) {
        let high_byte: u16 = (self.reg.h as u16) << 8;
        let low_byte: u16 = self.reg.l as u16;

        let hl_increm: u16 = (high_byte | low_byte).wrapping_add(0x0001);

        self.reg.h = (hl_increm >> 8) as u8;
        self.reg.l = (hl_increm & 0x00FF) as u8;
    }

    // 0x24
    // INC H
    fn op_inc_h(&mut self) {
        let half_carry = (self.reg.h & 0x0F) == 0x0F;

        // increment register h by 1
        self.reg.h = self.reg.h.wrapping_add(0x01);

        // if increment wraps register h back to 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.h == 0x00);

        // reset n (subtraction flag)
        self.reg.set_flag(Flag::N, false);

        // if value in register h overflows to high nibble, set H flag to 1
        // ex. 0x0F -> 0x10
        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x25
    // DEC H
    fn op_dec_h(&mut self) {
        // borrow from bit 4 occurs when the low nibble is 0
        let half_carry = (self.reg.h & 0x0F) == 0x00;

        // decrement register h by 1
        self.reg.h = self.reg.h.wrapping_sub(0x01);

        // if decrement results in 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.h == 0x00);

        // set n (subtraction flag)
        self.reg.set_flag(Flag::N, true);

        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x26
    // LD H, d8
    fn op_ld_h_d8(&mut self) {
        self.reg.h = self.fetch_byte();
    }

    // 0x27
    // DAA
    fn op_daa(&mut self) {}

    // 0x28
    // JR Z, r8
    fn op_jr_z_r8(&mut self) {}

    // 0x29
    // ADD HL, HL
    // add contents of HL to HL, and store result in HL
    fn op_add_hl_hl(&mut self) {
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        // half carry set when the lower 12 bits carry into bit 12
        let half_carry: bool = (hl & 0x0FFF) + (hl & 0x0FFF) > 0x0FFF;
        // carry set when the full 16-bit addition overflows
        let carry: bool = (hl as u32) + (hl as u32) > 0xFFFF;
        let sum: u16 = hl.wrapping_add(hl);

        self.reg.h = (sum >> 8) as u8;
        self.reg.l = (sum & 0x00FF) as u8;

        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, half_carry);
        self.reg.set_flag(Flag::C, carry);
    }

    // 0x2A
    // LD A, (HL+)
    // load value at memory addressed by HL into register A, then increment HL
    fn op_ld_a_mem_hli(&mut self) {
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        self.reg.a = self.bus.read(hl);

        // increment HL and store back into the register pair
        let hl_increm: u16 = hl.wrapping_add(0x0001);
        self.reg.h = (hl_increm >> 8) as u8;
        self.reg.l = (hl_increm & 0x00FF) as u8;
    }

    // 0x2B
    // DEC HL
    // decrement the contents of register pair HL
    fn op_dec_hl(&mut self) {
        let high_byte: u16 = (self.reg.h as u16) << 8;
        let low_byte: u16 = self.reg.l as u16;

        let hl_decrem: u16 = (high_byte | low_byte).wrapping_sub(0x0001);

        self.reg.h = (hl_decrem >> 8) as u8;
        self.reg.l = (hl_decrem & 0x00FF) as u8;
    }

    // 0x2C
    // INC L
    fn op_inc_l(&mut self) {
        let half_carry = (self.reg.l & 0x0F) == 0x0F;

        // increment register l by 1
        self.reg.l = self.reg.l.wrapping_add(0x01);

        // if increment wraps register l back to 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.l == 0x00);

        // reset n (subtraction flag)
        self.reg.set_flag(Flag::N, false);

        // if value in register l overflows to high nibble, set H flag to 1
        // ex. 0x0F -> 0x10
        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x2D
    // DEC L
    fn op_dec_l(&mut self) {
        // borrow from bit 4 occurs when the low nibble is 0
        let half_carry = (self.reg.l & 0x0F) == 0x00;

        // decrement register l by 1
        self.reg.l = self.reg.l.wrapping_sub(0x01);

        // if decrement results in 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.l == 0x00);

        // set n (subtraction flag)
        self.reg.set_flag(Flag::N, true);

        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x2E
    // LD L, d8
    fn op_ld_l_d8(&mut self) {
        self.reg.l = self.fetch_byte();
    }

    // 0x2F
    // CPL
    fn op_cpl(&mut self) {}

    // 0x30
    // JR NC, r8
    fn op_jr_nc_r8(&mut self) {}

    // 0x31
    // LD SP, d16
    fn op_ld_sp_d16(&mut self) {
        // stored little endian, so fetch the low byte first
        let low: u16 = self.fetch_byte() as u16;
        let high: u16 = self.fetch_byte() as u16;

        self.reg.sp = (high << 8) | low;
    }

    // 0x32
    // LD (HL-), A
    // store contents of register A at memory addressed by HL, then decrement HL
    fn op_ld_mem_hld_a(&mut self) {
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        self.bus.write(hl, self.reg.a);

        // decrement HL and store back into the register pair
        let hl_decrem: u16 = hl.wrapping_sub(0x0001);
        self.reg.h = (hl_decrem >> 8) as u8;
        self.reg.l = (hl_decrem & 0x00FF) as u8;
    }

    // 0x33
    // INC SP
    // increment the stack pointer
    fn op_inc_sp(&mut self) {
        self.reg.sp = self.reg.sp.wrapping_add(0x0001);
    }

    // 0x34
    // INC (HL)
    fn op_inc_mem_hl(&mut self) {}

    // 0x35
    // DEC (HL)
    fn op_dec_mem_hl(&mut self) {}

    // 0x36
    // LD (HL), d8
    fn op_ld_mem_hl_d8(&mut self) {}

    // 0x37
    // SCF
    fn op_scf(&mut self) {}

    // 0x38
    // JR C, r8
    fn op_jr_c_r8(&mut self) {}

    // 0x39
    // ADD HL, SP
    // add contents of SP to HL, and store result in HL
    fn op_add_hl_sp(&mut self) {
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        // half carry set when the lower 12 bits carry into bit 12
        let half_carry: bool = (hl & 0x0FFF) + (self.reg.sp & 0x0FFF) > 0x0FFF;
        // carry set when the full 16-bit addition overflows
        let carry: bool = (hl as u32) + (self.reg.sp as u32) > 0xFFFF;
        let sum: u16 = self.reg.sp.wrapping_add(hl);

        self.reg.h = (sum >> 8) as u8;
        self.reg.l = (sum & 0x00FF) as u8;

        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, half_carry);
        self.reg.set_flag(Flag::C, carry);
    }

    // 0x3A
    // LD A, (HL-)
    // load value at memory addressed by HL into register A, then decrement HL
    fn op_ld_a_mem_hld(&mut self) {
        let hl: u16 = ((self.reg.h as u16) << 8) | (self.reg.l as u16);

        self.reg.a = self.bus.read(hl);

        // decrement HL and store back into the register pair
        let hl_decrem: u16 = hl.wrapping_sub(0x0001);
        self.reg.h = (hl_decrem >> 8) as u8;
        self.reg.l = (hl_decrem & 0x00FF) as u8;
    }

    // 0x3B
    // DEC SP
    // decrement the stack pointer
    fn op_dec_sp(&mut self) {
        self.reg.sp = self.reg.sp.wrapping_sub(0x0001);
    }

    // 0x3C
    // INC A
    fn op_inc_a(&mut self) {
        let half_carry = (self.reg.a & 0x0F) == 0x0F;

        // increment register a by 1
        self.reg.a = self.reg.a.wrapping_add(0x01);

        // if increment wraps register a back to 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.a == 0x00);

        // reset n (subtraction flag)
        self.reg.set_flag(Flag::N, false);

        // if value in register a overflows to high nibble, set H flag to 1
        // ex. 0x0F -> 0x10
        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x3D
    // DEC A
    fn op_dec_a(&mut self) {
        // borrow from bit 4 occurs when the low nibble is 0
        let half_carry = (self.reg.a & 0x0F) == 0x00;

        // decrement register a by 1
        self.reg.a = self.reg.a.wrapping_sub(0x01);

        // if decrement results in 0x00, set Z flag to 1
        self.reg.set_flag(Flag::Z, self.reg.a == 0x00);

        // set n (subtraction flag)
        self.reg.set_flag(Flag::N, true);

        self.reg.set_flag(Flag::H, half_carry);
    }

    // 0x3E
    // LD A, d8
    fn op_ld_a_d8(&mut self) {
        self.reg.a = self.fetch_byte();
    }

    // 0x3F
    // CCF
    fn op_ccf(&mut self) {}

    // 0x40
    // LD B, B
    fn op_ld_b_b(&mut self) {}

    // 0x41
    // LD B, C
    fn op_ld_b_c(&mut self) {}

    // 0x42
    // LD B, D
    fn op_ld_b_d(&mut self) {}

    // 0x43
    // LD B, E
    fn op_ld_b_e(&mut self) {}

    // 0x44
    // LD B, H
    fn op_ld_b_h(&mut self) {}

    // 0x45
    // LD B, L
    fn op_ld_b_l(&mut self) {}

    // 0x46
    // LD B, (HL)
    fn op_ld_b_mem_hl(&mut self) {}

    // 0x47
    // LD B, A
    fn op_ld_b_a(&mut self) {}

    // 0x48
    // LD C, B
    fn op_ld_c_b(&mut self) {}

    // 0x49
    // LD C, C
    fn op_ld_c_c(&mut self) {}

    // 0x4A
    // LD C, D
    fn op_ld_c_d(&mut self) {}

    // 0x4B
    // LD C, E
    fn op_ld_c_e(&mut self) {}

    // 0x4C
    // LD C, H
    fn op_ld_c_h(&mut self) {}

    // 0x4D
    // LD C, L
    fn op_ld_c_l(&mut self) {}

    // 0x4E
    // LD C, (HL)
    fn op_ld_c_mem_hl(&mut self) {}

    // 0x4F
    // LD C, A
    fn op_ld_c_a(&mut self) {}

    // 0x50
    // LD D, B
    fn op_ld_d_b(&mut self) {}

    // 0x51
    // LD D, C
    fn op_ld_d_c(&mut self) {}

    // 0x52
    // LD D, D
    fn op_ld_d_d(&mut self) {}

    // 0x53
    // LD D, E
    fn op_ld_d_e(&mut self) {}

    // 0x54
    // LD D, H
    fn op_ld_d_h(&mut self) {}

    // 0x55
    // LD D, L
    fn op_ld_d_l(&mut self) {}

    // 0x56
    // LD D, (HL)
    fn op_ld_d_mem_hl(&mut self) {}

    // 0x57
    // LD D, A
    fn op_ld_d_a(&mut self) {}

    // 0x58
    // LD E, B
    fn op_ld_e_b(&mut self) {}

    // 0x59
    // LD E, C
    fn op_ld_e_c(&mut self) {}

    // 0x5A
    // LD E, D
    fn op_ld_e_d(&mut self) {}

    // 0x5B
    // LD E, E
    fn op_ld_e_e(&mut self) {}

    // 0x5C
    // LD E, H
    fn op_ld_e_h(&mut self) {}

    // 0x5D
    // LD E, L
    fn op_ld_e_l(&mut self) {}

    // 0x5E
    // LD E, (HL)
    fn op_ld_e_mem_hl(&mut self) {}

    // 0x5F
    // LD E, A
    fn op_ld_e_a(&mut self) {}

    // 0x60
    // LD H, B
    fn op_ld_h_b(&mut self) {}

    // 0x61
    // LD H, C
    fn op_ld_h_c(&mut self) {}

    // 0x62
    // LD H, D
    fn op_ld_h_d(&mut self) {}

    // 0x63
    // LD H, E
    fn op_ld_h_e(&mut self) {}

    // 0x64
    // LD H, H
    fn op_ld_h_h(&mut self) {}

    // 0x65
    // LD H, L
    fn op_ld_h_l(&mut self) {}

    // 0x66
    // LD H, (HL)
    fn op_ld_h_mem_hl(&mut self) {}

    // 0x67
    // LD H, A
    fn op_ld_h_a(&mut self) {}

    // 0x68
    // LD L, B
    fn op_ld_l_b(&mut self) {}

    // 0x69
    // LD L, C
    fn op_ld_l_c(&mut self) {}

    // 0x6A
    // LD L, D
    fn op_ld_l_d(&mut self) {}

    // 0x6B
    // LD L, E
    fn op_ld_l_e(&mut self) {}

    // 0x6C
    // LD L, H
    fn op_ld_l_h(&mut self) {}

    // 0x6D
    // LD L, L
    fn op_ld_l_l(&mut self) {}

    // 0x6E
    // LD L, (HL)
    fn op_ld_l_mem_hl(&mut self) {}

    // 0x6F
    // LD L, A
    fn op_ld_l_a(&mut self) {}

    // 0x70
    // LD (HL), B
    fn op_ld_mem_hl_b(&mut self) {}

    // 0x71
    // LD (HL), C
    fn op_ld_mem_hl_c(&mut self) {}

    // 0x72
    // LD (HL), D
    fn op_ld_mem_hl_d(&mut self) {}

    // 0x73
    // LD (HL), E
    fn op_ld_mem_hl_e(&mut self) {}

    // 0x74
    // LD (HL), H
    fn op_ld_mem_hl_h(&mut self) {}

    // 0x75
    // LD (HL), L
    fn op_ld_mem_hl_l(&mut self) {}

    // 0x76
    // HALT
    fn op_halt(&mut self) {}

    // 0x77
    // LD (HL), A
    fn op_ld_mem_hl_a(&mut self) {}

    // 0x78
    // LD A, B
    fn op_ld_a_b(&mut self) {}

    // 0x79
    // LD A, C
    fn op_ld_a_c(&mut self) {}

    // 0x7A
    // LD A, D
    fn op_ld_a_d(&mut self) {}

    // 0x7B
    // LD A, E
    fn op_ld_a_e(&mut self) {}

    // 0x7C
    // LD A, H
    fn op_ld_a_h(&mut self) {}

    // 0x7D
    // LD A, L
    fn op_ld_a_l(&mut self) {}

    // 0x7E
    // LD A, (HL)
    fn op_ld_a_mem_hl(&mut self) {}

    // 0x7F
    // LD A, A
    fn op_ld_a_a(&mut self) {}

    // 0x80
    // ADD A, B
    fn op_add_a_b(&mut self) {}

    // 0x81
    // ADD A, C
    fn op_add_a_c(&mut self) {}

    // 0x82
    // ADD A, D
    fn op_add_a_d(&mut self) {}

    // 0x83
    // ADD A, E
    fn op_add_a_e(&mut self) {}

    // 0x84
    // ADD A, H
    fn op_add_a_h(&mut self) {}

    // 0x85
    // ADD A, L
    fn op_add_a_l(&mut self) {}

    // 0x86
    // ADD A, (HL)
    fn op_add_a_mem_hl(&mut self) {}

    // 0x87
    // ADD A, A
    fn op_add_a_a(&mut self) {}

    // 0x88
    // ADC A, B
    fn op_adc_a_b(&mut self) {}

    // 0x89
    // ADC A, C
    fn op_adc_a_c(&mut self) {}

    // 0x8A
    // ADC A, D
    fn op_adc_a_d(&mut self) {}

    // 0x8B
    // ADC A, E
    fn op_adc_a_e(&mut self) {}

    // 0x8C
    // ADC A, H
    fn op_adc_a_h(&mut self) {}

    // 0x8D
    // ADC A, L
    fn op_adc_a_l(&mut self) {}

    // 0x8E
    // ADC A, (HL)
    fn op_adc_a_mem_hl(&mut self) {}

    // 0x8F
    // ADC A, A
    fn op_adc_a_a(&mut self) {}

    // 0x90
    // SUB B
    fn op_sub_b(&mut self) {}

    // 0x91
    // SUB C
    fn op_sub_c(&mut self) {}

    // 0x92
    // SUB D
    fn op_sub_d(&mut self) {}

    // 0x93
    // SUB E
    fn op_sub_e(&mut self) {}

    // 0x94
    // SUB H
    fn op_sub_h(&mut self) {}

    // 0x95
    // SUB L
    fn op_sub_l(&mut self) {}

    // 0x96
    // SUB (HL)
    fn op_sub_mem_hl(&mut self) {}

    // 0x97
    // SUB A
    fn op_sub_a(&mut self) {}

    // 0x98
    // SBC A, B
    fn op_sbc_a_b(&mut self) {}

    // 0x99
    // SBC A, C
    fn op_sbc_a_c(&mut self) {}

    // 0x9A
    // SBC A, D
    fn op_sbc_a_d(&mut self) {}

    // 0x9B
    // SBC A, E
    fn op_sbc_a_e(&mut self) {}

    // 0x9C
    // SBC A, H
    fn op_sbc_a_h(&mut self) {}

    // 0x9D
    // SBC A, L
    fn op_sbc_a_l(&mut self) {}

    // 0x9E
    // SBC A, (HL)
    fn op_sbc_a_mem_hl(&mut self) {}

    // 0x9F
    // SBC A, A
    fn op_sbc_a_a(&mut self) {}

    // 0xA0
    // AND B
    fn op_and_b(&mut self) {}

    // 0xA1
    // AND C
    fn op_and_c(&mut self) {}

    // 0xA2
    // AND D
    fn op_and_d(&mut self) {}

    // 0xA3
    // AND E
    fn op_and_e(&mut self) {}

    // 0xA4
    // AND H
    fn op_and_h(&mut self) {}

    // 0xA5
    // AND L
    fn op_and_l(&mut self) {}

    // 0xA6
    // AND (HL)
    fn op_and_mem_hl(&mut self) {}

    // 0xA7
    // AND A
    fn op_and_a(&mut self) {}

    // 0xA8
    // XOR B
    fn op_xor_b(&mut self) {}

    // 0xA9
    // XOR C
    fn op_xor_c(&mut self) {}

    // 0xAA
    // XOR D
    fn op_xor_d(&mut self) {}

    // 0xAB
    // XOR E
    fn op_xor_e(&mut self) {}

    // 0xAC
    // XOR H
    fn op_xor_h(&mut self) {}

    // 0xAD
    // XOR L
    fn op_xor_l(&mut self) {}

    // 0xAE
    // XOR (HL)
    fn op_xor_mem_hl(&mut self) {}

    // 0xAF
    // XOR A
    fn op_xor_a(&mut self) {}

    // 0xB0
    // OR B
    fn op_or_b(&mut self) {}

    // 0xB1
    // OR C
    fn op_or_c(&mut self) {}

    // 0xB2
    // OR D
    fn op_or_d(&mut self) {}

    // 0xB3
    // OR E
    fn op_or_e(&mut self) {}

    // 0xB4
    // OR H
    fn op_or_h(&mut self) {}

    // 0xB5
    // OR L
    fn op_or_l(&mut self) {}

    // 0xB6
    // OR (HL)
    fn op_or_mem_hl(&mut self) {}

    // 0xB7
    // OR A
    fn op_or_a(&mut self) {}

    // 0xB8
    // CP B
    fn op_cp_b(&mut self) {}

    // 0xB9
    // CP C
    fn op_cp_c(&mut self) {}

    // 0xBA
    // CP D
    fn op_cp_d(&mut self) {}

    // 0xBB
    // CP E
    fn op_cp_e(&mut self) {}

    // 0xBC
    // CP H
    fn op_cp_h(&mut self) {}

    // 0xBD
    // CP L
    fn op_cp_l(&mut self) {}

    // 0xBE
    // CP (HL)
    fn op_cp_mem_hl(&mut self) {}

    // 0xBF
    // CP A
    fn op_cp_a(&mut self) {}

    // 0xC0
    // RET NZ
    fn op_ret_nz(&mut self) {}

    // 0xC1
    // POP BC
    fn op_pop_bc(&mut self) {}

    // 0xC2
    // JP NZ, a16
    fn op_jp_nz_a16(&mut self) {}

    // 0xC3
    // JP a16
    fn op_jp_a16(&mut self) {}

    // 0xC4
    // CALL NZ, a16
    fn op_call_nz_a16(&mut self) {}

    // 0xC5
    // PUSH BC
    fn op_push_bc(&mut self) {}

    // 0xC6
    // ADD A, d8
    fn op_add_a_d8(&mut self) {}

    // 0xC7
    // RST 00H
    fn op_rst_00h(&mut self) {}

    // 0xC8
    // RET Z
    fn op_ret_z(&mut self) {}

    // 0xC9
    // RET
    fn op_ret(&mut self) {}

    // 0xCA
    // JP Z, a16
    fn op_jp_z_a16(&mut self) {}

    // 0xCB
    // PREFIX CB
    fn op_prefix_cb(&mut self) {}

    // 0xCC
    // CALL Z, a16
    fn op_call_z_a16(&mut self) {}

    // 0xCD
    // CALL a16
    fn op_call_a16(&mut self) {}

    // 0xCE
    // ADC A, d8
    fn op_adc_a_d8(&mut self) {}

    // 0xCF
    // RST 08H
    fn op_rst_08h(&mut self) {}

    // 0xD0
    // RET NC
    fn op_ret_nc(&mut self) {}

    // 0xD1
    // POP DE
    fn op_pop_de(&mut self) {}

    // 0xD2
    // JP NC, a16
    fn op_jp_nc_a16(&mut self) {}

    // 0xD4
    // CALL NC, a16
    fn op_call_nc_a16(&mut self) {}

    // 0xD5
    // PUSH DE
    fn op_push_de(&mut self) {}

    // 0xD6
    // SUB d8
    fn op_sub_d8(&mut self) {}

    // 0xD7
    // RST 10H
    fn op_rst_10h(&mut self) {}

    // 0xD8
    // RET C
    fn op_ret_c(&mut self) {}

    // 0xD9
    // RETI
    fn op_reti(&mut self) {}

    // 0xDA
    // JP C, a16
    fn op_jp_c_a16(&mut self) {}

    // 0xDC
    // CALL C, a16
    fn op_call_c_a16(&mut self) {}

    // 0xDE
    // SBC A, d8
    fn op_sbc_a_d8(&mut self) {}

    // 0xDF
    // RST 18H
    fn op_rst_18h(&mut self) {}

    // 0xE0
    // LDH (a8), A
    fn op_ldh_mem_a8_a(&mut self) {}

    // 0xE1
    // POP HL
    fn op_pop_hl(&mut self) {}

    // 0xE2
    // LD (C), A
    fn op_ld_mem_c_a(&mut self) {}

    // 0xE5
    // PUSH HL
    fn op_push_hl(&mut self) {}

    // 0xE6
    // AND d8
    fn op_and_d8(&mut self) {}

    // 0xE7
    // RST 20H
    fn op_rst_20h(&mut self) {}

    // 0xE8
    // ADD SP, r8
    fn op_add_sp_r8(&mut self) {}

    // 0xE9
    // JP (HL)
    fn op_jp_hl(&mut self) {}

    // 0xEA
    // LD (a16), A
    fn op_ld_mem_a16_a(&mut self) {}

    // 0xEE
    // XOR d8
    fn op_xor_d8(&mut self) {}

    // 0xEF
    // RST 28H
    fn op_rst_28h(&mut self) {}

    // 0xF0
    // LDH A, (a8)
    fn op_ldh_a_mem_a8(&mut self) {}

    // 0xF1
    // POP AF
    fn op_pop_af(&mut self) {}

    // 0xF2
    // LD A, (C)
    fn op_ld_a_mem_c(&mut self) {}

    // 0xF3
    // DI
    fn op_di(&mut self) {}

    // 0xF5
    // PUSH AF
    fn op_push_af(&mut self) {}

    // 0xF6
    // OR d8
    fn op_or_d8(&mut self) {}

    // 0xF7
    // RST 30H
    fn op_rst_30h(&mut self) {}

    // 0xF8
    // LD HL, SP+r8
    fn op_ld_hl_sp_r8(&mut self) {}

    // 0xF9
    // LD SP, HL
    fn op_ld_sp_hl(&mut self) {}

    // 0xFA
    // LD A, (a16)
    fn op_ld_a_mem_a16(&mut self) {}

    // 0xFB
    // EI
    fn op_ei(&mut self) {}

    // 0xFE
    // CP d8
    fn op_cp_d8(&mut self) {}

    // 0xFF
    // RST 38H
    fn op_rst_38h(&mut self) {}

}
