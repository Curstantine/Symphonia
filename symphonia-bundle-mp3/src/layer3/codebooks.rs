// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::io::vlc::*;

use lazy_static::lazy_static;

#[rustfmt::skip]
const MPEG_CODES_0: [u32; 0] = [ ];

#[rustfmt::skip]
const MPEG_BITS_0: [u8; 0] = [ ];

#[rustfmt::skip]
const MPEG_CODES_1: [u32; 4] = [
    0x0001, 0x0001, 0x0001, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_1: [u8; 4] = [
    1,  3,  2,  3,
];

#[rustfmt::skip]
const MPEG_CODES_2: [u32; 9] = [
    0x0001, 0x0002, 0x0001, 0x0003, 0x0001, 0x0001, 0x0003, 0x0002,
    0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_2: [u8; 9] = [
    1,  3,  6,  3,  3,  5,  5,  5,
    6,
];

#[rustfmt::skip]
const MPEG_CODES_3: [u32; 9] = [
    0x0003, 0x0002, 0x0001, 0x0001, 0x0001, 0x0001, 0x0003, 0x0002,
    0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_3: [u8; 9] = [
    2,  2,  6,  3,  2,  5,  5,  5,
    6,
];

#[rustfmt::skip]
const MPEG_CODES_5: [u32; 16] = [
    0x0001, 0x0002, 0x0006, 0x0005, 0x0003, 0x0001, 0x0004, 0x0004,
    0x0007, 0x0005, 0x0007, 0x0001, 0x0006, 0x0001, 0x0001, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_5: [u8; 16] = [
    1,  3,  6,  7,  3,  3,  6,  7,
    6,  6,  7,  8,  7,  6,  7,  8,
];

#[rustfmt::skip]
const MPEG_CODES_6: [u32; 16] = [
    0x0007, 0x0003, 0x0005, 0x0001, 0x0006, 0x0002, 0x0003, 0x0002,
    0x0005, 0x0004, 0x0004, 0x0001, 0x0003, 0x0003, 0x0002, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_6: [u8; 16] = [
    3,  3,  5,  7,  3,  2,  4,  5,
    4,  4,  5,  6,  6,  5,  6,  7,
];

#[rustfmt::skip]
const MPEG_CODES_7: [u32; 36] = [
    0x0001, 0x0002, 0x000a, 0x0013, 0x0010, 0x000a, 0x0003, 0x0003,
    0x0007, 0x000a, 0x0005, 0x0003, 0x000b, 0x0004, 0x000d, 0x0011,
    0x0008, 0x0004, 0x000c, 0x000b, 0x0012, 0x000f, 0x000b, 0x0002,
    0x0007, 0x0006, 0x0009, 0x000e, 0x0003, 0x0001, 0x0006, 0x0004,
    0x0005, 0x0003, 0x0002, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_7: [u8; 36] = [
    1,  3,  6,  8,  8,  9,  3,  4,
    6,  7,  7,  8,  6,  5,  7,  8,
    8,  9,  7,  7,  8,  9,  9,  9,
    7,  7,  8,  9,  9, 10,  8,  8,
    9, 10, 10, 10,
];

#[rustfmt::skip]
const MPEG_CODES_8: [u32; 36] = [
    0x0003, 0x0004, 0x0006, 0x0012, 0x000c, 0x0005, 0x0005, 0x0001,
    0x0002, 0x0010, 0x0009, 0x0003, 0x0007, 0x0003, 0x0005, 0x000e,
    0x0007, 0x0003, 0x0013, 0x0011, 0x000f, 0x000d, 0x000a, 0x0004,
    0x000d, 0x0005, 0x0008, 0x000b, 0x0005, 0x0001, 0x000c, 0x0004,
    0x0004, 0x0001, 0x0001, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_8: [u8; 36] = [
    2,  3,  6,  8,  8,  9,  3,  2,
    4,  8,  8,  8,  6,  4,  6,  8,
    8,  9,  8,  8,  8,  9,  9, 10,
    8,  7,  8,  9, 10, 10,  9,  8,
    9,  9, 11, 11,
];

#[rustfmt::skip]
const MPEG_CODES_9: [u32; 36] = [
    0x0007, 0x0005, 0x0009, 0x000e, 0x000f, 0x0007, 0x0006, 0x0004,
    0x0005, 0x0005, 0x0006, 0x0007, 0x0007, 0x0006, 0x0008, 0x0008,
    0x0008, 0x0005, 0x000f, 0x0006, 0x0009, 0x000a, 0x0005, 0x0001,
    0x000b, 0x0007, 0x0009, 0x0006, 0x0004, 0x0001, 0x000e, 0x0004,
    0x0006, 0x0002, 0x0006, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_9: [u8; 36] = [
    3,  3,  5,  6,  8,  9,  3,  3,
    4,  5,  6,  8,  4,  4,  5,  6,
    7,  8,  6,  5,  6,  7,  7,  8,
    7,  6,  7,  7,  8,  9,  8,  7,
    8,  8,  9,  9,
];

#[rustfmt::skip]
const MPEG_CODES_10: [u32; 64] = [
    0x0001, 0x0002, 0x000a, 0x0017, 0x0023, 0x001e, 0x000c, 0x0011,
    0x0003, 0x0003, 0x0008, 0x000c, 0x0012, 0x0015, 0x000c, 0x0007,
    0x000b, 0x0009, 0x000f, 0x0015, 0x0020, 0x0028, 0x0013, 0x0006,
    0x000e, 0x000d, 0x0016, 0x0022, 0x002e, 0x0017, 0x0012, 0x0007,
    0x0014, 0x0013, 0x0021, 0x002f, 0x001b, 0x0016, 0x0009, 0x0003,
    0x001f, 0x0016, 0x0029, 0x001a, 0x0015, 0x0014, 0x0005, 0x0003,
    0x000e, 0x000d, 0x000a, 0x000b, 0x0010, 0x0006, 0x0005, 0x0001,
    0x0009, 0x0008, 0x0007, 0x0008, 0x0004, 0x0004, 0x0002, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_10: [u8; 64] = [
    1,  3,  6,  8,  9,  9,  9, 10,
    3,  4,  6,  7,  8,  9,  8,  8,
    6,  6,  7,  8,  9, 10,  9,  9,
    7,  7,  8,  9, 10, 10,  9, 10,
    8,  8,  9, 10, 10, 10, 10, 10,
    9,  9, 10, 10, 11, 11, 10, 11,
    8,  8,  9, 10, 10, 10, 11, 11,
    9,  8,  9, 10, 10, 11, 11, 11,
];

#[rustfmt::skip]
const MPEG_CODES_11: [u32; 64] = [
    0x0003, 0x0004, 0x000a, 0x0018, 0x0022, 0x0021, 0x0015, 0x000f,
    0x0005, 0x0003, 0x0004, 0x000a, 0x0020, 0x0011, 0x000b, 0x000a,
    0x000b, 0x0007, 0x000d, 0x0012, 0x001e, 0x001f, 0x0014, 0x0005,
    0x0019, 0x000b, 0x0013, 0x003b, 0x001b, 0x0012, 0x000c, 0x0005,
    0x0023, 0x0021, 0x001f, 0x003a, 0x001e, 0x0010, 0x0007, 0x0005,
    0x001c, 0x001a, 0x0020, 0x0013, 0x0011, 0x000f, 0x0008, 0x000e,
    0x000e, 0x000c, 0x0009, 0x000d, 0x000e, 0x0009, 0x0004, 0x0001,
    0x000b, 0x0004, 0x0006, 0x0006, 0x0006, 0x0003, 0x0002, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_11: [u8; 64] = [
    2,  3,  5,  7,  8,  9,  8,  9,
    3,  3,  4,  6,  8,  8,  7,  8,
    5,  5,  6,  7,  8,  9,  8,  8,
    7,  6,  7,  9,  8, 10,  8,  9,
    8,  8,  8,  9,  9, 10,  9, 10,
    8,  8,  9, 10, 10, 11, 10, 11,
    8,  7,  7,  8,  9, 10, 10, 10,
    8,  7,  8,  9, 10, 10, 10, 10,
];

#[rustfmt::skip]
const MPEG_CODES_12: [u32; 64] = [
    0x0009, 0x0006, 0x0010, 0x0021, 0x0029, 0x0027, 0x0026, 0x001a,
    0x0007, 0x0005, 0x0006, 0x0009, 0x0017, 0x0010, 0x001a, 0x000b,
    0x0011, 0x0007, 0x000b, 0x000e, 0x0015, 0x001e, 0x000a, 0x0007,
    0x0011, 0x000a, 0x000f, 0x000c, 0x0012, 0x001c, 0x000e, 0x0005,
    0x0020, 0x000d, 0x0016, 0x0013, 0x0012, 0x0010, 0x0009, 0x0005,
    0x0028, 0x0011, 0x001f, 0x001d, 0x0011, 0x000d, 0x0004, 0x0002,
    0x001b, 0x000c, 0x000b, 0x000f, 0x000a, 0x0007, 0x0004, 0x0001,
    0x001b, 0x000c, 0x0008, 0x000c, 0x0006, 0x0003, 0x0001, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_12: [u8; 64] = [
    4,  3,  5,  7,  8,  9,  9,  9,
    3,  3,  4,  5,  7,  7,  8,  8,
    5,  4,  5,  6,  7,  8,  7,  8,
    6,  5,  6,  6,  7,  8,  8,  8,
    7,  6,  7,  7,  8,  8,  8,  9,
    8,  7,  8,  8,  8,  9,  8,  9,
    8,  7,  7,  8,  8,  9,  9, 10,
    9,  8,  8,  9,  9,  9,  9, 10,
];

#[rustfmt::skip]
const MPEG_CODES_13: [u32; 256] = [
    0x0001, 0x0005, 0x000e, 0x0015, 0x0022, 0x0033, 0x002e, 0x0047,
    0x002a, 0x0034, 0x0044, 0x0034, 0x0043, 0x002c, 0x002b, 0x0013,
    0x0003, 0x0004, 0x000c, 0x0013, 0x001f, 0x001a, 0x002c, 0x0021,
    0x001f, 0x0018, 0x0020, 0x0018, 0x001f, 0x0023, 0x0016, 0x000e,
    0x000f, 0x000d, 0x0017, 0x0024, 0x003b, 0x0031, 0x004d, 0x0041,
    0x001d, 0x0028, 0x001e, 0x0028, 0x001b, 0x0021, 0x002a, 0x0010,
    0x0016, 0x0014, 0x0025, 0x003d, 0x0038, 0x004f, 0x0049, 0x0040,
    0x002b, 0x004c, 0x0038, 0x0025, 0x001a, 0x001f, 0x0019, 0x000e,
    0x0023, 0x0010, 0x003c, 0x0039, 0x0061, 0x004b, 0x0072, 0x005b,
    0x0036, 0x0049, 0x0037, 0x0029, 0x0030, 0x0035, 0x0017, 0x0018,
    0x003a, 0x001b, 0x0032, 0x0060, 0x004c, 0x0046, 0x005d, 0x0054,
    0x004d, 0x003a, 0x004f, 0x001d, 0x004a, 0x0031, 0x0029, 0x0011,
    0x002f, 0x002d, 0x004e, 0x004a, 0x0073, 0x005e, 0x005a, 0x004f,
    0x0045, 0x0053, 0x0047, 0x0032, 0x003b, 0x0026, 0x0024, 0x000f,
    0x0048, 0x0022, 0x0038, 0x005f, 0x005c, 0x0055, 0x005b, 0x005a,
    0x0056, 0x0049, 0x004d, 0x0041, 0x0033, 0x002c, 0x002b, 0x002a,
    0x002b, 0x0014, 0x001e, 0x002c, 0x0037, 0x004e, 0x0048, 0x0057,
    0x004e, 0x003d, 0x002e, 0x0036, 0x0025, 0x001e, 0x0014, 0x0010,
    0x0035, 0x0019, 0x0029, 0x0025, 0x002c, 0x003b, 0x0036, 0x0051,
    0x0042, 0x004c, 0x0039, 0x0036, 0x0025, 0x0012, 0x0027, 0x000b,
    0x0023, 0x0021, 0x001f, 0x0039, 0x002a, 0x0052, 0x0048, 0x0050,
    0x002f, 0x003a, 0x0037, 0x0015, 0x0016, 0x001a, 0x0026, 0x0016,
    0x0035, 0x0019, 0x0017, 0x0026, 0x0046, 0x003c, 0x0033, 0x0024,
    0x0037, 0x001a, 0x0022, 0x0017, 0x001b, 0x000e, 0x0009, 0x0007,
    0x0022, 0x0020, 0x001c, 0x0027, 0x0031, 0x004b, 0x001e, 0x0034,
    0x0030, 0x0028, 0x0034, 0x001c, 0x0012, 0x0011, 0x0009, 0x0005,
    0x002d, 0x0015, 0x0022, 0x0040, 0x0038, 0x0032, 0x0031, 0x002d,
    0x001f, 0x0013, 0x000c, 0x000f, 0x000a, 0x0007, 0x0006, 0x0003,
    0x0030, 0x0017, 0x0014, 0x0027, 0x0024, 0x0023, 0x0035, 0x0015,
    0x0010, 0x0017, 0x000d, 0x000a, 0x0006, 0x0001, 0x0004, 0x0002,
    0x0010, 0x000f, 0x0011, 0x001b, 0x0019, 0x0014, 0x001d, 0x000b,
    0x0011, 0x000c, 0x0010, 0x0008, 0x0001, 0x0001, 0x0000, 0x0001,
];

#[rustfmt::skip]
const MPEG_BITS_13: [u8; 256] = [
     1,  4,  6,  7,  8,  9,  9, 10,
     9, 10, 11, 11, 12, 12, 13, 13,
     3,  4,  6,  7,  8,  8,  9,  9,
     9,  9, 10, 10, 11, 12, 12, 12,
     6,  6,  7,  8,  9,  9, 10, 10,
     9, 10, 10, 11, 11, 12, 13, 13,
     7,  7,  8,  9,  9, 10, 10, 10,
    10, 11, 11, 11, 11, 12, 13, 13,
     8,  7,  9,  9, 10, 10, 11, 11,
    10, 11, 11, 12, 12, 13, 13, 14,
     9,  8,  9, 10, 10, 10, 11, 11,
    11, 11, 12, 11, 13, 13, 14, 14,
     9,  9, 10, 10, 11, 11, 11, 11,
    11, 12, 12, 12, 13, 13, 14, 14,
    10,  9, 10, 11, 11, 11, 12, 12,
    12, 12, 13, 13, 13, 14, 16, 16,
     9,  8,  9, 10, 10, 11, 11, 12,
    12, 12, 12, 13, 13, 14, 15, 15,
    10,  9, 10, 10, 11, 11, 11, 13,
    12, 13, 13, 14, 14, 14, 16, 15,
    10, 10, 10, 11, 11, 12, 12, 13,
    12, 13, 14, 13, 14, 15, 16, 17,
    11, 10, 10, 11, 12, 12, 12, 12,
    13, 13, 13, 14, 15, 15, 15, 16,
    11, 11, 11, 12, 12, 13, 12, 13,
    14, 14, 15, 15, 15, 16, 16, 16,
    12, 11, 12, 13, 13, 13, 14, 14,
    14, 14, 14, 15, 16, 15, 16, 16,
    13, 12, 12, 13, 13, 13, 15, 14,
    14, 17, 15, 15, 15, 17, 16, 16,
    12, 12, 13, 14, 14, 14, 15, 14,
    15, 15, 16, 16, 19, 18, 19, 16,
];

#[rustfmt::skip]
const MPEG_CODES_15: [u32; 256] = [
    0x0007, 0x000c, 0x0012, 0x0035, 0x002f, 0x004c, 0x007c, 0x006c,
    0x0059, 0x007b, 0x006c, 0x0077, 0x006b, 0x0051, 0x007a, 0x003f,
    0x000d, 0x0005, 0x0010, 0x001b, 0x002e, 0x0024, 0x003d, 0x0033,
    0x002a, 0x0046, 0x0034, 0x0053, 0x0041, 0x0029, 0x003b, 0x0024,
    0x0013, 0x0011, 0x000f, 0x0018, 0x0029, 0x0022, 0x003b, 0x0030,
    0x0028, 0x0040, 0x0032, 0x004e, 0x003e, 0x0050, 0x0038, 0x0021,
    0x001d, 0x001c, 0x0019, 0x002b, 0x0027, 0x003f, 0x0037, 0x005d,
    0x004c, 0x003b, 0x005d, 0x0048, 0x0036, 0x004b, 0x0032, 0x001d,
    0x0034, 0x0016, 0x002a, 0x0028, 0x0043, 0x0039, 0x005f, 0x004f,
    0x0048, 0x0039, 0x0059, 0x0045, 0x0031, 0x0042, 0x002e, 0x001b,
    0x004d, 0x0025, 0x0023, 0x0042, 0x003a, 0x0034, 0x005b, 0x004a,
    0x003e, 0x0030, 0x004f, 0x003f, 0x005a, 0x003e, 0x0028, 0x0026,
    0x007d, 0x0020, 0x003c, 0x0038, 0x0032, 0x005c, 0x004e, 0x0041,
    0x0037, 0x0057, 0x0047, 0x0033, 0x0049, 0x0033, 0x0046, 0x001e,
    0x006d, 0x0035, 0x0031, 0x005e, 0x0058, 0x004b, 0x0042, 0x007a,
    0x005b, 0x0049, 0x0038, 0x002a, 0x0040, 0x002c, 0x0015, 0x0019,
    0x005a, 0x002b, 0x0029, 0x004d, 0x0049, 0x003f, 0x0038, 0x005c,
    0x004d, 0x0042, 0x002f, 0x0043, 0x0030, 0x0035, 0x0024, 0x0014,
    0x0047, 0x0022, 0x0043, 0x003c, 0x003a, 0x0031, 0x0058, 0x004c,
    0x0043, 0x006a, 0x0047, 0x0036, 0x0026, 0x0027, 0x0017, 0x000f,
    0x006d, 0x0035, 0x0033, 0x002f, 0x005a, 0x0052, 0x003a, 0x0039,
    0x0030, 0x0048, 0x0039, 0x0029, 0x0017, 0x001b, 0x003e, 0x0009,
    0x0056, 0x002a, 0x0028, 0x0025, 0x0046, 0x0040, 0x0034, 0x002b,
    0x0046, 0x0037, 0x002a, 0x0019, 0x001d, 0x0012, 0x000b, 0x000b,
    0x0076, 0x0044, 0x001e, 0x0037, 0x0032, 0x002e, 0x004a, 0x0041,
    0x0031, 0x0027, 0x0018, 0x0010, 0x0016, 0x000d, 0x000e, 0x0007,
    0x005b, 0x002c, 0x0027, 0x0026, 0x0022, 0x003f, 0x0034, 0x002d,
    0x001f, 0x0034, 0x001c, 0x0013, 0x000e, 0x0008, 0x0009, 0x0003,
    0x007b, 0x003c, 0x003a, 0x0035, 0x002f, 0x002b, 0x0020, 0x0016,
    0x0025, 0x0018, 0x0011, 0x000c, 0x000f, 0x000a, 0x0002, 0x0001,
    0x0047, 0x0025, 0x0022, 0x001e, 0x001c, 0x0014, 0x0011, 0x001a,
    0x0015, 0x0010, 0x000a, 0x0006, 0x0008, 0x0006, 0x0002, 0x0000,
];

#[rustfmt::skip]
const MPEG_BITS_15: [u8; 256] = [
     3,  4,  5,  7,  7,  8,  9,  9,
     9, 10, 10, 11, 11, 11, 12, 13,
     4,  3,  5,  6,  7,  7,  8,  8,
     8,  9,  9, 10, 10, 10, 11, 11,
     5,  5,  5,  6,  7,  7,  8,  8,
     8,  9,  9, 10, 10, 11, 11, 11,
     6,  6,  6,  7,  7,  8,  8,  9,
     9,  9, 10, 10, 10, 11, 11, 11,
     7,  6,  7,  7,  8,  8,  9,  9,
     9,  9, 10, 10, 10, 11, 11, 11,
     8,  7,  7,  8,  8,  8,  9,  9,
     9,  9, 10, 10, 11, 11, 11, 12,
     9,  7,  8,  8,  8,  9,  9,  9,
     9, 10, 10, 10, 11, 11, 12, 12,
     9,  8,  8,  9,  9,  9,  9, 10,
    10, 10, 10, 10, 11, 11, 11, 12,
     9,  8,  8,  9,  9,  9,  9, 10,
    10, 10, 10, 11, 11, 12, 12, 12,
     9,  8,  9,  9,  9,  9, 10, 10,
    10, 11, 11, 11, 11, 12, 12, 12,
    10,  9,  9,  9, 10, 10, 10, 10,
    10, 11, 11, 11, 11, 12, 13, 12,
    10,  9,  9,  9, 10, 10, 10, 10,
    11, 11, 11, 11, 12, 12, 12, 13,
    11, 10,  9, 10, 10, 10, 11, 11,
    11, 11, 11, 11, 12, 12, 13, 13,
    11, 10, 10, 10, 10, 11, 11, 11,
    11, 12, 12, 12, 12, 12, 13, 13,
    12, 11, 11, 11, 11, 11, 11, 11,
    12, 12, 12, 12, 13, 13, 12, 13,
    12, 11, 11, 11, 11, 11, 11, 12,
    12, 12, 12, 12, 13, 13, 13, 13,
];

#[rustfmt::skip]
const MPEG_CODES_16: [u32; 256] = [
    0x0001, 0x0005, 0x000e, 0x002c, 0x004a, 0x003f, 0x006e, 0x005d,
    0x00ac, 0x0095, 0x008a, 0x00f2, 0x00e1, 0x00c3, 0x0178, 0x0011,
    0x0003, 0x0004, 0x000c, 0x0014, 0x0023, 0x003e, 0x0035, 0x002f,
    0x0053, 0x004b, 0x0044, 0x0077, 0x00c9, 0x006b, 0x00cf, 0x0009,
    0x000f, 0x000d, 0x0017, 0x0026, 0x0043, 0x003a, 0x0067, 0x005a,
    0x00a1, 0x0048, 0x007f, 0x0075, 0x006e, 0x00d1, 0x00ce, 0x0010,
    0x002d, 0x0015, 0x0027, 0x0045, 0x0040, 0x0072, 0x0063, 0x0057,
    0x009e, 0x008c, 0x00fc, 0x00d4, 0x00c7, 0x0183, 0x016d, 0x001a,
    0x004b, 0x0024, 0x0044, 0x0041, 0x0073, 0x0065, 0x00b3, 0x00a4,
    0x009b, 0x0108, 0x00f6, 0x00e2, 0x018b, 0x017e, 0x016a, 0x0009,
    0x0042, 0x001e, 0x003b, 0x0038, 0x0066, 0x00b9, 0x00ad, 0x0109,
    0x008e, 0x00fd, 0x00e8, 0x0190, 0x0184, 0x017a, 0x01bd, 0x0010,
    0x006f, 0x0036, 0x0034, 0x0064, 0x00b8, 0x00b2, 0x00a0, 0x0085,
    0x0101, 0x00f4, 0x00e4, 0x00d9, 0x0181, 0x016e, 0x02cb, 0x000a,
    0x0062, 0x0030, 0x005b, 0x0058, 0x00a5, 0x009d, 0x0094, 0x0105,
    0x00f8, 0x0197, 0x018d, 0x0174, 0x017c, 0x0379, 0x0374, 0x0008,
    0x0055, 0x0054, 0x0051, 0x009f, 0x009c, 0x008f, 0x0104, 0x00f9,
    0x01ab, 0x0191, 0x0188, 0x017f, 0x02d7, 0x02c9, 0x02c4, 0x0007,
    0x009a, 0x004c, 0x0049, 0x008d, 0x0083, 0x0100, 0x00f5, 0x01aa,
    0x0196, 0x018a, 0x0180, 0x02df, 0x0167, 0x02c6, 0x0160, 0x000b,
    0x008b, 0x0081, 0x0043, 0x007d, 0x00f7, 0x00e9, 0x00e5, 0x00db,
    0x0189, 0x02e7, 0x02e1, 0x02d0, 0x0375, 0x0372, 0x01b7, 0x0004,
    0x00f3, 0x0078, 0x0076, 0x0073, 0x00e3, 0x00df, 0x018c, 0x02ea,
    0x02e6, 0x02e0, 0x02d1, 0x02c8, 0x02c2, 0x00df, 0x01b4, 0x0006,
    0x00ca, 0x00e0, 0x00de, 0x00da, 0x00d8, 0x0185, 0x0182, 0x017d,
    0x016c, 0x0378, 0x01bb, 0x02c3, 0x01b8, 0x01b5, 0x06c0, 0x0004,
    0x02eb, 0x00d3, 0x00d2, 0x00d0, 0x0172, 0x017b, 0x02de, 0x02d3,
    0x02ca, 0x06c7, 0x0373, 0x036d, 0x036c, 0x0d83, 0x0361, 0x0002,
    0x0179, 0x0171, 0x0066, 0x00bb, 0x02d6, 0x02d2, 0x0166, 0x02c7,
    0x02c5, 0x0362, 0x06c6, 0x0367, 0x0d82, 0x0366, 0x01b2, 0x0000,
    0x000c, 0x000a, 0x0007, 0x000b, 0x000a, 0x0011, 0x000b, 0x0009,
    0x000d, 0x000c, 0x000a, 0x0007, 0x0005, 0x0003, 0x0001, 0x0003,
];

#[rustfmt::skip]
const MPEG_BITS_16: [u8; 256] = [
     1,  4,  6,  8,  9,  9, 10, 10,
    11, 11, 11, 12, 12, 12, 13,  9,
     3,  4,  6,  7,  8,  9,  9,  9,
    10, 10, 10, 11, 12, 11, 12,  8,
     6,  6,  7,  8,  9,  9, 10, 10,
    11, 10, 11, 11, 11, 12, 12,  9,
     8,  7,  8,  9,  9, 10, 10, 10,
    11, 11, 12, 12, 12, 13, 13, 10,
     9,  8,  9,  9, 10, 10, 11, 11,
    11, 12, 12, 12, 13, 13, 13,  9,
     9,  8,  9,  9, 10, 11, 11, 12,
    11, 12, 12, 13, 13, 13, 14, 10,
    10,  9,  9, 10, 11, 11, 11, 11,
    12, 12, 12, 12, 13, 13, 14, 10,
    10,  9, 10, 10, 11, 11, 11, 12,
    12, 13, 13, 13, 13, 15, 15, 10,
    10, 10, 10, 11, 11, 11, 12, 12,
    13, 13, 13, 13, 14, 14, 14, 10,
    11, 10, 10, 11, 11, 12, 12, 13,
    13, 13, 13, 14, 13, 14, 13, 11,
    11, 11, 10, 11, 12, 12, 12, 12,
    13, 14, 14, 14, 15, 15, 14, 10,
    12, 11, 11, 11, 12, 12, 13, 14,
    14, 14, 14, 14, 14, 13, 14, 11,
    12, 12, 12, 12, 12, 13, 13, 13,
    13, 15, 14, 14, 14, 14, 16, 11,
    14, 12, 12, 12, 13, 13, 14, 14,
    14, 16, 15, 15, 15, 17, 15, 11,
    13, 13, 11, 12, 14, 14, 13, 14,
    14, 15, 16, 15, 17, 15, 14, 11,
     9,  8,  8,  9,  9, 10, 10, 10,
    11, 11, 11, 11, 11, 11, 11,  8,
];

#[rustfmt::skip]
const MPEG_CODES_24: [u32; 256] = [
    0x000f, 0x000d, 0x002e, 0x0050, 0x0092, 0x0106, 0x00f8, 0x01b2,
    0x01aa, 0x029d, 0x028d, 0x0289, 0x026d, 0x0205, 0x0408, 0x0058,
    0x000e, 0x000c, 0x0015, 0x0026, 0x0047, 0x0082, 0x007a, 0x00d8,
    0x00d1, 0x00c6, 0x0147, 0x0159, 0x013f, 0x0129, 0x0117, 0x002a,
    0x002f, 0x0016, 0x0029, 0x004a, 0x0044, 0x0080, 0x0078, 0x00dd,
    0x00cf, 0x00c2, 0x00b6, 0x0154, 0x013b, 0x0127, 0x021d, 0x0012,
    0x0051, 0x0027, 0x004b, 0x0046, 0x0086, 0x007d, 0x0074, 0x00dc,
    0x00cc, 0x00be, 0x00b2, 0x0145, 0x0137, 0x0125, 0x010f, 0x0010,
    0x0093, 0x0048, 0x0045, 0x0087, 0x007f, 0x0076, 0x0070, 0x00d2,
    0x00c8, 0x00bc, 0x0160, 0x0143, 0x0132, 0x011d, 0x021c, 0x000e,
    0x0107, 0x0042, 0x0081, 0x007e, 0x0077, 0x0072, 0x00d6, 0x00ca,
    0x00c0, 0x00b4, 0x0155, 0x013d, 0x012d, 0x0119, 0x0106, 0x000c,
    0x00f9, 0x007b, 0x0079, 0x0075, 0x0071, 0x00d7, 0x00ce, 0x00c3,
    0x00b9, 0x015b, 0x014a, 0x0134, 0x0123, 0x0110, 0x0208, 0x000a,
    0x01b3, 0x0073, 0x006f, 0x006d, 0x00d3, 0x00cb, 0x00c4, 0x00bb,
    0x0161, 0x014c, 0x0139, 0x012a, 0x011b, 0x0213, 0x017d, 0x0011,
    0x01ab, 0x00d4, 0x00d0, 0x00cd, 0x00c9, 0x00c1, 0x00ba, 0x00b1,
    0x00a9, 0x0140, 0x012f, 0x011e, 0x010c, 0x0202, 0x0179, 0x0010,
    0x014f, 0x00c7, 0x00c5, 0x00bf, 0x00bd, 0x00b5, 0x00ae, 0x014d,
    0x0141, 0x0131, 0x0121, 0x0113, 0x0209, 0x017b, 0x0173, 0x000b,
    0x029c, 0x00b8, 0x00b7, 0x00b3, 0x00af, 0x0158, 0x014b, 0x013a,
    0x0130, 0x0122, 0x0115, 0x0212, 0x017f, 0x0175, 0x016e, 0x000a,
    0x028c, 0x015a, 0x00ab, 0x00a8, 0x00a4, 0x013e, 0x0135, 0x012b,
    0x011f, 0x0114, 0x0107, 0x0201, 0x0177, 0x0170, 0x016a, 0x0006,
    0x0288, 0x0142, 0x013c, 0x0138, 0x0133, 0x012e, 0x0124, 0x011c,
    0x010d, 0x0105, 0x0200, 0x0178, 0x0172, 0x016c, 0x0167, 0x0004,
    0x026c, 0x012c, 0x0128, 0x0126, 0x0120, 0x011a, 0x0111, 0x010a,
    0x0203, 0x017c, 0x0176, 0x0171, 0x016d, 0x0169, 0x0165, 0x0002,
    0x0409, 0x0118, 0x0116, 0x0112, 0x010b, 0x0108, 0x0103, 0x017e,
    0x017a, 0x0174, 0x016f, 0x016b, 0x0168, 0x0166, 0x0164, 0x0000,
    0x002b, 0x0014, 0x0013, 0x0011, 0x000f, 0x000d, 0x000b, 0x0009,
    0x0007, 0x0006, 0x0004, 0x0007, 0x0005, 0x0003, 0x0001, 0x0003,
];

#[rustfmt::skip]
const MPEG_BITS_24: [u8; 256] = [
     4,  4,  6,  7,  8,  9,  9, 10,
    10, 11, 11, 11, 11, 11, 12,  9,
     4,  4,  5,  6,  7,  8,  8,  9,
     9,  9, 10, 10, 10, 10, 10,  8,
     6,  5,  6,  7,  7,  8,  8,  9,
     9,  9,  9, 10, 10, 10, 11,  7,
     7,  6,  7,  7,  8,  8,  8,  9,
     9,  9,  9, 10, 10, 10, 10,  7,
     8,  7,  7,  8,  8,  8,  8,  9,
     9,  9, 10, 10, 10, 10, 11,  7,
     9,  7,  8,  8,  8,  8,  9,  9,
     9,  9, 10, 10, 10, 10, 10,  7,
     9,  8,  8,  8,  8,  9,  9,  9,
     9, 10, 10, 10, 10, 10, 11,  7,
    10,  8,  8,  8,  9,  9,  9,  9,
    10, 10, 10, 10, 10, 11, 11,  8,
    10,  9,  9,  9,  9,  9,  9,  9,
     9, 10, 10, 10, 10, 11, 11,  8,
    10,  9,  9,  9,  9,  9,  9, 10,
    10, 10, 10, 10, 11, 11, 11,  8,
    11,  9,  9,  9,  9, 10, 10, 10,
    10, 10, 10, 11, 11, 11, 11,  8,
    11, 10,  9,  9,  9, 10, 10, 10,
    10, 10, 10, 11, 11, 11, 11,  8,
    11, 10, 10, 10, 10, 10, 10, 10,
    10, 10, 11, 11, 11, 11, 11,  8,
    11, 10, 10, 10, 10, 10, 10, 10,
    11, 11, 11, 11, 11, 11, 11,  8,
    12, 10, 10, 10, 10, 10, 10, 11,
    11, 11, 11, 11, 11, 11, 11,  8,
    8,   7,  7,  7,  7,  7,  7,  7,
    7,   7,  7,  8,  8,  8,  8,  4,
];

const MPEG_QUADS_CODES_A: [u32; 16] = [1, 5, 4, 5, 6, 5, 4, 4, 7, 3, 6, 0, 7, 2, 3, 1];
const MPEG_QUADS_BITS_A: [u8; 16] = [1, 4, 4, 5, 4, 6, 5, 6, 4, 5, 5, 6, 5, 6, 6, 6];

const MPEG_QUADS_CODES_B: [u32; 16] = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
const MPEG_QUADS_BITS_B: [u8; 16] = [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4];

struct MpegTable {
    codes: &'static [u32],
    lens: &'static [u8],
    wrap: u16,
}
struct MpegQuadsTable {
    codes: &'static [u32],
    lens: &'static [u8],
    wrap: u16,
}

const MPEG_TABLES: [MpegTable; 18] = [
    // Table 0
    MpegTable { codes: &MPEG_CODES_0, lens: &MPEG_BITS_0, wrap: 0x0 },
    // Table 1
    MpegTable { codes: &MPEG_CODES_1, lens: &MPEG_BITS_1, wrap: 0x2 },
    // Table 2
    MpegTable { codes: &MPEG_CODES_2, lens: &MPEG_BITS_2, wrap: 0x3 },
    // Table 3
    MpegTable { codes: &MPEG_CODES_3, lens: &MPEG_BITS_3, wrap: 0x3 },
    // Table 4 (not used)
    MpegTable { codes: &MPEG_CODES_0, lens: &MPEG_BITS_0, wrap: 0x0 },
    // Table 5
    MpegTable { codes: &MPEG_CODES_5, lens: &MPEG_BITS_5, wrap: 0x4 },
    // Table 6
    MpegTable { codes: &MPEG_CODES_6, lens: &MPEG_BITS_6, wrap: 0x4 },
    // Table 7
    MpegTable { codes: &MPEG_CODES_7, lens: &MPEG_BITS_7, wrap: 0x6 },
    // Table 8
    MpegTable { codes: &MPEG_CODES_8, lens: &MPEG_BITS_8, wrap: 0x6 },
    // Table 9
    MpegTable { codes: &MPEG_CODES_9, lens: &MPEG_BITS_9, wrap: 0x6 },
    // Table 10
    MpegTable { codes: &MPEG_CODES_10, lens: &MPEG_BITS_10, wrap: 0x8 },
    // Table 11
    MpegTable { codes: &MPEG_CODES_11, lens: &MPEG_BITS_11, wrap: 0x8 },
    // Table 12
    MpegTable { codes: &MPEG_CODES_12, lens: &MPEG_BITS_12, wrap: 0x8 },
    // Table 13
    MpegTable { codes: &MPEG_CODES_13, lens: &MPEG_BITS_13, wrap: 0x10 },
    // Table 14 (not used)
    MpegTable { codes: &MPEG_CODES_0, lens: &MPEG_BITS_0, wrap: 0x0 },
    // Table 15
    MpegTable { codes: &MPEG_CODES_15, lens: &MPEG_BITS_15, wrap: 0x10 },
    // Tables 16..24 (number of linbits vary)
    MpegTable { codes: &MPEG_CODES_16, lens: &MPEG_BITS_16, wrap: 0x10 },
    // Tables 24..32 (number of linbits vary)
    MpegTable { codes: &MPEG_CODES_24, lens: &MPEG_BITS_24, wrap: 0x10 },
];

const MPEG_QUADS_TABLES: [MpegQuadsTable; 2] = [
    // Table A
    MpegQuadsTable { codes: &MPEG_QUADS_CODES_A, lens: &MPEG_QUADS_BITS_A, wrap: 0x10 },
    // Table B
    MpegQuadsTable { codes: &MPEG_QUADS_CODES_B, lens: &MPEG_QUADS_BITS_B, wrap: 0x10 },
];

pub const CODEBOOK_LINBITS: [u32; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 8, 10, 13, 4, 5, 6, 7, 8, 9, 11,
    13,
];

fn mpeg_gen_value(i: u16, wrap: u16) -> u16 {
    ((i / wrap) << 4) | (i % wrap)
}

lazy_static! {
    pub static ref CODEBOOK_TABLES: [Codebook<Entry16x16>; 18] = {
        let mut codebooks: [Codebook<Entry16x16>; 18] = Default::default();

        for (codebook, table) in codebooks.iter_mut().zip(&MPEG_TABLES) {
            assert!(table.codes.len() == table.lens.len());

            let len = table.codes.len() as u16;

            // Generate values for the codebook.
            let values: Vec<u16> = (0..len).map(|i| mpeg_gen_value(i, table.wrap))
                                           .collect();

            // Generate the codebook.
            let mut builder = CodebookBuilder::new(BitOrder::Verbatim);

            // Decode a maximum of 8 bits per read.
            builder.bits_per_read(8);

            *codebook = builder.make(table.codes, table.lens, &values).unwrap();
        }

        codebooks
    };
}

lazy_static! {
    pub static ref QUADS_CODEBOOK_TABLE: [Codebook<Entry16x16>; 2] = {
        let mut codebooks: [Codebook<Entry16x16>; 2] = Default::default();

        for (codebook, table) in codebooks.iter_mut().zip(&MPEG_QUADS_TABLES) {
            assert!(table.codes.len() == table.lens.len());

            let len = table.codes.len() as u16;

            // Generate values for the codebook.
            let values: Vec<u16> = (0..len).map(|i| mpeg_gen_value(i, table.wrap))
                                           .collect();

            // Generate the codebook.
            let mut builder = CodebookBuilder::new(BitOrder::Verbatim);

            // Decode a maximum of 8 bits per read.
            builder.bits_per_read(8);

            *codebook = builder.make(table.codes, table.lens, &values).unwrap();
        }

        codebooks
    };
}
