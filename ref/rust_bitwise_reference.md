# Rust Bitwise Operations — Bit-Level Reference

A reference for every Rust bitwise operator and the bit-level operation it performs. Examples use `u8` (8 bits) so the full byte is visible.

> **Notation:** bits are written MSB→LSB. Bit `n` has place value `2^n`. So `0b1000_0001 = 2^7 + 2^0 = 129`.

---

## 1. AND `&`

**Per-bit rule:** output bit is `1` only if *both* input bits are `1`.

| a | b | a & b |
|---|---|-------|
| 0 | 0 | 0     |
| 0 | 1 | 0     |
| 1 | 0 | 0     |
| 1 | 1 | 1     |

Algebraically, for each bit position `i`:  `out_i = a_i · b_i`

**Worked example:**

```
  1100_1010   (0xCA = 202)
& 0110_0110   (0x66 = 102)
-----------
  0100_0010   (0x42 = 66)
```

```rust
let r: u8 = 0b1100_1010 & 0b0110_0110; // 0b0100_0010 = 66
```

**Primary use — masking** (force selected bits to 0, keep the rest):

```rust
let byte: u8 = 0b1011_0110;
let low_nibble = byte & 0x0F; // 0b0000_0110 — top 4 bits cleared
let bit3 = (byte >> 3) & 1;   // isolate a single bit -> 0 or 1
```

---

## 2. OR `|`

**Per-bit rule:** output bit is `1` if *either* input bit is `1`.

| a | b | a \| b |
|---|---|--------|
| 0 | 0 | 0      |
| 0 | 1 | 1      |
| 1 | 0 | 1      |
| 1 | 1 | 1      |

For each bit position `i`:  `out_i = a_i + b_i − a_i·b_i`  (saturating add — caps at 1)

**Worked example:**

```
  1100_1010   (0xCA)
| 0110_0110   (0x66)
-----------
  1110_1110   (0xEE = 238)
```

```rust
let r: u8 = 0b1100_1010 | 0b0110_0110; // 0b1110_1110 = 238
```

**Primary use — setting bits** (force selected bits to 1, keep the rest):

```rust
let flags: u8 = 0b0000_0001;
let set = flags | 0b0001_0000; // 0b0001_0001 — bit 4 forced on
```

---

## 3. XOR `^`

**Per-bit rule:** output bit is `1` if the input bits *differ*.

| a | b | a ^ b |
|---|---|-------|
| 0 | 0 | 0     |
| 0 | 1 | 1     |
| 1 | 0 | 1     |
| 1 | 1 | 0     |

For each bit position `i`:  `out_i = a_i ⊕ b_i = (a_i + b_i) mod 2`

**Worked example:**

```
  1100_1010   (0xCA)
^ 0110_0110   (0x66)
-----------
  1010_1100   (0xAC = 172)
```

```rust
let r: u8 = 0b1100_1010 ^ 0b0110_0110; // 0b1010_1100 = 172
```

**Key properties:**

- `a ^ a == 0` — anything XOR'd with itself cancels.
- `a ^ 0 == a` — XOR with 0 is identity.
- `a ^ b ^ b == a` — XOR is its own inverse (useful for toggling and cheap encryption).

**Primary use — toggling bits** (flip selected bits, keep the rest):

```rust
let x: u8 = 0b0000_1111;
let toggled = x ^ 0b0000_0101; // 0b0000_1010 — bits 0 and 2 flipped
```

---

## 4. NOT `!`  (bitwise complement)

**Per-bit rule:** flip every bit. `0→1`, `1→0`. This is a *unary* operator.

For each bit position `i`:  `out_i = 1 − a_i`

**Worked example:**

```
! 1100_1010   (0xCA = 202)
-----------
  0011_0101   (0x35 = 53)
```

```rust
let r: u8 = !0b1100_1010; // 0b0011_0101 = 53
```

> **Width matters.** `!x` flips *all* bits of the type, so the result depends on the type's bit width. `!0u8 == 255`, but `!0u32 == 4_294_967_295`. In Rust, `!` is also boolean NOT (`!true == false`); the compiler picks the meaning from the operand's type.

For unsigned integers, `!x == MAX − x`. For two's-complement signed integers, `!x == −x − 1`.

**Primary use — clearing bits** (build an inverted mask, then AND):

```rust
let byte: u8 = 0b1011_0110;
let cleared = byte & !0b0001_0000; // clear bit 4 -> 0b1010_0110
```

---

## 5. Left shift `<<`

**Rule:** slide all bits left by `n` positions. Vacated low bits fill with `0`. Bits shifted past the MSB are **discarded**.

Numerically (when no bits fall off the top):  `x << n  ==  x · 2^n`

**Worked example:**

```
  0001_0110   (22) << 2
-----------
  0101_1000   (88)        // 22 × 2² = 88
```

```rust
let r: u8 = 0b0001_0110 << 2; // 0b0101_1000 = 88
```

**Overflow example** (top bits drop off in a `u8`):

```
  1100_0000  (192) << 1
-----------
  1000_0000  (128)        // the leading 1 is lost
```

---

## 6. Right shift `>>`

