#![cfg(all(feature = "math", feature = "builder"))]
use neko_cat_macros::{Builder, Math};

#[derive(Debug, Builder, Math, Clone)]
struct CalcStruct {
    #[opt(default = 10_usize)]
    usize: usize,
    #[opt(default = 20_u8)]
    u8: u8,
    #[opt(default = 30_u16)]
    u16: u16,
    #[opt(default = 40_u32)]
    u32: u32,
    #[opt(default = 50_u64)]
    u64: u64,
    #[opt(default = 60_u128)]
    u128: u128,
    #[opt(default = 70_isize)]
    isize: isize,
    #[opt(default = 80_i16)]
    i16: i16,
    #[opt(default = 90_i32)]
    i32: i32,
    #[opt(default = 100_i64)]
    i64: i64,
    #[opt(default = 110_i128)]
    i128: i128,
    #[opt(default = 120.0_f64)]
    f64: f64,
    #[opt(default = 130.0_f32)]
    f32: f32,
}

#[test]
fn approach_usize() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_usize(20, 6);
    assert_eq!(calc_struct.usize, 16);
    calc_struct.approach_usize(20, 6);
    assert_eq!(calc_struct.usize, 20);
}

#[test]
fn approach_u8() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_u8(0, 5);
    assert_eq!(calc_struct.u8, 15);
    calc_struct.approach_u8(0, 5);
    assert_eq!(calc_struct.u8, 10);
}

#[test]
fn approach_u16() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_u16(50, 15);
    assert_eq!(calc_struct.u16, 45);
    calc_struct.approach_u16(50, 15);
    assert_eq!(calc_struct.u16, 50);
}

#[test]
fn approach_u32() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_u32(30, 5);
    assert_eq!(calc_struct.u32, 35);
    calc_struct.approach_u32(30, 5);
    assert_eq!(calc_struct.u32, 30);
}

#[test]
fn approach_u64() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_u64(100, 30);
    assert_eq!(calc_struct.u64, 80);
    calc_struct.approach_u64(100, 30);
    assert_eq!(calc_struct.u64, 100);
}

#[test]
fn approach_u128() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_u128(90, 20);
    assert_eq!(calc_struct.u128, 80);
    calc_struct.approach_u128(90, 20);
    assert_eq!(calc_struct.u128, 90);
}

#[test]
fn approach_isize() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_isize(50, 15);
    assert_eq!(calc_struct.isize, 55);
    calc_struct.approach_isize(50, 15);
    assert_eq!(calc_struct.isize, 50);
}

#[test]
fn approach_i16() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_i16(100, 10);
    assert_eq!(calc_struct.i16, 90);
    calc_struct.approach_i16(100, 10);
    assert_eq!(calc_struct.i16, 100);
}

#[test]
fn approach_i32() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_i32(50, 25);
    assert_eq!(calc_struct.i32, 65);
    calc_struct.approach_i32(50, 25);
    assert_eq!(calc_struct.i32, 50);
}

#[test]
fn approach_i64() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_i64(120, 15);
    assert_eq!(calc_struct.i64, 115);
    calc_struct.approach_i64(120, 15);
    assert_eq!(calc_struct.i64, 120);
}

#[test]
fn approach_i128() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_i128(100, 8);
    assert_eq!(calc_struct.i128, 102);
    calc_struct.approach_i128(100, 8);
    assert_eq!(calc_struct.i128, 100);
}

#[test]
fn approach_f64() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_f64(130.0, 5.0);
    assert_eq!(calc_struct.f64, 125.0);
    calc_struct.approach_f64(130.0, 5.0);
    assert_eq!(calc_struct.f64, 130.0);
}

#[test]
fn approach_f32() {
    let mut calc_struct = CalcStruct::new();
    calc_struct.approach_f32(120.0, 8.0);
    assert_eq!(calc_struct.f32, 122.0);
    calc_struct.approach_f32(120.0, 8.0);
    assert_eq!(calc_struct.f32, 120.0);
}

#[test]
fn sum_usize() {
    let mut s = CalcStruct::new();
    s.sum_usize(5);
    assert_eq!(s.usize, 15);
}

#[test]
fn sum_u8() {
    let mut s = CalcStruct::new();
    s.sum_u8(10);
    assert_eq!(s.u8, 30);
}

#[test]
fn sum_u16() {
    let mut s = CalcStruct::new();
    s.sum_u16(20);
    assert_eq!(s.u16, 50);
}

#[test]
fn sum_u32() {
    let mut s = CalcStruct::new();
    s.sum_u32(5);
    assert_eq!(s.u32, 45);
}

