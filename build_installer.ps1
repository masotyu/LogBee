# 1. Rust バイナリをリリースモードでビルド
Write-Host "--- Building Rust binary ---" -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Cargo build failed!" -ForegroundColor Red
    exit 1
}

# 2. Inno Setup でインストーラーを作成
Write-Host "--- Compiling Installer with Inno Setup ---" -ForegroundColor Cyan
# パスが通っていない場合はフルパスで記述: & "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" setup.iss
iscc installer\windows\setup.iss

if ($LASTEXITCODE -ne 0) {
    Write-Host "Inno Setup compilation failed!" -ForegroundColor Red
    exit 1
}

Write-Host "--- Success! Installer is ready in target\installer ---" -ForegroundColor Green