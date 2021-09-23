# pipeviewer-byte-analyzer

generated random file to test 16kB limit of initial configuration using

```bash
dd if=/dev/urandom bs=1024 count = 128 of=myfile
```

this file was then piped into the executable using

```bash
cat myfile | target/debug/pipeviewer > myfile2
```

an identical file was generated titled myfile2, which was checked for accuracy using

```bash
diff myfile myfile2
```

the additional implementation of the dbg! macro and an environment variable were tested using

```bash
echo "a string" | cargo run
```

however, because the environment variable was not previously declared, it returned **false**

to test the assignment of the environment variable declared, the test was repeated with an explicit declaration

```bash
echo "a string" | PV_SILENT=something cargo run
```
