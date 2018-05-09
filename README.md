Portable replacement for

``` sh
while true; do
  xclip -o
  sleep 0.1
done
```

with very basic deduplication written in Rust: It queries the clipboard every
100ms and prints the current content followed by a newline, unless another copy
of the exact same line exists in the five previous lines.
