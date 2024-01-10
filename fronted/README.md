# Quest





## UseAge
1. install:`pnpm i`
2. run dev:`pnpm dev`


## Dev
1. shadcn-ui install:`pnpm dlx shadcn-ui@latest init`
    ```bash
    > pnpm dlx shadcn-ui@latest init
    ../../../../.pnpm-store/v3/tmp/dlx-26516 | +199 ++++++++++++++++++++
    ../../../../.pnpm-store/v3/tmp/dlx-26516 | Progress: resolved 199, reused 199, downloaded 0, added 199, done
    √ Would you like to use TypeScript (recommended)? ... no / yes
    √ Which style would you like to use? » New York
    √ Which color would you like to use as base color? » Gray
    √ Where is your global CSS file? ... app/globals.css
    √ Would you like to use CSS variables for colors? ... no / yes
    √ Where is your tailwind.config.js located? ... tailwind.config.js
    √ Configure the import alias for components: ... @/components
    √ Configure the import alias for utils: ... @/components/lib/utils
    √ Are you using React Server Components? ... no / yes
    √ Write configuration to components.json. Proceed? ... yes
    
    ✔ Writing components.json...
    ✔ Initializing project...
    ✔ Installing dependencies...
    ```
2. add near sdk
    ```bash
    pnpm update
    pnpm i near-api-js@^2.1.3
    pnpm i @near-wallet-selector/core
    pnpm install @near-wallet-selector/my-near-wallet @near-wallet-selector/here-wallet @near-wallet-selector/modal-ui
    ```



            // "test:integration": "npm run build:contract && cd integration-tests && npm test -- -- \"./contract/target/wasm32-unknown-unknown/release/contract.wasm\""
        // "integration_test_contract": "cargo run --example test \"./target/wasm32-unknown-unknown/release/hello_near.wasm\"",
        // "start": "cd frontend && npm run start",
        // "deploy": "cd contract && ./deploy.sh",
        // "build": "npm run build:contract && npm run build:web",
        // "build:web": "cd frontend && npm run build",
        // "build:contract": "cd contract && ./build.sh",
        // "test": "npm run test:unit && npm run test:integration",
        // "test:unit": "cd contract && cargo test",
        // "postinstall": "cd integration-tests && npm install && cd .. && echo rs contract"