#[test]
fn sum_u64() {
    let mut s = CalcStruct::new();
    s.sum_u64(25);
    assert_eq!(s.u64, 75);
}

#[test]
fn sum_u128() {
    let mut s = CalcStruct::new();
    s.sum_u128(40);
    assert_eq!(s.u128, 100);
}

#[test]
fn sum_isize() {
    let mut s = CalcStruct::new();
    s.sum_isize(5);
    assert_eq!(s.isize, 75);
}

#[test]
fn sum_i16() {
    let mut s = CalcStruct::new();
    s.sum_i16(-30);
    assert_eq!(s.i16, 50);
}

#[test]
fn sum_i32() {
    let mut s = CalcStruct::new();
    s.sum_i32(10);
    assert_eq!(s.i32, 100);
}

#[test]
fn sum_i64() {
    let mut s = CalcStruct::new();
    s.sum_i64(-50);
    assert_eq!(s.i64, 50);
}

#[test]
fn sum_i128() {
    let mut s = CalcStruct::new();
    s.sum_i128(10);
    assert_eq!(s.i128, 120);
}

#[test]
fn sum_f64() {
    let mut s = CalcStruct::new();
    s.sum_f64(5.0);
    assert_eq!(s.f64, 125.0);
}

#[test]
fn sum_f32() {
    let mut s = CalcStruct::new();
    s.sum_f32(5.0);
    assert_eq!(s.f32, 135.0);
}

#[test]
fn sub_usize() {
    let mut s = CalcStruct::new();
    s.sub_usize(5);
    assert_eq!(s.usize, 5);
}

#[test]
fn sub_u8() {
    let mut s = CalcStruct::new();
    s.sub_u8(10);
    assert_eq!(s.u8, 10);
}

#[test]
fn sub_u16() {
    let mut s = CalcStruct::new();
    s.sub_u16(5);
    assert_eq!(s.u16, 25);
}

#[test]
fn sub_u32() {
    let mut s = CalcStruct::new();
    s.sub_u32(15);
    assert_eq!(s.u32, 25);
}

#[test]
fn sub_u64() {
    let mut s = CalcStruct::new();
    s.sub_u64(20);
    assert_eq!(s.u64, 30);
}

#[test]
fn sub_u128() {
    let mut s = CalcStruct::new();
    s.sub_u128(10);
    assert_eq!(s.u128, 50);
}

#[test]
fn sub_isize() {
    let mut s = CalcStruct::new();
    s.sub_isize(15);
    assert_eq!(s.isize, 55);
}

#[test]
fn sub_i16() {
    let mut s = CalcStruct::new();
    s.sub_i16(50);
    assert_eq!(s.i16, 30);
}

#[test]
fn sub_i32() {
    let mut s = CalcStruct::new();
    s.sub_i32(-10);
    assert_eq!(s.i32, 100);
}

#[test]
fn sub_i64() {
    let mut s = CalcStruct::new();
    s.sub_i64(70);
    assert_eq!(s.i64, 30);
}

#[test]
fn sub_i128() {
    let mut s = CalcStruct::new();
    s.sub_i128(10);
    assert_eq!(s.i128, 100);
}

#[test]
fn sub_f64() {
    let mut s = CalcStruct::new();
    s.sub_f64(20.5);
    assert_eq!(s.f64, 99.5);
}

#[test]
fn sub_f32() {
    let mut s = CalcStruct::new();
    s.sub_f32(10.5);
    assert_eq!(s.f32, 119.5);
}

#[test]
fn div_usize() {
    let mut s = CalcStruct::new();
    s.div_usize(2);
    assert_eq!(s.usize, 5);
}

#[test]
fn div_u8() {
    let mut s = CalcStruct::new();
    s.div_u8(2);
    assert_eq!(s.u8, 10);
}

#[test]
fn div_u16() {
    let mut s = CalcStruct::new();
    s.div_u16(3);
    assert_eq!(s.u16, 10);
}

#[test]
fn div_u32() {
    let mut s = CalcStruct::new();
    s.div_u32(5);
    assert_eq!(s.u32, 8);
}

#[test]
fn div_u64() {
    let mut s = CalcStruct::new();
    s.div_u64(10);
    assert_eq!(s.u64, 5);
}

#[test]
fn div_u128() {
    let mut s = CalcStruct::new();
    s.div_u128(4);
    assert_eq!(s.u128, 15);
}

#[test]
fn div_isize() {
    let mut s = CalcStruct::new();
    s.div_isize(2);
    assert_eq!(s.isize, 35);
}

