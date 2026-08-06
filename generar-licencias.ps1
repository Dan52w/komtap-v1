# generar-licencias.ps1
# Genera un lote de licencias permanentes y temporales para Komtap.
# Uso:
#   .\generar-licencias.ps1
#   .\generar-licencias.ps1 -CantidadPermanentes 50 -CantidadTemporales 200 -DiasTemporal 90

param(
    [int]$CantidadPermanentes = 100,
    [int]$CantidadTemporales = 100,
    [int]$DiasTemporal = 30
)

$ErrorActionPreference = "Stop"
$raiz = $PSScriptRoot
$srcTauri = Join-Path $raiz "src-tauri"

Write-Host "Compilando el generador de licencias (release)..." -ForegroundColor Cyan
Push-Location $srcTauri
cargo build --release --bin generar_licencia
Pop-Location

$exe = Join-Path $srcTauri "target\release\generar_licencia.exe"
if (-not (Test-Path $exe)) {
    throw "No se encontró el ejecutable en $exe"
}

$salida = Join-Path $raiz "licencias_generadas.csv"
"tipo,dias,codigo,fecha_generacion" | Out-File -FilePath $salida -Encoding utf8

$fecha = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

Write-Host "Generando $CantidadPermanentes licencias permanentes..." -ForegroundColor Cyan
for ($i = 1; $i -le $CantidadPermanentes; $i++) {
    $codigo = & $exe permanente
    "permanente,,$codigo,$fecha" | Out-File -FilePath $salida -Append -Encoding utf8
    Write-Progress -Activity "Permanentes" -Status "$i / $CantidadPermanentes" -PercentComplete (($i / $CantidadPermanentes) * 100)
}

Write-Host "Generando $CantidadTemporales licencias temporales ($DiasTemporal dias)..." -ForegroundColor Cyan
for ($i = 1; $i -le $CantidadTemporales; $i++) {
    $codigo = & $exe temporal $DiasTemporal
    "temporal,$DiasTemporal,$codigo,$fecha" | Out-File -FilePath $salida -Append -Encoding utf8
    Write-Progress -Activity "Temporales" -Status "$i / $CantidadTemporales" -PercentComplete (($i / $CantidadTemporales) * 100)
}

Write-Host "Listo. Se generaron $($CantidadPermanentes + $CantidadTemporales) licencias en: $salida" -ForegroundColor Green