# sbf

A [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) interpreter that lets programs perform OS syscalls via a custom instruction, hopefully portable across architectures

## Syscall Instruction

When syscall instruction (`!`) occurs with the cell pointer at cell n:

| Cell [n, n+3] | Cell n+4 | Cell n+5 | Cell [n+6, n+14] | ... |
|---|---|---|---|---|
| syscall number | argc | arg1 type | arg1 | ... |

- Syscall number takes 4 cells (32-bit integer)
- Each argument takes 8 cells (64-bit), the size of a register on 64-bit architecture
- Arg types:
  - 0 (integer)
  - 1 (pointer)
- Pointers should be given as an offset relative to the beginning of the tape. The interpreter will translate the relative address to a real pointer
- All arguments are little-endian

## Example

`test_prog` prints "hello\n" twice: first via normal Brainfuck instructions, then via a write syscall. Then, it exits via an exit syscall with exit code 67.

```
$ cargo run test_prog
hello
hello
$ echo $?
67
```