#[test]
fn div_i16() {
    let mut s = CalcStruct::new();
    s.div_i16(4);
    assert_eq!(s.i16, 20);
}

#[test]
fn div_i32() {
    let mut s = CalcStruct::new();
    s.div_i32(-3);
    assert_eq!(s.i32, -30);
}

#[test]
fn div_i64() {
    let mut s = CalcStruct::new();
    s.div_i64(4);
    assert_eq!(s.i64, 25);
}

#[test]
fn div_i128() {
    let mut s = CalcStruct::new();
    s.div_i128(10);
    assert_eq!(s.i128, 11);
}

#[test]
fn div_f64() {
    let mut s = CalcStruct::new();
    s.div_f64(2.0);
    assert_eq!(s.f64, 60.0);
}

#[test]
fn div_f32() {
    let mut s = CalcStruct::new();
    s.div_f32(2.0);
    assert_eq!(s.f32, 65.0);
}

#[test]
fn mul_usize() {
    let mut s = CalcStruct::new();
    s.mul_usize(3);
    assert_eq!(s.usize, 30);
}

#[test]
fn mul_u8() {
    let mut s = CalcStruct::new();
    s.mul_u8(2);
    assert_eq!(s.u8, 40);
}

#[test]
fn mul_u16() {
    let mut s = CalcStruct::new();
    s.mul_u16(2);
    assert_eq!(s.u16, 60);
}

#[test]
fn mul_u32() {
    let mut s = CalcStruct::new();
    s.mul_u32(3);
    assert_eq!(s.u32, 120);
}

#[test]
fn mul_u64() {
    let mut s = CalcStruct::new();
    s.mul_u64(4);
    assert_eq!(s.u64, 200);
}

#[test]
fn mul_u128() {
    let mut s = CalcStruct::new();
    s.mul_u128(2);
    assert_eq!(s.u128, 120);
}

#[test]
fn mul_isize() {
    let mut s = CalcStruct::new();
    s.mul_isize(2);
    assert_eq!(s.isize, 140);
}

#[test]
fn mul_i16() {
    let mut s = CalcStruct::new();
    s.mul_i16(-1);
    assert_eq!(s.i16, -80);
}

#[test]
fn mul_i32() {
    let mut s = CalcStruct::new();
    s.mul_i32(0);
    assert_eq!(s.i32, 0);
}

#[test]
fn mul_i64() {
    let mut s = CalcStruct::new();
    s.mul_i64(2);
    assert_eq!(s.i64, 200);
}

#[test]
fn mul_i128() {
    let mut s = CalcStruct::new();
    s.mul_i128(3);
    assert_eq!(s.i128, 330);
}

#[test]
fn mul_f64() {
    let mut s = CalcStruct::new();
    s.mul_f64(1.5);
    assert_eq!(s.f64, 180.0);
}

#[test]
fn mul_f32() {
    let mut s = CalcStruct::new();
    s.mul_f32(0.5);
    assert_eq!(s.f32, 65.0);
}

#[test]
fn inflate_usize() {
    let mut s = CalcStruct::new();
    s.inflate_usize(50); // +50%
    assert_eq!(s.usize, 15);
}

#[test]
fn inflate_u8() {
    let mut s = CalcStruct::new();
    s.inflate_u8(50);
    assert_eq!(s.u8, 30);
}

#[test]
fn inflate_u16() {
    let mut s = CalcStruct::new();
    s.inflate_u16(10);
    assert_eq!(s.u16, 33);
}

#[test]
fn inflate_u32() {
    let mut s = CalcStruct::new();
    s.inflate_u32(100);
    assert_eq!(s.u32, 80);
}

#[test]
fn inflate_u64() {
    let mut s = CalcStruct::new();
    s.inflate_u64(20);
    assert_eq!(s.u64, 60);
}

#[test]
fn inflate_u128() {
    let mut s = CalcStruct::new();
    s.inflate_u128(10);
    assert_eq!(s.u128, 66);
}

#[test]
fn inflate_isize() {
    let mut s = CalcStruct::new();
    s.inflate_isize(50);
    assert_eq!(s.isize, 105);
}

#[test]
fn inflate_i16() {
    let mut s = CalcStruct::new();
    s.inflate_i16(25);
    assert_eq!(s.i16, 100);
}

#[test]
fn inflate_i32() {
    let mut s = CalcStruct::new();
    s.inflate_i32(10);
    assert_eq!(s.i32, 99);
}

#[test]
fn inflate_i64() {
    let mut s = CalcStruct::new();
    s.inflate_i64(200);
    assert_eq!(s.i64, 300);
}

