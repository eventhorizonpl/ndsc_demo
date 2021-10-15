N dimensional symmetric cipher
==============================

How to run?
-----------

2D encryption & decryption
--------------------------

```
$ cargo run -- -p test -E -d 2 -i test.txt -o test.2dsc
```

```
$ cargo run -- -p test -D -d 2 -i test.2dsc -o test.txte
```

3D encryption & decryption
--------------------------

```
$ cargo run -- -p test -E -d 3 -i test.txt -o test.3dsc
```

```
$ cargo run -- -p test -D -d 3 -i test.3dsc -o test.txte
```

4D encryption & decryption
--------------------------

```
$ cargo run -- -p test -E -d 4 -i test.txt -o test.4dsc
```

```
$ cargo run -- -p test -D -d 4 -i test.4dsc -o test.txte
```
