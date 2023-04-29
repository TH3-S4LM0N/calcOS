### idt
```rust
pub struct Idt {
	table: [Fn; 25]
}
```

### vga
```rust
pub struct Buffer {
	characters: [[char; 80]; 25]
}
```
### str to int
```rust
let one = "1".parse::<i32>();
```

### sqrt
```rust
pub fn sqrt(self) -> Self {
	if self >= Self::ZERO {
		Self::from_bits((self.to_bits() + 0x3f80_0000) >> 1)
	} else {
		Self::NAN
	}
}
```
### Q_rsqrt
```rust
pub fn Q_rsqrt(number: f32) -> f32 {
	let mut i: i32;
	let (x2, mut y): (f32, f32);
	const THREEHALVES: f32 = 1.5;

	x2 = number * 0.5;
	y = number;
	i = y.to_bits() as i32; // evil floating point bit level hacking
	i = 0x5f3759df - (i >> 1); // what the heck?
	y = f32::from_bits(i as u32);
	y = y * (THREEHALVES - (x2 * y * y)); // 1st iteration
	// y = y * (THREEHALVES - (x2 * y * y)); // 2nd iteration, this can be removed

	return y;
}
```