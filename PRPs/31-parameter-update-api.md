# PRP-31: Parameter Update API Endpoints

## Context & Motivation

**Integration Goal**: Expose REST API endpoints for runtime parameter updates, building on PRP-30's scheduler.

**User Requirement**: Enable external systems to modify generator parameters via HTTP API calls.

**Technical Challenge**: Design intuitive API that maintains consistency and provides feedback.

## Requirements

### API Design
1. **RESTful Endpoints**: CRUD operations for parameter updates
2. **Batch Updates**: Support multiple parameter changes in one request
3. **Validation Responses**: Clear error messages for invalid updates
4. **Update Status**: Query pending and applied updates

### Security & Validation
1. **Input Validation**: Comprehensive parameter validation
2. **Rate Limiting**: Prevent update flooding
3. **Audit Logging**: Track all parameter modifications
4. **Rollback API**: Endpoints to revert changes

## Implementation Blueprint

### Phase 1: API Endpoints
1. Extend `src/server/api/handlers.rs` with update endpoints
2. Add `PUT /api/parameters` for immediate updates
3. Add `POST /api/parameters/schedule` for scheduled updates
4. Add `GET /api/parameters/history` for update history

### Phase 2: Request Handling
1. Define update request/response models in `models.rs`
2. Implement validation middleware
3. Add rate limiting using Tower middleware
4. Create audit log integration

### Phase 3: WebSocket Updates
1. Add WebSocket notifications for parameter changes
2. Implement subscription model for update events
3. Create real-time parameter sync
4. Add connection management

## Success Criteria

### Validation Gates
```bash
# Test API endpoints
cargo test api_parameters
cargo test --features api-server integration

# API documentation
cargo doc --features api-server --open
```

### Implementation Metrics
- [ ] All endpoints return within 100ms
- [ ] Comprehensive OpenAPI documentation
- [ ] 100% input validation coverage
- [ ] WebSocket latency < 50ms

## Dependencies & References

**Prerequisites**:
- PRP-30 (Dynamic Parameter Scheduler) must be completed
- Existing API server infrastructure

**Rust Libraries**:
- Existing: `axum`, `tower`, `utoipa`
- Consider: `tower-governor` for rate limiting
- Use `tracing` for audit logs

**API Patterns**:
- Follow existing handler patterns in `handlers.rs`
- Use same validation approach as current endpoints
- Extend OpenAPI spec consistently

## Implementation Tasks

### Phase 1: Core Endpoints (2-3 hours)
1. Define API models
2. Implement update endpoints
3. Add validation logic
4. Write API tests

### Phase 2: Advanced Features (2-3 hours)
1. Add batch update support
2. Implement rate limiting
3. Create audit logging
4. Add rollback endpoints

### Phase 3: Real-time Updates (1-2 hours)
1. Extend WebSocket support
2. Add parameter subscriptions
3. Test real-time sync
4. Update documentation

## Risk Mitigation
- Validate all inputs thoroughly
- Include request size limits
- Add operation timeouts
- Provide clear error messages

## Success Score
**9/10** - Builds on existing API infrastructure with clear patterns to follow.