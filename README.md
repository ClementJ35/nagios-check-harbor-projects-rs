# Readme

This check aims to use Harbor exposed metrics to check projects quota.

## Usage

```sh
$ nagios-check-harbor-projects-rs --url http://harbor-url:9090/metrics -w 80 -c 90
QUOTA OK: All quotas are OK | kubernetes-proxy=0%;80;90 quay-proxy=67%;80;90 amazon-proxy=0%;80;90 github-proxy=46%;80;90
```

## Prerequisite

See [cargo.toml] for building dependencies

## Building

```sh
$ cargo build --release
   Compiling nagios-check-harbor-projects-rs v0.1.0 (nagios-check-harbor-projects-rs)
    Finished `release` profile [optimized] target(s) in 0.84s
```

### License

See [LICENSE]

### Contributing

Contributions all welcome. Merges requests are welcome. Just explain what and why you are trying to achieve.
