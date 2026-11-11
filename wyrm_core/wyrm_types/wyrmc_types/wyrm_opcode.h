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
    WYRM_OP_NOT = 0x09,
    // Opcodes de control de flujo
    WYRM_OP_JUMP = 0x0A,
    WYRM_OP_JUMP_IF = 0x0B,
    WYRM_OP_JUMP_IF_NOT = 0x0C,
    // Opcodes de valores
    WYRM_OP_CONSTANT = 0x0D,
    WYRM_OP_CAST = 0x0E,
    // Opcodes de funciones
    WYRM_OP_CALL = 0x0F,
    WYRM_OP_RETURN = 0x10,
    // Opcodes de variables
    WYRM_OP_GET_LOCAL = 0x11,
    WYRM_OP_SET_LOCAL = 0x12,
    // Opcodes relacionales
    WYRM_OP_LESS = 0x13,
    WYRM_OP_GREATER = 0x14,
    WYRM_OP_EQUAL = 0x15,
    WYRM_OP_NOT_EQUAL = 0x16,

} wyrm_opcode_t;