## Introduction
This is a Chrome extension to create event in Google Calander from selected text with the help of ChatGPT.
Also the code is mainly generated by ChatGPT.

## Install
1. Go to url: chrome://extensions/
2. Click load unpacked extension Chrome
3. Select `ashisutanto-extension/build`

## Replace API-KEY
1. Replace the api_key in `src/gpt.rs`
2. Compile it to wasm with `wasm-pack build --target web --out-dir ./pkg`
3. Copy the .js and .wasm file from `pkg/` into `ashisutanto-extension/public/`
4. Build react app in `ashisutanto-extension` with `npm run build`