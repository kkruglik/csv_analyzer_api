# CSV Analyzer API - Application Design

## Overview

REST API wrapper for the `csv_processor` library providing HTTP endpoints for CSV data analysis, statistics, and JSON export functionality.

## Core Principles

- **Library-First**: Leverage existing `csv_processor` crate capabilities
- **Stateless Design**: Each request is independent, no session management
- **Resource-Oriented**: RESTful endpoints following HTTP semantics
- **Performance**: Async processing with efficient memory usage
- **Error Resilience**: Comprehensive error handling and validation

## High-Level Architecture

### Layered Architecture

```
┌─────────────────────────────────────┐
│          HTTP Layer (Axum)          │  ← Routing, middleware, serialization
├─────────────────────────────────────┤
│       Handler Layer                 │  ← Request/response handling
├─────────────────────────────────────┤
│       Service Layer                 │  ← Business logic orchestration
├─────────────────────────────────────┤
│    csv_processor Library            │  ← Core CSV processing logic
└─────────────────────────────────────┘
```

## Domain Model

### Core Entities

- **CSV Dataset**: Uploaded CSV data with metadata
- **Column Analysis**: Type inference and statistical summary
- **Processing Report**: Analysis results and missing value reports
- **Export Format**: JSON serialization options (columns/records/values)

### Data Flow

1. **Upload** → CSV file received via multipart/form-data
2. **Process** → `csv_processor::DataFrame` creation and analysis
3. **Analyze** → Statistical calculations and type inference
4. **Export** → JSON transformation in requested orientation

## API Design Patterns

### Resource Identification

- Use UUIDs for temporary dataset identification
- Stateless processing with optional result caching
- RESTful URL structure: `/api/v1/csv/{operation}`

### Request/Response Patterns

- **Async Processing**: Long-running analysis operations
- **Content Negotiation**: Support multiple export formats
- **Error Standardization**: Consistent error response structure
- **Validation**: Input validation at handler level

## Non-Functional Requirements

### Performance

- Stream processing for large CSV files
- Memory-efficient DataFrame operations
- Async I/O for concurrent request handling

### Reliability

- Input validation and sanitization
- Graceful error handling and recovery
- Resource cleanup and memory management

### Scalability

- Stateless design for horizontal scaling
- Configurable processing limits
- Resource monitoring and metrics

## Technology Decisions

### Framework Choice: Axum

- **Rationale**: Modern async-first Rust web framework
- **Benefits**: Type-safe extractors, excellent performance, Tower ecosystem
- **Trade-offs**: Learning curve, fewer examples than older frameworks

### Processing Library: csv_processor

- **Integration**: Direct dependency, no serialization overhead
- **Extension**: Minimal wrapper logic around core functionality
- **Compatibility**: Version pinning for API stability

### Data Serialization: Serde

- **JSON**: Primary response format with multiple orientations
- **Validation**: Derive-based request/response models
- **Error Handling**: Automatic serialization error mapping

## Security Considerations

### Input Validation

- File size limits and MIME type validation
- CSV structure validation before processing
- Malformed data handling and error boundaries

### Resource Protection

- Request rate limiting and timeout handling
- Memory usage limits for large datasets
- Temporary file cleanup and storage limits

## Development Guidelines

### Code Organization

- Handler functions: Thin, focused on HTTP concerns
- Service layer: Business logic and csv_processor orchestration
- Model definitions: Request/response DTOs with validation
- Error types: Domain-specific error handling

### Testing Strategy

- Unit tests: Service layer and utility functions
- Integration tests: End-to-end API endpoint testing
- Property tests: CSV processing edge cases
- Performance tests: Large dataset handling

### Configuration Management

- Environment-based configuration
- Validation at startup with meaningful error messages
- Separate configs for development/production environments

