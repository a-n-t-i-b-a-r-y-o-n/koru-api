koru-api
==========
_API servers/clients for use with koru library_

## Roadmap

### Security
- [ ] Implement TLS/SSL for REST
- [ ] Encrypt the database? (it's just metadata TBH)

### Configuration
- [ ] Command-line arguments
- [ ] Config file?

### Database
- [ ] Make location configurable
- [ ] Add code to create db programmatically
    - [x] ~~Linux~~
    - [ ] Windows
    - [ ] macOS/BSD

### Error handling
- [ ] Work on refactoring out occurrences of `unwrap()`

### Features
- [ ] MQTT client