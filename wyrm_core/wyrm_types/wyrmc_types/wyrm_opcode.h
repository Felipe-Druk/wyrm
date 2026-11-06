#pragma once

typedef enum WyrmOpcode {
    // Opcodes para el stack
    WYRM_OP_PUSH = 0x00,
    WYRM_OP_POP = 0x01,
    // Opcodes operadores aritméticos
    WYRM_OP_ADD = 0x02,
    WYRM_OP_SUB = 0x03,
    WYRM_OP_MUL = 0x04,
    WYRM_OP_DIV = 0x05,
    WYRM_OP_MOD = 0x06,
    // Opcodes operadores lógicos
    WYRM_OP_AND = 0x07,
    WYRM_OP_OR = 0x08,
    WYRM_OP_NOT = 0x09
} wyrm_opcode_t;