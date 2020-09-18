# Bombolone
Babycam Web Server

## Design
This is a web server and http proxy that authenticates and authorizes requests to the mjpg videa stream from the Raspberry Pi Camera. 

Authentication is done via code that is managed in the config and displayed as a QR code for pairing. 

The video stream can be served by something like mjpg_streamer in http mode or something supporting either the PiCam or Video4Linux drivers depending on your target architecture. The only real requirement is that the video stream be served over HTTP that Bombolone will proxy. 

## Technology
- Rust
- Rustls
- Actix-Web

## Development and Building

### Debug Build
Development and testing should be done with a debug build for faster build times. 
```shell script
cargo build 
```

### Release Build
For optimized production build
```shell script
cargo build --release
```

### ARMv7 for RPI 
If cross compiling from AMD64 linux, setup the toolchain properly and then do the following:
```shell script
cargo build --release --target armv7-unknown-linux-gnueabihf
```

## Configuration
Bombolone uses dotenv for configuration. 

If making publicly accessible, please set the `DOMAIN` env variable to your external domain for cookies to work correctly. 

```shell script
# HTTP Server Options
BIND_ADDR=127.0.0.1
BIND_PORT=8443

# TLS
CERT_FILE=./static/cert/cert.pem
KEY_FILE=./static/cert/key.pem

# Video Stream URL
STREAM_BASE_URL=http://localhost:8080

# Static Web
STATIC_WEB_ROOT=./static/root/

# Pairing Code
APP_SECRET=poopydiaper

# Cookie
SIGNING_KEY=abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123

```

