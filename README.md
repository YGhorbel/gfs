![Guepard](/resources/guepard-cover.png)

<div align="center">
    <h1>Git For database Systems</h1>
    <br />  
    <p align="center">
    <a href="https://youtu.be/WlOkLnoY2h8?si=hb6-7kLhlOvVL1u6">
        <img src="https://img.shields.io/badge/Watch-YouTube-%23ffcb51?logo=youtube&logoColor=black" alt="Watch on YouTube" />
    </a>
    <a href="https://discord.gg/nCXAsUd3hm">
        <img src="https://img.shields.io/badge/Join-Community-%23ffcb51?logo=discord&logoColor=black" alt="Join our Community" />
    </a>
    <a href="https://github.com/Guepard-Corp/gfs/actions/workflows/main.yml" target="_blank">
        <img src="https://img.shields.io/github/actions/workflow/status/Guepard-Corp/gfs/main.yml?branch=main" alt="Build">
    </a>
    <a href="https://github.com/Guepard-Corp/gfs/blob/main/LICENCE" target="_blank">
        <img src="https://img.shields.io/badge/license-ELv2-blue.svg" alt="License" />
    </a>s
    </a>
    <a href="https://github.com/Guepard-Corp/gfs/pulls" target="_blank">
        <img src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" alt="PRs Welcome" />
    </a>
    </p>
</div>

## Important Notice

🚧 This project is under active development and not yet suitable for production use. Expect breaking changes, incomplete features, and evolving APIs.

## Running

- Init the repository 

```bash
cargo run --bin gfs init --database-provider postgres --database-version 17
```

- Commit your database branch

```bash
cargo run --bin gfs commit -m "v1"
```

- Check history

```bash
cargo run --bin gfs log
```

## Testing

### Run all tests

```bash
cargo test
```
