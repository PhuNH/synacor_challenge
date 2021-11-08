use std::convert::TryInto;
use std::fs;

use synacor_vm::SynacorVm;

mod synacor_vm;

pub fn read_input_u16(path: &str) -> Vec<u16> {
    let bin_input = fs::read(path).unwrap();
    bin_input.chunks(2)
        .map(|c| u16::from_le_bytes(c.try_into().unwrap()))
        .collect()
}

fn main() {
    let bin_input = read_input_u16("input/challenge.bin");
    let mut vm = SynacorVm::new(bin_input);
    let prepared = r#"doorway
north
north
bridge
continue
down
east
take empty lantern
west
west
passage
ladder
west
south
north
take can
use can
use lantern
west
ladder
darkness
continue
west
west
west
west
north
take red coin
north
east
take concave coin
down
take corroded coin
up
west
west
take blue coin
up
take shiny coin
down
east
use blue coin
use red coin
use shiny coin
use concave coin
use corroded coin
north
take teleporter
use teleporter
take business card
"#;
    let second_prepared = r#"north
north
north
north
north
north
north
north
north
take orb
north
east
east
north
west
south
east
east
west
north
north
east
vault
take mirror
use mirror
"#;
    vm.run(prepared, second_prepared);
}

/* python z3
from z3 import *
slots = [Int("s_%s" % i) for i in range(5)]
values = [Or(And(slots[i] < 10, slots[i] > 1, slots[i] % 2 == 1), slots[i] == 2) for i in range(5)]
distinction = Distinct(slots)
solve(values + [distinction, slots[0] + slots[1] * slots[2] ** 2 + slots[3] ** 3 - slots[4] == 399])
[s_0 = 9, s_1 = 2, s_2 = 5, s_3 = 7, s_4 = 3]
*/

/* The strange book
The cover of this book subtly swirls with colors.  It is titled "A Brief Introduction to Interdimensional Physics".  It reads:

Recent advances in interdimensional physics have produced fascinating
predictions about the fundamentals of our universe!  For example,
interdimensional physics seems to predict that the universe is, at its root, a
purely mathematical construct, and that all events are caused by the
interactions between eight pockets of energy called "registers".
Furthermore, it seems that while the lower registers primarily control mundane
things like sound and light, the highest register (the so-called "eighth
register") is used to control interdimensional events such as teleportation.

A hypothetical such teleportation device would need to have have exactly two
destinations.  One destination would be used when the eighth register is at its
minimum energy level - this would be the default operation assuming the user
has no way to control the eighth register.  In this situation, the teleporter
should send the user to a preconfigured safe location as a default.

The second destination, however, is predicted to require a very specific
energy level in the eighth register.  The teleporter must take great care to
confirm that this energy level is exactly correct before teleporting its user!
If it is even slightly off, the user would (probably) arrive at the correct
location, but would briefly experience anomalies in the fabric of reality
itself - this is, of course, not recommended.  Any teleporter would need to test
the energy level in the eighth register and abort teleportation if it is not
exactly correct.

This required precision implies that the confirmation mechanism would be very
computationally expensive.  While this would likely not be an issue for large-
scale teleporters, a hypothetical hand-held teleporter would take billions of
years to compute the result and confirm that the eighth register is correct.

If you find yourself trapped in an alternate dimension with nothing but a
hand-held teleporter, you will need to extract the confirmation algorithm,
reimplement it on more powerful hardware, and optimize it.  This should, at the
very least, allow you to determine the value of the eighth register which would
have been accepted by the teleporter's confirmation mechanism.

Then, set the eighth register to this value, activate the teleporter, and
bypass the confirmation mechanism.  If the eighth register is set correctly, no
anomalies should be experienced, but beware - if it is set incorrectly, the
now-bypassed confirmation mechanism will not protect you!

Of course, since teleportation is impossible, this is all totally ridiculous.
*/

