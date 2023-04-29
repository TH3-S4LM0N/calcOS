# `calc_os`
An `x86_64` calculator operating system.
## Features
- Simple math with all 5 operators:
- `+` Addition
- `-` Subtraction
- `*` Multiplication
- `/` Division
- `^` Powers
- Algorithms:
- `sqrt` Square Root
- `Q_rsqrt` *Quake* Fast Inverse Square Root
## How making os' works
### VGA Buffer
How you display text. \
A table of width and height with each entry holding a character.
So a 5x2 VGA Buffer displaying "Hello World" would be:

| H | e | l | l | o |
|---|---|---|---|---|
| **W** | **o** | **r** | **l** | **d** |

and look like:
```
Hello
World
```
I'm using a 80x25 buffer so I can display a full long paragraph about.
### User Input
The cpu or central processing unit has interrupts which are things that pause the cpu and have to be handled before processing can continue.
Interrupt descriptor table
- div by 0 interrupt
- double and triple fault
- hardware intterupts

idt is a table of functions that are called when the matching interrupt occurs. we look at args to see the user pressed a key, and then take action corresponding to what key they pressed.

So we can take user input and print so we can do math.
### Math
This is where things get complicated. Explain stdlibs and lack of allocator. \
Now i could do code/str to int, but it has operators

### Square rooting
Square rooting gave me a lot of pain, in math a square root is `x ^ 0.5`. \
Thats not how computers work, and its inneficient on large scales, so a square root is code snippets/sqrt
#### Q_rsqrt
inverse square root: `1/sqrt(x)`. this is horribly inneficient. \
quake inverse square root code snippets/Q_rsqrt