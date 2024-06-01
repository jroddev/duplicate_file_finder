## Duplicate File Finder
Small utility to recursively scan a directory tree to find duplicate files.
Duplication is based purely on the sha256 hash of the file content.

### Progress Tracking
The tool uses [indicatif](https://github.com/console-rs/indicatif/tree/main) to show progress.
```
 [ elapsed] [-------------visual percentage----------] processed/total (estimated completion time)
 [00:02:36] [##################>---------------------] 9382/20230 (6m)
```

### Output Format
The output format is as below. Entries are sorted in descending order by count.
```
Hash: 77e2a8022e81c4b5b2d836b3905ce9d771264jfhc8ada84e11b5bfbeb35de35e
First Instance: /Storage/Pictures/2022/2400754484615.jpg
Count: 9
Size: 32693 bytes

Hash: 07e3c10e07c10af958ea77278da3ad838db3486ef36e8c9a40c34848d686ad07
First Instance: /Storage/Pictures/2019/1600303418904.jpg
Count: 8
Size: 72435 bytes
```

### Memory Usage
While hashing a file its entire contents needs to be pulled into memory.
The application also processes files in parallel using [rayon](https://github.com/rayon-rs/rayon) `par_iter` which means that multiple complete files can be in-memory at the same time.
If you are processing very large files then you may want to:
- Remove or limit the concurrency of Rayon.
- Change the hashing algorithm so that it doesn't load the entire file at once.

### Build and Run
This application requires the Rust toolchain to build.
`cargo run --release <file path> | tee output.txt`
