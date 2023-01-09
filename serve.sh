http-server /d/invokeai/outputs/ --p 8000 --cors & cargo run --bin service & cd application && trunk serve --address 0.0.0.0 && kill $!
