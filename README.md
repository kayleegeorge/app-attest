## App Attest

This library implements the Apple attestation protocol in Rust.

### Validating apps that connect to your server

when your app connects to your server, you may need to validate that the app is an authentic and untampered instance of your app.

Apple creates an attestation object, `Attestation` that consists of authenticator data and an attestation statement. The attestation statement contains the x5c certificate chain that is used to verify the authenticator data.

After successful attestation, the server can require the client to accompany server requests with an assertion object, `Assertion`. The assertion object reestablishes the legitamcy of the app.

[App Attest Docs](https://developer.apple.com/documentation/devicecheck/establishing-your-app-s-integrity)
