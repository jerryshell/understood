# undeRStood

_undeRStood_ is an image classification tool written in Rust 🦀

It uses images in the `img_sample` folder as samples, finds similar images in the `img_source` folder, and then moves the similar images to the `img_result` folder

## How to use

```bash
cargo install --git https://github.com/jerryshell/understood
```

```
Usage: understood [OPTIONS]

Options:
  -s, --img-sample-path <IMG_SAMPLE_PATH>
          [default: img_sample]
  -i, --img-source-path <IMG_SOURCE_PATH>
          [default: img_source]
  -o, --img-result-path <IMG_RESULT_PATH>
          [default: img_result]
  -t, --hamming-threshold <HAMMING_THRESHOLD>
          [default: 10]
      --clean-flag
          If the --clean-flag is explicitly specified, incorrectly formatted images will be automatically deleted
  -h, --help
          Print help
  -V, --version
          Print version
```

## References

- [jerryshell/similars](https://github.com/jerryshell/similars)

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
