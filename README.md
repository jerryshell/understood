# undeRStood

undeRStood is a picture classification tool.

It uses the pictures in the `img_sample` folder as specimens to find similar pictures in the `img_source` folder, and moves the similar pictures to the `img_result` folder.

## Usage

```
USAGE:
    understood [OPTIONS]

OPTIONS:
    -h, --help
            Print help information

    -i, --img-source-path <IMG_SOURCE_PATH>
            [default: img_source]

    -n, --n-workers <N_WORKERS>
            If n_workers is 0, the number of cpu cores is automatically used [default: 0]

    -o, --img-result-path <IMG_RESULT_PATH>
            [default: img_result]

    -s, --img-sample-path <IMG_SAMPLE_PATH>
            [default: img_sample]

    -t, --hamming-threshold <HAMMING_THRESHOLD>
            [default: 16]

    -V, --version
            Print version information
```

## References

* [https://github.com/jerryshell/similars](https://github.com/jerryshell/similars)

## License

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
