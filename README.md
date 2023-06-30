# rusty-grep
a CLI grep implemented in Rust
# usage
```{bash}
cargo run -- <mode> <query> <target> [depth] [num_threads] [--invert]
```
The program has two modes, content and file search.
<br>Example 1:

```{bash}
cargo run -- content How ./poem.txt 
```
result:
```{bash}
searching for: How
in: ./poem.txt
Line 6: How dreary to be somebody!
Line 7: How public, like a frog 
```

Example 2:
```{bash}
cargo run -- file ro ../ 2
```
result:
```{bash}
searching for: ro
in: ../
../round-robin
../round-robin
../fcfs/processes.txt
../fcfs/processes.txt
../round-robin/processes.txt
../round-robin/processes.txt
```