/* confirmation process
5483: set r0 4
5486: set r1 1
5489: call 6027             af()
                            # confirmation done
5491: eq r1 r0 6            r1 = 1 if r0 == 6 else 0
5495: jf r1 5579            if r1 != 0:
5498: push r0                 ...
5500: push r1
5502: push r2
5504: set r0 29014
5507: set r1 1531
5510: add r2 302 1601
5514: call 1458
                            def af:
6027: jt r0 6035              if r0 == 0:
6030: add r0 r1 1               r0 = r1 + 1
6034: ret                       return
6035: jt r1 6048              if r1 == 0:
6038: add r0 r0 32767           r0 -= 1
6042: set r1 r7                 r1 = r7
6045: call 6027                 af()
6048: push r0                 push r0
6050: add r1 r1 32767         r1 -= 1
6054: call 6027               af()
6056: set r1 r0               r1 = r0
6059: pop r0                  pop r0
6061: add r0 r0 32767         r0 -= 1
6065: call 6027               af()
6067: ret                     return
*/

/* Ackermann function at ip = 6027
af(0, n) = n + 1
af(m + 1, 0) = af(m, x) with x = r7
af(m + 1, n + 1) = af(m, af(m + 1, n))

af(4, 1) = af(3, af(4, 0)) = af(3, af3x)
af(4, 0) = af(3, x) = af3x
af(3, n) = af(2, af(3, n-1)) = (x+1) * af(3, n-1) + 2x+1 = (x+1) * ((x+1) * af(3, n-2) + 2x+1) + 2x+1
         = (x+1)^2 * af(3, n-2) + (x+1+1)(2x+1) = (x+1)^2 * ((x+1) * af(3, n-3) + 2x+1) + (x+1+1)(2x+1)
         = (x+1)^3 * af(3, n-3) + ((x+1)^2 + x+1 + 1)(2x+1) = ...
         = (x+1)^n * af(3, 0) + (2x+1)((x+1)^(n-1) + ... + x+1 + 1)
         = (x+1)^(n+2) + x * (x+1)^n + ((x+1)^(n-1) + ... + x+1 + 1)(x+1 + x)
         = (x+1)^(n+2) + x * ((x+1)^n + (x+1)^(n-1) + ... + x+1 + 1)
                       +      (x+1)^n + (x+1)^(n-1) + ... + x+1 + 1 - 1
         = (x+1)^(n+2) + (x+1)^(n+1) + (x+1)^n + ... + (x+1)^2 + (x+1) + 1 - 2
         = (x+1)^(n+2) + (x+1)^(n+1) + (x+1)^n + ... + (x+1)^2 + x // use this
    with x != 0:
         = ((x+1)^(n+3) - 1) / x - 2 // this cannot be used due to lost accuracy during type conversion
    with x == 1:
         = 2^(n+3) - 3
    with x == 0:
         = 1 + n
af(3, 0) = af(2, x) = (x+1)^2 + x
af(2, n) = af(1, af(2, n-1)) = x+1 + af(2, n-1) = 2(x+1) + af(2, n-2) = ... = n(x+1) + af(2, 0)
         = n(x+1) + x+1 + x = (n+1)(x+1) + x
af(2, 0) = af(1, x) = x+1 + x
af(1, n) = af(0, af(1, n-1)) = af(1, n-1) + 1 = af(1, n-2) + 2 = ... = af(1, 0) + n = x+1 + n
af(1, 0) = af(0, x) = x+1
*/

/* room grid
                       30
|   * y | 8 y |  - r |  1 ry |
|   4 g | *   | 11   |  *  y |
|   + g | 4   |  -   | 18  y |
| !22   | - r |  9 r |  *  y |

*/

// SPOILER AHEAD







/*
oHVlEiuRDDqk - from arch-spec
SaFPTyYYPxtc - welcome before self-test
xHUCoNHlSgmn - self-test completion
ycDPlkIuhlbC - tablet
frOgQzZlguet - found oil
buMeVvBwqgHJ - after using teleporter with r7 = 0
uQWfNPkCwlXV - after using teleporter with r7 = 25734
qAddloUWuIAd - in the vault, on my forehead - bAIuWUolbbAp
*/
