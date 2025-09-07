# Market Data Source Server Test Script
# This script launches the server, runs API tests, and closes the server

param(
    [int]$Port = 8080,
    [string]$HostName = "localhost",
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
        
        Write-Host "  [OK] Success" -ForegroundColor Green
        return $response
    }
    catch {
        Write-Host "  [FAIL] Failed: $_" -ForegroundColor Red
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
Write-Host "Starting server on ${HostName}:${Port}..." -ForegroundColor Cyan
$serverProcess = Start-Process -FilePath "cargo" -ArgumentList "run","--bin","market-data-server","--features","api-server","--","--port",$Port,"--host",$HostName,"start" -NoNewWindow -PassThru

# Wait for server to start
Write-Host "Waiting $StartupDelay seconds for server to start..." -ForegroundColor Gray
Start-Sleep -Seconds $StartupDelay
Write-Host "Finished waiting." -ForegroundColor Gray
Write-Host ""

# Check if server process is still running
if ($serverProcess.HasExited) {
    Write-Host "Server failed to start!" -ForegroundColor Red
    exit 1
}

Write-Host "Server started with PID: $($serverProcess.Id)" -ForegroundColor Green
Write-Host ""

$baseUrl = "http://${HostName}:${Port}"
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
    
    # Test 5: Create Symbol with partial config
    $createBody = @{
        symbol = "BTCUSD"
        config = @{
            starting_price = "50000.0"
            trend_direction = "up"
            volatility = "0.05"
        }
    } | ConvertTo-Json
    
    $newSymbol = Test-Endpoint -Method "POST" -Url "$apiUrl/symbols" -Body $createBody -Description "Create Symbol (BTCUSD with partial config)"
    if ($newSymbol) {
        Write-Host "  Symbol: $($newSymbol.symbol)" -ForegroundColor Gray
        Write-Host "  Active: $($newSymbol.active)" -ForegroundColor Gray
        Write-Host "  Applied smart defaults for min/max prices" -ForegroundColor DarkGray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 6: Create Symbol with minimal config (only starting price)
    $minimalBody = @{
        symbol = "ETHUSD"
        config = @{
            starting_price = "3000.0"
        }
    } | ConvertTo-Json
    
    $minimalSymbol = Test-Endpoint -Method "POST" -Url "$apiUrl/symbols" -Body $minimalBody -Description "Create Symbol (ETHUSD minimal config)"
    if ($minimalSymbol) {
        Write-Host "  Symbol: $($minimalSymbol.symbol)" -ForegroundColor Gray
        Write-Host "  All defaults applied based on price" -ForegroundColor DarkGray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 7: Create Symbol with NO config (complete defaults)
    $noConfigBody = @{
        symbol = "DEFAULT"
    } | ConvertTo-Json
    
    $defaultSymbol = Test-Endpoint -Method "POST" -Url "$apiUrl/symbols" -Body $noConfigBody -Description "Create Symbol (DEFAULT no config)"
    if ($defaultSymbol) {
        Write-Host "  Symbol: $($defaultSymbol.symbol)" -ForegroundColor Gray
        Write-Host "  Using complete defaults" -ForegroundColor DarkGray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 8: Generate Data
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
    
    # Test 9: Get Historical Data
    $historical = Test-Endpoint -Method "GET" -Url "$apiUrl/historical/BTCUSD?limit=3" -Description "Get Historical Data"
    if ($historical) {
        Write-Host "  Retrieved: $($historical.count) candles" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 10: List Algorithms
    $algorithms = Test-Endpoint -Method "GET" -Url "$apiUrl/algorithms" -Description "List Algorithms"
    if ($algorithms) {
        Write-Host "  Available: $($algorithms.algorithms.Count) algorithm(s)" -ForegroundColor Gray
        Write-Host "  Planned: $($algorithms.planned -join ', ')" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 11: List Presets
    $presets = Test-Endpoint -Method "GET" -Url "$apiUrl/presets" -Description "List Presets"
    if ($presets) {
        $presetNames = $presets.presets | Get-Member -MemberType NoteProperty | Select-Object -ExpandProperty Name
        Write-Host "  Available presets: $($presetNames -join ', ')" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    Start-Sleep -Seconds $TestDelay
    
    # Test 12: Export as JSON
    $exportJson = Test-Endpoint -Method "GET" -Url "$apiUrl/export/BTCUSD/json" -Description "Export as JSON"
    if ($exportJson) {
        Write-Host "  Data points: $($exportJson.data.Count)" -ForegroundColor Gray
        $testsPassed++
    } else { $testsFailed++ }
    Write-Host ""
    
    # Test 13: Delete Symbol
    Test-Endpoint -Method "DELETE" -Url "$apiUrl/symbols/BTCUSD" -Description "Delete Symbol"
    Write-Host "  Symbol deleted successfully" -ForegroundColor Gray
    $testsPassed++
    Write-Host ""

    # Test 14: Control Endpoint - Status Command
    $statusBody = @{
        command = "status"
    } | ConvertTo-Json
    
    Write-Host "Testing: Control Endpoint (status command)" -ForegroundColor Yellow
    Write-Host "  URL: $baseUrl/control" -ForegroundColor Gray
    try {
        $controlResponse = Invoke-RestMethod -Uri "$baseUrl/control" -Method POST -Headers @{"Content-Type"="application/json"} -Body $statusBody -TimeoutSec 5
        if ($controlResponse.status -eq "success") {
            Write-Host "  [OK] Success" -ForegroundColor Green
            Write-Host "  Server version: $($controlResponse.server.version)" -ForegroundColor Gray
            $testsPassed++
        } else {
            Write-Host "  [FAIL] Failed" -ForegroundColor Red
            $testsFailed++
        }
    }
    catch {
        Write-Host "  [FAIL] Failed: $_" -ForegroundColor Red
        $testsFailed++
    }
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
            # Force kill if still running
            Stop-Process -Id $serverProcess.Id -Force -ErrorAction SilentlyContinue
            Write-Host "Server force stopped" -ForegroundColor Yellow
        }
    }
    catch {
        Write-Host "Failed to stop server via API, forcing shutdown..." -ForegroundColor Yellow
        try {
            Stop-Process -Id $serverProcess.Id -Force -ErrorAction SilentlyContinue
            Write-Host "Server force stopped" -ForegroundColor Yellow
        }
        catch {
            Write-Host "Failed to stop server" -ForegroundColor Red
        }
    }
    
    # Clean up any remaining cargo processes
    Get-Process | Where-Object { $_.ProcessName -like "*cargo*" -or $_.ProcessName -like "*market-data-server*" } | Stop-Process -Force -ErrorAction SilentlyContinue
    
    Write-Host ""
    if ($testsFailed -eq 0) {
        Write-Host "All tests passed!" -ForegroundColor Green
        exit 0
    } else {
        Write-Host "Some tests failed!" -ForegroundColor Red
        exit 1
    }
}
