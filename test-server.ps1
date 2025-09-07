# Market Data Source Server Test Script
# This script launches the server, runs API tests, and closes the server

param(
    [int]$Port = 8080,
    [string]$Host = "localhost",
    [int]$StartupDelay = 3,
    [int]$TestDelay = 1
)

$ErrorActionPreference = "Stop"

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "Market Data Source Server Test Suite" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Function to test API endpoints
function Test-Endpoint {
    param(
        [string]$Method,
        [string]$Url,
        [string]$Body = $null,
        [string]$Description
    )
    
    Write-Host "Testing: $Description" -ForegroundColor Yellow
    Write-Host "  URL: $Url" -ForegroundColor Gray
    
    try {
        $headers = @{
            "Content-Type" = "application/json"
            "Accept" = "application/json"
        }
        
        if ($Method -eq "GET") {
            $response = Invoke-RestMethod -Uri $Url -Method $Method -Headers $headers -TimeoutSec 5
        } else {
            $response = Invoke-RestMethod -Uri $Url -Method $Method -Headers $headers -Body $Body -TimeoutSec 5
        }
        
        Write-Host "  ✓ Success" -ForegroundColor Green
        return $response
    }
    catch {
        Write-Host "  ✗ Failed: $_" -ForegroundColor Red
        return $null
    }
}

# Build the server first
Write-Host "Building server..." -ForegroundColor Cyan
$buildProcess = Start-Process -FilePath "cargo" -ArgumentList "build","--bin","market-data-server","--features","api-server" -NoNewWindow -PassThru -Wait

if ($buildProcess.ExitCode -ne 0) {
    Write-Host "Failed to build server. Exit code: $($buildProcess.ExitCode)" -ForegroundColor Red
    exit 1
}
Write-Host "Build successful!" -ForegroundColor Green
Write-Host ""

# Start the server in background
Write-Host "Starting server on ${Host}:${Port}..." -ForegroundColor Cyan
$serverProcess = Start-Process -FilePath "cargo" -ArgumentList "run","--bin","market-data-server","--features","api-server","--","--port",$Port,"--host",$Host -NoNewWindow -PassThru

# Wait for server to start
Write-Host "Waiting $StartupDelay seconds for server to start..." -ForegroundColor Gray
Start-Sleep -Seconds $StartupDelay

# Check if server process is still running
if ($serverProcess.HasExited) {
    Write-Host "Server failed to start!" -ForegroundColor Red
    exit 1
}

Write-Host "Server started with PID: $($serverProcess.Id)" -ForegroundColor Green
Write-Host ""

$baseUrl = "http://${Host}:${Port}"
$apiUrl = "${baseUrl}/api/v1"
$testsPassed = 0
$testsFailed = 0

