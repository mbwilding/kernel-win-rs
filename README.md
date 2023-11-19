# Kernel Driver in Rust

## Environment Variables

| Key           | Value                                                       |
|---------------|-------------------------------------------------------------|
| WIN_SDK_TOOLS | C:\Program Files (x86)\Windows Kits\10\bin\10.0.22621.0\x64 |

## Requirements

### SDK and WDK

[Install both the SDK and WDK](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)

#### Install Cargo Make

```powershell
cargo install cargo-make
```

### Build

```powershell
cargo make deploy
```

## Test Signing

```powershell
bcdedit -set TESTSIGNING ON
bcdedit /debug on
```

## Driver

### Create

```powershell
sc create kernel_win_rs binPath="%FullPath%\kernel_win_rs.sys" type=kernel start=demand
```

### Start

```powershell
sc start kernel_win_rs
```

### Remove

```powershell
sc delete kernel_win_rs
```
