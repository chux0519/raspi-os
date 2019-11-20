.text //code section
.globl _start
_start:
1:  wfe
    b       1b