try {
    Write-Host "Running API Tests" -ForegroundColor Cyan
    Write-Host "=================" -ForegroundColor Cyan
    Write-Host ""
    
    # Test 1: Health Check
    $health = Test-Endpoint -Method "GET" -Url "$baseUrl/health" -Description "Health Check"
    if ($health) {
        Write-Host "  Status: $($health.status)" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 2: API Discovery
    $discovery = Test-Endpoint -Method "GET" -Url "$baseUrl/api" -Description "API Discovery"
    if ($discovery) {
        Write-Host "  Version: $($discovery.version)" -ForegroundColor Gray
        Write-Host "  WebSocket: $($discovery.websocket.url)" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 3: Get Capabilities
    $capabilities = Test-Endpoint -Method "GET" -Url "$apiUrl/capabilities" -Description "Get Capabilities"
    if ($capabilities) {
        Write-Host "  Features: $($capabilities.features.Count) available" -ForegroundColor Gray
        Write-Host "  Export Formats: $($capabilities.export_formats -join ', ')" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 4: List Symbols (should be empty initially)
    $symbols = Test-Endpoint -Method "GET" -Url "$apiUrl/symbols" -Description "List Symbols"
    if ($symbols) {
        Write-Host "  Symbols found: $($symbols.count)" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 5: Create Symbol
    $createBody = @{
        symbol = "BTCUSD"
        config = @{
            initial_price = "50000.0"
            volatility = "0.05"
            drift = "0.0002"
        }
    } | ConvertTo-Json
    
    $newSymbol = Test-Endpoint -Method "POST" -Url "$apiUrl/symbols" -Body $createBody -Description "Create Symbol (BTCUSD)"
    if ($newSymbol) {
        Write-Host "  Symbol: $($newSymbol.symbol)" -ForegroundColor Gray
        Write-Host "  Active: $($newSymbol.active)" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 6: Generate Data
    $generateBody = @{
        count = 5
        format = "ohlc"
    } | ConvertTo-Json
    
    $data = Test-Endpoint -Method "POST" -Url "$apiUrl/generate/BTCUSD" -Body $generateBody -Description "Generate OHLC Data"
    if ($data) {
        Write-Host "  Generated: $($data.metadata.count) candles" -ForegroundColor Gray
        if ($data.data -and $data.data.Count -gt 0) {
            $firstCandle = $data.data[0]
            Write-Host "  First OHLC: O=$($firstCandle.open) H=$($firstCandle.high) L=$($firstCandle.low) C=$($firstCandle.close)" -ForegroundColor Gray
        }
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 7: Get Historical Data
    $historical = Test-Endpoint -Method "GET" -Url "$apiUrl/historical/BTCUSD?limit=3" -Description "Get Historical Data"
    if ($historical) {
        Write-Host "  Retrieved: $($historical.count) candles" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 8: List Algorithms
    $algorithms = Test-Endpoint -Method "GET" -Url "$apiUrl/algorithms" -Description "List Algorithms"
    if ($algorithms) {
        Write-Host "  Available: $($algorithms.algorithms.Count) algorithm(s)" -ForegroundColor Gray
        Write-Host "  Planned: $($algorithms.planned -join ', ')" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 9: List Presets
    $presets = Test-Endpoint -Method "GET" -Url "$apiUrl/presets" -Description "List Presets"
    if ($presets) {
        $presetNames = $presets.presets | Get-Member -MemberType NoteProperty | Select-Object -ExpandProperty Name
        Write-Host "  Available presets: $($presetNames -join ', ')" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 10: Export as JSON
    $exportJson = Test-Endpoint -Method "GET" -Url "$apiUrl/export/BTCUSD/json" -Description "Export as JSON"
    if ($exportJson) {
        Write-Host "  Data points: $($exportJson.data.Count)" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    
    # Test 11: WebSocket Connection (just test if endpoint responds)
    Write-Host "Testing: WebSocket Endpoint" -ForegroundColor Yellow
    Write-Host "  URL: ws://${Host}:${Port}/ws" -ForegroundColor Gray
    try {
        # We can't easily test WebSocket in PowerShell, so just check if the HTTP upgrade would work
        $wsHeaders = @{
            "Upgrade" = "websocket"
            "Connection" = "Upgrade"
            "Sec-WebSocket-Key" = [Convert]::ToBase64String((1..16 | ForEach-Object { Get-Random -Maximum 256 } | ForEach-Object { [byte]$_ }))
            "Sec-WebSocket-Version" = "13"
        }
        
        # This will fail but if it returns 101 or 426, the endpoint exists
        try {
            $wsResponse = Invoke-WebRequest -Uri "$baseUrl/ws" -Headers $wsHeaders -Method GET -TimeoutSec 2
        }
        catch {
            if ($_.Exception.Response.StatusCode -eq 426 -or $_.Exception.Response.StatusCode -eq 101) {
                Write-Host "  ✓ WebSocket endpoint available" -ForegroundColor Green
                $testsPassed++
            } else {
                throw
            }
        }
    }
    catch {
        Write-Host "  ✗ WebSocket check failed" -ForegroundColor Red
        $testsFailed++
    }
    Write-Host ""
    
    # Test 12: Delete Symbol
    $delete = Test-Endpoint -Method "DELETE" -Url "$apiUrl/symbols/BTCUSD" -Description "Delete Symbol"
    if ($delete -or $LASTEXITCODE -eq 0) {
        Write-Host "  Symbol deleted successfully" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    
}
finally {
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "Test Results" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "  Passed: $testsPassed" -ForegroundColor Green
    Write-Host "  Failed: $testsFailed" -ForegroundColor $(if ($testsFailed -eq 0) { "Green" } else { "Red" })
    Write-Host ""
    
    # Stop the server using control endpoint
    Write-Host "Stopping server using control API..." -ForegroundColor Yellow
    
    try {
        $shutdownBody = @{
            command = "shutdown"
            params = @{
                delay_ms = 500
            }
        } | ConvertTo-Json
        
        $shutdownResponse = Invoke-RestMethod -Uri "$baseUrl/control" -Method POST -Headers @{"Content-Type"="application/json"} -Body $shutdownBody -TimeoutSec 5
        Write-Host "  Server response: $($shutdownResponse.message)" -ForegroundColor Gray
        
        # Wait for server to shutdown
        Start-Sleep -Seconds 1
        
        # Check if process has exited
        if ($serverProcess.HasExited) {
            Write-Host "Server stopped successfully" -ForegroundColor Green
        } else {
           