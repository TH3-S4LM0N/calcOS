# calcOS
a calculator operating system for a honors hs class \
due may 1st

### random info so i dont forget
needs: \
    - nightly \
    - llvm-tools-preview \
rust-analyzer doesnt understand and isnt reliable here

### todo
- order of operations (pain)
- functions
- make a command line

### credit
huge thanks to the people at https://github.com/rust-osdev for making rust os stuff possible

### presentation
- explain vga
- explain square rooting and cpu instructions
##### why sqrt doesnt exist
square rooting is logic which is something a computer can't do.
if i want to square root 36, i just know 6 * 6 = 36, but there's no mathematical equation for square rooting so therefore a computer cant do it

### functions
- `Q_rsqrt` Quake fast inverse square root

- `triangles::sum` determine if 3 values can make up triangle angles
- `triangles::ext_sum` determine if 3 values can make up triangle exterior angles
- `triangle::py_thrm` the pythagorean theorem