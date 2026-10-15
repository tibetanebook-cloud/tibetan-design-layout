$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
$dir = Join-Path $root '.build-tools/tibetan-font'
New-Item -ItemType Directory -Force $dir | Out-Null
$font = Join-Path $dir 'NotoSerifTibetan.ttf'
$license = Join-Path $dir 'OFL.txt'
$base = 'https://raw.githubusercontent.com/google/fonts/main/ofl/notoseriftibetan'
if (-not (Test-Path -LiteralPath $font)) {
    Invoke-WebRequest "$base/NotoSerifTibetan%5Bwght%5D.ttf" -OutFile $font
}
if (-not (Test-Path -LiteralPath $license)) {
    Invoke-WebRequest "$base/OFL.txt" -OutFile $license
}
if ((Get-FileHash -LiteralPath $font -Algorithm SHA256).Hash -ne '060EC022B04C306DE3F58D051FB0E1CF81A5B610C5910FBFC43BBA154C057CDA') {
    throw 'Noto Tibetan font checksum mismatch. Review the upstream update before building.'
}
if ((Get-FileHash -LiteralPath $license -Algorithm SHA256).Hash -ne '0077789FC3CB20F358278BA4C6437EB14ADE97D4D3E3281EAA42853EFCD9DC8B') {
    throw 'Noto Tibetan font license checksum mismatch.'
}
$env:TIBETAN_UI_FONT = $font
Write-Output "Tibetan UI font prepared: $font"

$craftDir = Join-Path $root '.build-tools/layout-fonts'
New-Item -ItemType Directory -Force (Join-Path $craftDir 'fonts/tibetan') | Out-Null
Copy-Item -LiteralPath $font -Destination (Join-Path $craftDir 'fonts/tibetan/NotoSerifTibetan.ttf')
Copy-Item -LiteralPath $license -Destination (Join-Path $craftDir 'fonts/tibetan/OFL.txt')
Set-Content -LiteralPath (Join-Path $craftDir 'fonts/manifest.txt') -Value 'Noto Serif Tibetan | Regular | fonts/tibetan/NotoSerifTibetan.ttf | Tibt' -Encoding utf8NoBOM
$env:CRAFT_FONTS_DIR = $craftDir
$env:CRAFT_FONTS_REQUIRED = '1'
