### Hex encode/decode cli tool

#### 0. Usage
```text
Usage: cli-hex [OPTIONS]

Options:
  -t, --text <TEXT>  read in text mode (default)
  -f, --file <FILE>  read in file mode
  -d, --decode       decode text or file
  -l, --lower        output lower alphabet
  -h, --help         Print help
```

#### 1. Encode text

  ```bash
  cli-hex -t text_need_encode
  // 746578745F6E6565645F656E636F6465
  ```

#### 2. Decode text

```bash
cli-hex -d -t 746578745F6E6565645F6465636F6465
// text_need_decode
```

#### 3. Encode file

```bash
cli-hex -f ./file_to_encode.txt
// 746578745F746F5F656E636F6465
```

#### 4. Decode file

```bash
cli-hex -d -f ./file_to_decode.txt
// text_need_decode
```