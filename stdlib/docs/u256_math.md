
## std::math::u256
| Instruction | Description | 
| ----------- | ------------- |
| add_unsafe |  |
| sub_unsafe |  |
| and |  |
| or |  |
| xor |  |
| iszero_unsafe |  |
| eq_unsafe |  |
| mul_unsafe.6 |  Performs addition of two unsigned 256 bit integers discarding the overflow.<br /> The input values are assumed to be represented using 32 bit limbs, but this is not checked.<br /> Stack transition looks as follows:<br /> [b7, b6, b5, b4, b3, b2, b1, b0, a7, a6, a5, a4, a3, a2, a1, a0, ...] -> [c7, c6, c5, c4, c3, c2, c1, c0, ...]<br /> where c = (a * b) % 2^256, and a0, b0, and c0 are least significant 32-bit limbs of a, b, and c respectively. |