#[test]
fn inflate_i128() {
    let mut s = CalcStruct::new();
    s.inflate_i128(50);
    assert_eq!(s.i128, 165);
}

#[test]
fn inflate_f64() {
    let mut s = CalcStruct::new();
    s.inflate_f64(10.0);
    assert!((s.f64 - 132.0).abs() < 1e-6);
}

#[test]
fn inflate_u8_caps_at_max() {
    let mut s = CalcStruct::new().u8(CalcStructU8(100));
    s.inflate_u8(255);
    assert_eq!(s.u8, u8::MAX);
}

#[test]
fn inflate_u128_caps_at_max() {
    let mut s = CalcStruct::new().u128(CalcStructU128(1_000_000_000_000_000_000_000));
    s.inflate_u128(1_000_000_000_000_000_000_000);
    assert_eq!(s.u128, u128::MAX);
}

#[test]
fn inflate_f64_infinite_percentage_keeps_original() {
    let mut s = CalcStruct::new();
    s.inflate_f64(f64::INFINITY);
    assert!((s.f64 - 120.0).abs() < 1e-6);
}

#[test]
fn inflate_f32_nan_percentage_keeps_original() {
    let mut s = CalcStruct::new();
    s.inflate_f32(f32::NAN);
    assert!((s.f32 - 130.0).abs() < 1e-3);
}

#[test]
fn inflate_f32() {
    let mut s = CalcStruct::new();
    s.inflate_f32(50.0);
    assert!((s.f32 - 195.0).abs() < 1e-3);
}

#[test]
fn discount_usize() {
    let mut s = CalcStruct::new();
    s.discount_usize(50);
    assert_eq!(s.usize, 5);
}

#[test]
fn discount_u8() {
    let mut s = CalcStruct::new();
    s.discount_u8(25);
    assert_eq!(s.u8, 15);
}

#[test]
fn discount_u16() {
    let mut s = CalcStruct::new();
    s.discount_u16(10);
    assert_eq!(s.u16, 27);
}

#[test]
fn discount_u32() {
    let mut s = CalcStruct::new();
    s.discount_u32(50);
    assert_eq!(s.u32, 20);
}

#[test]
fn discount_u64() {
    let mut s = CalcStruct::new();
    s.discount_u64(20);
    assert_eq!(s.u64, 40);
}

#[test]
fn discount_u128() {
    let mut s = CalcStruct::new();
    s.discount_u128(10);
    assert_eq!(s.u128, 54);
}

#[test]
fn discount_isize() {
    let mut s = CalcStruct::new();
    s.discount_isize(50);
    assert_eq!(s.isize, 35);
}

#[test]
fn discount_i16() {
    let mut s = CalcStruct::new();
    s.discount_i16(25);
    assert_eq!(s.i16, 60);
}

#[test]
fn discount_i32() {
    let mut s = CalcStruct::new();
    s.discount_i32(10);
    assert_eq!(s.i32, 81);
}

#[test]
fn discount_i64() {
    let mut s = CalcStruct::new();
    s.discount_i64(50);
    assert_eq!(s.i64, 50);
}

#[test]
fn discount_i128() {
    let mut s = CalcStruct::new();
    s.discount_i128(50);
    assert_eq!(s.i128, 55);
}

#[test]
fn discount_f64() {
    let mut s = CalcStruct::new();
    s.discount_f64(10.0);
    assert!((s.f64 - 108.0).abs() < 1e-6);
}

#[test]
fn discount_f32() {
    let mut s = CalcStruct::new();
    s.discount_f32(50.0);
    assert!((s.f32 - 65.0).abs() < 1e-3);
}

#[test]
fn discount_u8_cant_go_negative() {
    let mut s = CalcStruct::new();
    s.discount_u8(200);
    assert_eq!(s.u8, 0);
}

#[test]
fn discount_u128_big_percent_caps_at_zero() {
    let mut s = CalcStruct::new();
    s.discount_u128(1_000_000);
    assert_eq!(s.u128, 0);
}

#[test]
fn discount_i32_allow_negative_if_small_percent_over_100() {
    let mut s = CalcStruct::new();
    s.discount_i32(200);
    assert_eq!(s.i32, 0);
}

#[test]
fn discount_f64_large_percent_goes_to_zero() {
    let mut s = CalcStruct::new();
    s.discount_f64(1000.0);
    assert!((s.f64 - 0.0).abs() < 1e-6);
}

#[test]
fn discount_f32_zero_percent_no_change() {
    let mut s = CalcStruct::new();
    s.discount_f32(0.0);
    assert!((s.f32 - 130.0).abs() < 1e-3);
}
