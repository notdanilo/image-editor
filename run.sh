http-server /d/invokeai/outputs/ --p 8000 --cors & cargo run --bin service & cargo run --bin image-editor && kill $!
