# Bombolone
Babycam Web Server

![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/andricDu/Bombolone/Rust/main?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/andricDu/Bombolone?style=for-the-badge)

## Design
This is a web server and http proxy that authenticates and authorizes requests to the mjpg video stream from the Raspberry Pi Camera. 

Authentication is done via code that is managed in the config and displayed as a QR code for pairing. 

The video stream can be served by something like mjpg_streamer in http mode or something supporting either the PiCam or Video4Linux drivers depending on your target architecture. The only real requirement is that the video stream be served over HTTP that Bombolone will proxy. 

### Why this way
- Rustls instead of openssl
- Centralized web server, http proxy, and auth into a single tiny deployable 
- Allows for build of microservices for handling things like camera and gpio sensors while being able to distribute them to PiZeros running independently. 

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


## Running
Either build and run the binary or you can invoke the cargo command. Bombolone will print out the QR code you can use for pairing. 
```shell script
$ cargo run 
   Compiling bombolone v0.1.0 (/home/baby_cam/Bombolone)
    Finished dev [unoptimized + debuginfo] target(s) in 11.49s
     Running `target/debug/bombolone`
                                                                                       
                                                                                       
                                                                                       
                                                                                       
            █████████████████████   █████████   ███   █████████████████████            
            ███               ███   ███   ███         ███               ███            
            ███   █████████   ███      ████████████   ███   █████████   ███            
            ███   █████████   ███   █████████   ███   ███   █████████   ███            
            ███   █████████   ███            ███      ███   █████████   ███            
            ███               ███      ███   ██████   ███               ███            
            █████████████████████   ███   ███   ███   █████████████████████            
                                    ███   █████████                                    
            ███   ██████   █████████         ██████   ███      ███   ██████            
            ██████   ███   ███   ███   ██████   ██████      ███   ███   ███            
                  ███         ██████   ███   ███   █████████         ██████            
               ███         ███      ███████████████      ███   ███                     
            ██████         █████████               ███      ███      ██████            
                                    ██████   █████████      ███                        
            █████████████████████   ██████   ███      █████████                        
            ███               ███   ███   ██████   ███         █████████               
            ███   █████████   ███               ███         ████████████               
            ███   █████████   ███   █████████   ███   ██████   ███   ███               
            ███   █████████   ███   ███   █████████            ██████                  
            ███               ███         ███   ███████████████         ███            
            █████████████████████   ███      ███████████████   ███                     
                                                                                       
                                                                                       
                                                                                       
                                                                                       
[2020-09-18T21:03:19Z INFO  actix_server::builder] Starting 8 workers
[2020-09-18T21:03:19Z INFO  actix_server::builder] Starting "actix-web-service-192.168.1.7:8443" service on 192.168.1.7:8443

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

