///AArch64 Relocation Constants.
pub const R_AARCH64_NONE: usize = 0;
pub const R_AARCH64_P32_ABS32: usize = 1;
pub const R_AARCH64_P32_COPY: usize = 180;
pub const R_AARCH64_P32_GLOB_DAT: usize = 181;
pub const R_AARCH64_P32_JUMP_SLOT: usize = 182;
pub const R_AARCH64_P32_RELATIVE: usize = 183;
pub const R_AARCH64_P32_TLS_DTPMOD: usize = 184;
pub const R_AARCH64_P32_TLS_DTPREL: usize = 185;
pub const R_AARCH64_P32_TLS_TPREL: usize = 186;
pub const R_AARCH64_P32_TLSDESC: usize = 187;
pub const R_AARCH64_P32_IRELATIVE: usize = 188;
pub const R_AARCH64_ABS64: usize = 257;
pub const R_AARCH64_ABS32: usize = 258;
pub const R_AARCH64_ABS16: usize = 259;
pub const R_AARCH64_PREL64: usize = 260;
pub const R_AARCH64_PREL32: usize = 261;
pub const R_AARCH64_PREL16: usize = 262;
pub const R_AARCH64_MOVW_UABS_G0: usize = 263;
pub const R_AARCH64_MOVW_UABS_G0_NC: usize = 264;
pub const R_AARCH64_MOVW_UABS_G1: usize = 265;
pub const R_AARCH64_MOVW_UABS_G1_NC: usize = 266;
pub const R_AARCH64_MOVW_UABS_G2: usize = 267;
pub const R_AARCH64_MOVW_UABS_G2_NC: usize = 268;
pub const R_AARCH64_MOVW_UABS_G3: usize = 269;
pub const R_AARCH64_MOVW_SABS_G0: usize = 270;
pub const R_AARCH64_MOVW_SABS_G1: usize = 271;
pub const R_AARCH64_MOVW_SABS_G2: usize = 272;
pub const R_AARCH64_LD_PREL_LO19: usize = 273;
pub const R_AARCH64_ADR_PREL_LO21: usize = 274;
pub const R_AARCH64_ADR_PREL_PG_HI21: usize = 275;
pub const R_AARCH64_ADR_PREL_PG_HI21_NC: usize = 276;
pub const R_AARCH64_ADD_ABS_LO12_NC: usize = 277;
pub const R_AARCH64_LDST8_ABS_LO12_NC: usize = 278;
pub const R_AARCH64_TSTBR14: usize = 279;
pub const R_AARCH64_CONDBR19: usize = 280;
pub const R_AARCH64_JUMP26: usize = 282;
pub const R_AARCH64_CALL26: usize = 283;
pub const R_AARCH64_LDST16_ABS_LO12_NC: usize = 284;
pub const R_AARCH64_LDST32_ABS_LO12_NC: usize = 285;
pub const R_AARCH64_LDST64_ABS_LO12_NC: usize = 286;
pub const R_AARCH64_MOVW_PREL_G0: usize = 287;
pub const R_AARCH64_MOVW_PREL_G0_NC: usize = 288;
pub const R_AARCH64_MOVW_PREL_G1: usize = 289;
pub const R_AARCH64_MOVW_PREL_G1_NC: usize = 290;
pub const R_AARCH64_MOVW_PREL_G2: usize = 291;
pub const R_AARCH64_MOVW_PREL_G2_NC: usize = 292;
pub const R_AARCH64_MOVW_PREL_G3: usize = 293;
pub const R_AARCH64_LDST128_ABS_LO12_NC: usize = 299;
pub const R_AARCH64_MOVW_GOTOFF_G0: usize = 300;
pub const R_AARCH64_MOVW_GOTOFF_G0_NC: usize = 301;
pub const R_AARCH64_MOVW_GOTOFF_G1: usize = 302;
pub const R_AARCH64_MOVW_GOTOFF_G1_NC: usize = 303;
pub const R_AARCH64_MOVW_GOTOFF_G2: usize = 304;
pub const R_AARCH64_MOVW_GOTOFF_G2_NC: usize = 305;
pub const R_AARCH64_MOVW_GOTOFF_G3: usize = 306;
pub const R_AARCH64_GOTREL64: usize = 307;
pub const R_AARCH64_GOTREL32: usize = 308;
pub const R_AARCH64_GOT_LD_PREL19: usize = 309;
pub const R_AARCH64_LD64_GOTOFF_LO15: usize = 310;
pub const R_AARCH64_ADR_GOT_PAGE: usize = 311;
pub const R_AARCH64_LD64_GOT_LO12_NC: usize = 312;
pub const R_AARCH64_LD64_GOTPAGE_LO15: usize = 313;
pub const R_AARCH64_TLSGD_ADR_PREL21: usize = 512;
pub const R_AARCH64_TLSGD_ADR_PAGE21: usize = 513;
pub const R_AARCH64_TLSGD_ADD_LO12_NC: usize = 514;
pub const R_AARCH64_TLSGD_MOVW_G1: usize = 515;
pub const R_AARCH64_TLSGD_MOVW_G0_NC: usize = 516;
pub const R_AARCH64_TLSLD_ADR_PREL21: usize = 517;
pub const R_AARCH64_TLSLD_ADR_PAGE21: usize = 518;
pub const R_AARCH64_TLSLD_ADD_LO12_NC: usize = 519;
pub const R_AARCH64_TLSLD_MOVW_G1: usize = 520;
pub const R_AARCH64_TLSLD_MOVW_G0_NC: usize = 521;
pub const R_AARCH64_TLSLD_LD_PREL19: usize = 522;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G2: usize = 523;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G1: usize = 524;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G1_NC: usize = 525;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G0: usize = 526;
pub const R_AARCH64_TLSLD_MOVW_DTPREL_G0_NC: usize = 527;
pub const R_AARCH64_TLSLD_ADD_DTPREL_HI12: usize = 528;
pub const R_AARCH64_TLSLD_ADD_DTPREL_LO12: usize = 529;
pub const R_AARCH64_TLSLD_ADD_DTPREL_LO12_NC: usize = 530;
pub const R_AARCH64_TLSLD_LDST8_DTPREL_LO12: usize = 531;
pub const R_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC: usize = 532;
pub const R_AARCH64_TLSLD_LDST16_DTPREL_LO12: usize = 533;
pub const R_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC: usize = 534;
pub const R_AARCH64_TLSLD_LDST32_DTPREL_LO12: usize = 535;
pub const R_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC: usize = 536;
pub const R_AARCH64_TLSLD_LDST64_DTPREL_LO12: usize = 537;
pub const R_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC: usize = 538;
pub const R_AARCH64_TLSIE_MOVW_GOTTPREL_G1: usize = 539;
pub const R_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC: usize = 540;
pub const R_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21: usize = 541;
pub const R_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC: usize = 542;
pub const R_AARCH64_TLSIE_LD_GOTTPREL_PREL19: usize = 543;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G2: usize = 544;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G1: usize = 545;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G1_NC: usize = 546;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G0: usize = 547;
pub const R_AARCH64_TLSLE_MOVW_TPREL_G0_NC: usize = 548;
pub const R_AARCH64_TLSLE_ADD_TPREL_HI12: usize = 549;
pub const R_AARCH64_TLSLE_ADD_TPREL_LO12: usize = 550;
pub const R_AARCH64_TLSLE_ADD_TPREL_LO12_NC: usize = 551;
pub const R_AARCH64_TLSLE_LDST8_TPREL_LO12: usize = 552;
pub const R_AARCH64_TLSLE_LDST8_TPREL_LO12_NC: usize = 553;
pub const R_AARCH64_TLSLE_LDST16_TPREL_LO12: usize = 554;
pub const R_AARCH64_TLSLE_LDST16_TPREL_LO12_NC: usize = 555;
pub const R_AARCH64_TLSLE_LDST32_TPREL_LO12: usize = 556;
pub const R_AARCH64_TLSLE_LDST32_TPREL_LO12_NC: usize = 557;
pub const R_AARCH64_TLSLE_LDST64_TPREL_LO12: usize = 558;
pub const R_AARCH64_TLSLE_LDST64_TPREL_LO12_NC: usize = 559;
pub const R_AARCH64_TLSDESC_LD_PREL19: usize = 560;
pub const R_AARCH64_TLSDESC_ADR_PREL21: usize = 561;
pub const R_AARCH64_TLSDESC_ADR_PAGE21: usize = 562;
pub const R_AARCH64_TLSDESC_LD64_LO12: usize = 563;
pub const R_AARCH64_TLSDESC_ADD_LO12: usize = 564;
pub const R_AARCH64_TLSDESC_OFF_G1: usize = 565;
pub const R_AARCH64_TLSDESC_OFF_G0_NC: usize = 566;
pub const R_AARCH64_TLSDESC_LDR: usize = 567;
pub const R_AARCH64_TLSDESC_ADD: usize = 568;
pub const R_AARCH64_TLSDESC_CALL: usize = 569;
pub const R_AARCH64_TLSLE_LDST128_TPREL_LO12: usize = 570;
pub const R_AARCH64_TLSLE_LDST128_TPREL_LO12_NC: usize = 571;
pub const R_AARCH64_TLSLD_LDST128_DTPREL_LO12: usize = 572;
pub const R_AARCH64_TLSLD_LDST128_DTPREL_LO12_NC: usize = 573;
pub const R_AARCH64_COPY: usize = 1024;
pub const R_AARCH64_GLOB_DAT: usize = 1025;
pub const R_AARCH64_JUMP_SLOT: usize = 1026;
pub const R_AARCH64_RELATIVE: usize = 1027;
pub const R_AARCH64_TLS_DTPMOD: usize = 1028;
pub const R_AARCH64_TLS_DTPMOD64: usize = 1028;
pub const R_AARCH64_TLS_DTPREL: usize = 1029;
pub const R_AARCH64_TLS_DTPREL64: usize = 1029;
pub const R_AARCH64_TLS_TPREL: usize = 1030;
pub const R_AARCH64_TLS_TPREL64: usize = 1030;
pub const R_AARCH64_TLSDESC: usize = 1031;

// REL_OFFSET32 does not exist.
pub const REL_NONE: usize = R_AARCH64_NONE;
pub const REL_SYMBOLIC: usize = R_AARCH64_ABS64;
pub const REL_OFFSET32: usize = (-95 as isize) as usize;
pub const REL_GOT: usize = R_AARCH64_GLOB_DAT;
pub const REL_PLT: usize = R_AARCH64_JUMP_SLOT;
pub const REL_RELATIVE: usize = R_AARCH64_RELATIVE;
pub const REL_COPY: usize = R_AARCH64_COPY;
pub const REL_DTPMOD: usize = R_AARCH64_TLS_DTPMOD64;
pub const REL_DTPOFF: usize = R_AARCH64_TLS_DTPREL64;
pub const REL_TPOFF: usize = R_AARCH64_TLS_TPREL64;
pub const REL_TLSDESC: usize = R_AARCH64_TLSDESC;
