#!/usr/bin/env node
// WebSocket Test Script for Market Data Source Server
// This properly tests WebSocket functionality that PowerShell can't handle

const WebSocket = require('ws');

const WS_URL = 'ws://localhost:8080/ws';

console.log('Testing WebSocket connection to:', WS_URL);

const ws = new WebSocket(WS_URL);

let testsPassed = 0;
let testsFailed = 0;
let receivedWelcome = false;
let receivedPong = false;
let receivedData = false;

ws.on('open', () => {
    console.log('✓ WebSocket connected successfully');
    testsPassed++;
    
    // Test 1: Send Ping
    console.log('Sending Ping...');
    ws.send(JSON.stringify({ type: 'Ping' }));
    
    // Test 2: Subscribe to BTCUSD
    setTimeout(() => {
        console.log('Subscribing to BTCUSD...');
        ws.send(JSON.stringify({
            type: 'Subscribe',
            payload: {
                symbol: 'BTCUSD',
                interval: 1000
            }
        }));
    }, 500);
    
    // Close after 5 seconds
    setTimeout(() => {
        ws.close();
    }, 5000);
});

ws.on('message', (data) => {
    try {
        const msg = JSON.parse(data.toString());
        console.log('Received:', msg.type);
        
        if (msg.type === 'Welcome') {
            console.log('✓ Received Welcome message');
            console.log('  Version:', msg.data.version);
            console.log('  Capabilities:', msg.data.capabilities.join(', '));
            receivedWelcome = true;
            testsPassed++;
        } else if (msg.type === 'Pong') {
            console.log('✓ Received Pong response');
            receivedPong = true;
            testsPassed++;
        } else if (msg.type === 'Subscribed') {
            console.log('✓ Successfully subscribed to', msg.data.symbol);
            testsPassed++;
        } else if (msg.type === 'MarketData') {
            if (!receivedData) {
                console.log('✓ Received market data for', msg.data.symbol);
                console.log('  Price:', msg.data.ohlc.close);
                receivedData = true;
                testsPassed++;
            }
        } else if (msg.type === 'Error') {
            console.log('✗ Error:', msg.data.message);
            testsFailed++;
        }
    } catch (e) {
        console.error('Failed to parse message:', e);
        testsFailed++;
    }
});

ws.on('error', (err) => {
    console.error('✗ WebSocket error:', err.message);
    testsFailed++;
    process.exit(1);
});

ws.on('close', () => {
    console.log('\nWebSocket closed');
    console.log('===================');
    console.log('Test Results:');
    console.log('  Passed:', testsPassed);
    console.log('  Failed:', testsFailed);
    
    if (!receivedWelcome) {
        console.log('  ✗ Never received Welcome message');
    }
    if (!receivedPong) {
        console.log('  ✗ Never received Pong response');
    }
    if (!receivedData) {
        console.log('  ✗ Never received market data');
    }
    
    if (testsFailed === 0 && testsPassed >= 4) {
        console.log('\n✓ All WebSocket tests passed!');
        process.exit(0);
    } else {
        console.log('\n✗ Some WebSocket tests failed');
        process.exit(1);
    }
});

console.log('Connecting to WebSocket...');