# Grammar

### General

All values are specified as numerical values between 0 and 255, except for the octave setting parameter.

### Note

```
a/b/c/d/e/f/g/r/x [+/-/=] [1/2/3/...] [.]
```

Example:

  * `r`
  * `b4`
  * `c-`
  * `d+16.`

### Octave

* `>`: Octave up.
* `<`: Octave down.
* `on`: Set octave `n` which must be between 1 and 8.

The default octave is `4`, which corresponds to the note `a` at 440Hz.

### Long

Default long is `4`.

### Volume

* `vn`: Set volume `n`.

The default volume is `153`, which represents 60% of the maximum volume.
