# OData for Rust

OData crate with helper functions and structures to process OData 4 requests and responses.

_This project is in a very early stage of development and not ready for general use._

## Try it out

### Online sample client
You can run the sample client by executing: `cargo run --bin sample-client`

### Sample server implementation
You can run the sample server by executing: `cargo run --bin sample-server`

Try a request with something like: http://localhost:8080/V4/TripPinService/Products?$filter=Name eq 'Milk' and Price lt 2.55

## DISCLAIMER

Please note: all content in this repository is released for use "AS IS" without any warranties of any kind, including, but not limited to their installation, use, or performance. We disclaim any and all warranties, either express or implied, including but not limited to any warranty of noninfringement, merchantability, and/ or fitness for a particular purpose. We do not warrant that the technology will meet your requirements, that the operation thereof will be uninterrupted or error-free, or that any errors will be corrected.

Any use of these scripts and tools is at your own risk. There is no guarantee that they have been through thorough testing in a comparable environment and we are not responsible for any damage or data loss incurred with their use.

You are responsible for reviewing and testing any generated code you run thoroughly before use in any non-testing environment.

## About Avaya

Avaya elevates communications to the next generation of engagement, connecting organizations to their customers, workforce, and communities with secure, intelligent experiences that matter.

Check us out on: https://www.avaya.com