**Rule:** slide all bits right by `n`. Bits shifted past the LSB are discarded. **What fills the vacated high bits depends on signedness:**

- **Unsigned types** → *logical* shift: high bits fill with `0`.
- **Signed types** → *arithmetic* shift: high bits fill with the **sign bit** (preserves sign).

Numerically:  `x >> n  ==  ⌊x / 2^n⌋`  (floor division by a power of two).

**Unsigned (logical):**

```
  1011_0100  (180 as u8) >> 2
-----------
  0010_1101  (45)        // 180 ÷ 4 = 45, zeros fill in
```

```rust
let r: u8 = 0b1011_0100 >> 2; // 0b0010_1101 = 45
```

**Signed (arithmetic) — sign bit is replicated:**

```
  1011_0100  (-76 as i8) >> 2
-----------
  1110_1101  (-19)       // top bit (1) copied in, sign preserved
```

```rust
let r: i8 = (-76i8) >> 2; // -19, not 45
```

> This split is why emulator and driver code is careful about `u8`/`i8`/`u32` choices: the *same* `>>` token does two different things.

---

## 7. Compound assignment operators

Each operator has an in-place form that reads, applies, and writes back to the left operand.

| Operator | Equivalent to     |
|----------|-------------------|
| `a &= b` | `a = a & b`       |
| `a \|= b`| `a = a \| b`      |
| `a ^= b` | `a = a ^ b`       |
| `a <<= n`| `a = a << n`      |
| `a >>= n`| `a = a >> n`      |

```rust
let mut flags: u8 = 0b0000_0001;
flags |= 0b0001_0000;  // set bit 4   -> 0b0001_0001
flags &= !0b0000_0001; // clear bit 0 -> 0b0001_0000
flags ^= 0b0001_0000;  // toggle bit 4-> 0b0000_0000
```

---

## 8. Rotate methods (not operators)

Rust has **no rotate operator**; rotation is provided as methods. Unlike shift, rotation wraps bits around instead of discarding them — nothing is lost.

- `x.rotate_left(n)` — bits shifted off the **top** re-enter at the **bottom**.
- `x.rotate_right(n)` — bits shifted off the **bottom** re-enter at the **top**.

**rotate_left example:**

```
  1100_0001  (0xC1).rotate_left(1)
-----------
  1000_0011  (0x83)      // MSB wrapped around to LSB
```

```rust
let r: u8 = 0b1100_0001u8.rotate_left(1); // 0b1000_0011
```

> These map directly onto CPU rotate instructions (e.g. SM83 `RLC`/`RRC`). Note a subtle difference: `rotate_left`/`rotate_right` are pure 8-bit rotations, whereas `RL`/`RR` rotate *through the carry flag* — a 9-bit rotation. For carry-through behavior you typically implement it manually with shifts, the carry bit, and `|`.

---

## 9. Safe shifting (avoiding overflow panics)

In debug builds, shifting by an amount **≥ the type's bit width** panics; in release builds the shift amount is masked, giving surprising results. Use the explicit methods when the count may be large or untrusted:

- `x.wrapping_shl(n)` / `x.wrapping_shr(n)` — count taken `mod bit_width`, never panics.
- `x.checked_shl(n)` / `x.checked_shr(n)` — returns `Option`, `None` if `n ≥ width`.
- `x.overflowing_shl(n)` — returns `(value, overflowed_bool)`.

```rust
let safe = 1u8.checked_shl(8); // None — 8 is out of range for u8
let wrap = 1u8.wrapping_shl(8); // 1 — 8 mod 8 = 0, no shift
```

---

## 10. Common bit-manipulation patterns

| Goal                  | Expression                  |
|-----------------------|-----------------------------|
| Test bit `n`          | `(x >> n) & 1 == 1`         |
| Set bit `n`           | `x \| (1 << n)`             |
| Clear bit `n`         | `x & !(1 << n)`             |
| Toggle bit `n`        | `x ^ (1 << n)`              |
| Isolate low `k` bits  | `x & ((1 << k) - 1)`        |
| Mask off bit `n`      | `x & !(1 << n)`             |
| Extract field `[hi:lo]` | `(x >> lo) & ((1 << (hi - lo + 1)) - 1)` |

```rust
fn test_bit(x: u8, n: u8) -> bool { (x >> n) & 1 == 1 }
fn set_bit(x: u8, n: u8)  -> u8   { x | (1 << n) }
fn clear_bit(x: u8, n: u8) -> u8  { x & !(1 << n) }
fn toggle_bit(x: u8, n: u8) -> u8 { x ^ (1 << n) }
```

---

## Quick summary

| Operator | Name        | Bit rule                          | Numeric effect (unsigned) |
|----------|-------------|-----------------------------------|---------------------------|
| `&`      | AND         | 1 if both bits 1                  | mask / keep bits          |
| `\|`     | OR          | 1 if either bit 1                 | set bits                  |
| `^`      | XOR         | 1 if bits differ                  | toggle bits               |
| `!`      | NOT         | flip every bit                    | `MAX − x`                 |
| `<<`     | left shift  | slide left, zero-fill low         | `× 2^n`                   |
| `>>`     | right shift | slide right (zero/sign-fill high) | `÷ 2^n` (floor)           |
