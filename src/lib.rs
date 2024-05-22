pub mod assertion;
pub mod attestation;
pub mod certificate;
pub mod root;
pub mod types;
pub mod utils;

/*
 * First decode the raw attestation and key_id objects. Then validate.
 */
pub fn validate_raw_attestation(
    raw_attestation: &str,
    raw_key_id: &str,
    challenge: &str,
    app_id: &str,
    production: bool,
    leaf_cert_only: bool,
) -> bool {
    let attestation = utils::decode_attestation(raw_attestation.to_string()).unwrap();
    let key_id = utils::decode_base64_to_bytes(&raw_key_id.to_string());

    validate_decoded_attestation(
        attestation,
        challenge.to_string(),
        key_id,
        app_id.to_string(),
        production,
        leaf_cert_only,
    )
}

/*
 * Validate a decoded attestation object.
 */
pub fn validate_decoded_attestation(
    attestation: types::AttestationObject,
    challenge: String,
    key_id: Vec<u8>,
    app_id: String,
    production: bool,
    leaf_cert_only: bool,
) -> bool {
    attestation::validate_attestation(
        attestation,
        challenge,
        key_id,
        app_id,
        production,
        leaf_cert_only,
    )
}

/*
 * Decode a raw assertion and client data object. Then validate.
 */
pub fn validate_raw_assertion(
    raw_assertion: &str,
    raw_client_data: &str,
    public_key_uncompressed_hex: &str,
    client_app_id: &str,
    stored_challenge: &str,
    prev_counter: u32,
) -> bool {
    let assertion = utils::decode_assertion(raw_assertion.to_string()).unwrap();
    let client_data = utils::decode_base64_to_bytes(&raw_client_data.to_string());

    validate_decoded_assertion(
        assertion,
        client_data,
        public_key_uncompressed_hex.to_string(),
        client_app_id.to_string(),
        stored_challenge.to_string(),
        prev_counter,
    )
}

/*
 * Validate a decoded assertion object.
 */
pub fn validate_decoded_assertion(
    assertion: types::AssertionObject,
    client_data: Vec<u8>,
    public_key_uncompressed_hex: String,
    client_app_id: String,
    stored_challenge: String,
    prev_counter: u32,
) -> bool {
    assertion::validate_assertion(
        assertion,
        client_data,
        public_key_uncompressed_hex,
        client_app_id,
        stored_challenge,
        prev_counter,
    )
}

// Decode base64 string into bytes.
pub fn decode_base64_to_bytes(encoded: String) -> Vec<u8> {
    let decoded = utils::decode_base64_to_bytes(&encoded);
    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example usage to validate raw attestation object.
    #[test]
    fn test_validate_attestation() {
        // Valid attestation object.
        let raw_attestation: &str = "o2NmbXRvYXBwbGUtYXBwYXR0ZXN0Z2F0dFN0bXSiY3g1Y4JZAuswggLnMIICbaADAgECAgYBeNT03AYwCgYIKoZIzj0EAwIwTzEjMCEGA1UEAwwaQXBwbGUgQXBwIEF0dGVzdGF0aW9uIENBIDExEzARBgNVBAoMCkFwcGxlIEluYy4xEzARBgNVBAgMCkNhbGlmb3JuaWEwHhcNMjEwNDE0MDk1NTIwWhcNMjEwNDE3MDk1NTIwWjCBkTFJMEcGA1UEAwxAMDFjM2ZmYTY3YTY4MzU1M2M4MjU4NjRlYmU2MjJmNWIzMGVmOWIxOTA1YTEwMDg0ZTE0YmJiMzY0ZTk2ODgwMDEaMBgGA1UECwwRQUFBIENlcnRpZmljYXRpb24xEzARBgNVBAoMCkFwcGxlIEluYy4xEzARBgNVBAgMCkNhbGlmb3JuaWEwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAAQ3xAT6K7-PvPTucIBXPV-oDE9sw6IvfbQ6-Sw5TnzRyIDJWrQilyYl6OZzrxvaKwlmVOm2AolWAfklu1lBxTCCo4HxMIHuMAwGA1UdEwEB_wQCMAAwDgYDVR0PAQH_BAQDAgTwMH4GCSqGSIb3Y2QIBQRxMG-kAwIBCr-JMAMCAQG_iTEDAgEAv4kyAwIBAb-JMwMCAQG_iTQmBCQzNU1GWVkySlk1LmNvLmNoaWZmLmF0dGVzdGF0aW9uLXRlc3SlBgQEc2tzIL-JNgMCAQW_iTcDAgEAv4k5AwIBAL-JOgMCAQAwGQYJKoZIhvdjZAgHBAwwCr-KeAYEBDE0LjQwMwYJKoZIhvdjZAgCBCYwJKEiBCCOPSSk1ZLu7Zc9Zd2TmGO7tY5ktIclyAclmfTBJdpmjjAKBggqhkjOPQQDAgNoADBlAjEAzHk20GzLdZlaaJXKchriZkmJWhfTCgQHRpn3D6Y7Coit7UQABhIABVh6D4qwPysZAjAFDGuGqb796A9H-1UVCgui5ufZnWZHl1SVT-6iobxfS9av2ahGkLF8hYQXVT3pofxZAkcwggJDMIIByKADAgECAhAJusXhvEAa2dRTlbw4GghUMAoGCCqGSM49BAMDMFIxJjAkBgNVBAMMHUFwcGxlIEFwcCBBdHRlc3RhdGlvbiBSb290IENBMRMwEQYDVQQKDApBcHBsZSBJbmMuMRMwEQYDVQQIDApDYWxpZm9ybmlhMB4XDTIwMDMxODE4Mzk1NVoXDTMwMDMxMzAwMDAwMFowTzEjMCEGA1UEAwwaQXBwbGUgQXBwIEF0dGVzdGF0aW9uIENBIDExEzARBgNVBAoMCkFwcGxlIEluYy4xEzARBgNVBAgMCkNhbGlmb3JuaWEwdjAQBgcqhkjOPQIBBgUrgQQAIgNiAASuWzegd015sjWPQOfR8iYm8cJf7xeALeqzgmpZh0_40q0VJXiaomYEGRJItjy5ZwaemNNjvV43D7-gjjKegHOphed0bqNZovZvKdsyr0VeIRZY1WevniZ-smFNwhpmzpmjZjBkMBIGA1UdEwEB_wQIMAYBAf8CAQAwHwYDVR0jBBgwFoAUrJEQUzO9vmhB_6cMqeX66uXliqEwHQYDVR0OBBYEFD7jXRwEGanJtDH4hHTW4eFXcuObMA4GA1UdDwEB_wQEAwIBBjAKBggqhkjOPQQDAwNpADBmAjEAu76IjXONBQLPvP1mbQlXUDW81ocsP4QwSSYp7dH5FOh5mRya6LWu-NOoVDP3tg0GAjEAqzjt0MyB7QCkUsO6RPmTY2VT_swpfy60359evlpKyraZXEuCDfkEOG94B7tYlDm3Z3JlY2VpcHRZDl0wgAYJKoZIhvcNAQcCoIAwgAIBATEPMA0GCWCGSAFlAwQCAQUAMIAGCSqGSIb3DQEHAaCAJIAEggPoMYIEGDAsAgECAgEBBCQzNU1GWVkySlk1LmNvLmNoaWZmLmF0dGVzdGF0aW9uLXRlc3QwggL1AgEDAgEBBIIC6zCCAucwggJtoAMCAQICBgF41PTcBjAKBggqhkjOPQQDAjBPMSMwIQYDVQQDDBpBcHBsZSBBcHAgQXR0ZXN0YXRpb24gQ0EgMTETMBEGA1UECgwKQXBwbGUgSW5jLjETMBEGA1UECAwKQ2FsaWZvcm5pYTAeFw0yMTA0MTQwOTU1MjBaFw0yMTA0MTcwOTU1MjBaMIGRMUkwRwYDVQQDDEAwMWMzZmZhNjdhNjgzNTUzYzgyNTg2NGViZTYyMmY1YjMwZWY5YjE5MDVhMTAwODRlMTRiYmIzNjRlOTY4ODAwMRowGAYDVQQLDBFBQUEgQ2VydGlmaWNhdGlvbjETMBEGA1UECgwKQXBwbGUgSW5jLjETMBEGA1UECAwKQ2FsaWZvcm5pYTBZMBMGByqGSM49AgEGCCqGSM49AwEHA0IABDfEBPorv4-89O5wgFc9X6gMT2zDoi99tDr5LDlOfNHIgMlatCKXJiXo5nOvG9orCWZU6bYCiVYB-SW7WUHFMIKjgfEwge4wDAYDVR0TAQH_BAIwADAOBgNVHQ8BAf8EBAMCBPAwfgYJKoZIhvdjZAgFBHEwb6QDAgEKv4kwAwIBAb-JMQMCAQC_iTIDAgEBv4kzAwIBAb-JNCYEJDM1TUZZWTJKWTUuY28uY2hpZmYuYXR0ZXN0YXRpb24tdGVzdKUGBARza3Mgv4k2AwIBBb-JNwMCAQC_iTkDAgEAv4k6AwIBADAZBgkqhkiG92NkCAcEDDAKv4p4BgQEMTQuNDAzBgkqhkiG92NkCAIEJjAkoSIEII49JKTVku7tlz1l3ZOYY7u1jmS0hyXIByWZ9MEl2maOMAoGCCqGSM49BAMCA2gAMGUCMQDMeTbQbMt1mVpolcpyGuJmSYlaF9MKBAdGmfcPpjsKiK3tRAAGEgAFWHoPirA_KxkCMAUMa4apvv3oD0f7VRUKC6Lm59mdZkeXVJVP7qKhvF9L1q_ZqEaQsXyFhBdVPemh_DAoAgEEAgEBBCBsbdptlEbTsu5ktHjBTEiDsfbajKOKz4hxgskGW0mjojBgAgEFAgEBBFgxZzZKcm5JdXg5eHFzWDFzSDQ1ekUwUzVvWGJCM0Njenp3aVpZOXJxSkMxc2ZWa3J0T3ZIRk92UXF5Wjg1NE80Yk5zOEloWkV1eVRzNmZPQ01VMmtlZz09MA4CAQYCAQEEBkFUVEVTVDAPAgEHAgEBBAdzYW5kYm94MCACAQwCAQEEGDIwMjEtMAQ0NC0xNVQwOTo1NToyMC4yMDdaMCACARUCAQEEGDIwMjEtMDctMTRUMDk6NTU6MjAuMjA3WgAAAAAAAKCAMIIDrTCCA1SgAwIBAgIQWTNWreVZgs9EQjes30UbUzAKBggqhkjOPQQDAjB8MTAwLgYDVQQDDCdBcHBsZSBBcHBsaWNhdGlvbiBJbnRlZ3JhdGlvbiBDQSA1IC0gRzExJjAkBgNVBAsMHUFwcGxlIENlcnRpZmljYXRpb24gQXV0aG9yaXR5MRMwEQYDVQQKDApBcHBsZSBJbmMuMQswCQYDVQQGEwJVUzAeFw0yMDA1MTkxNzQ3MzFaFw0yMTA2MTgxNzQ3MzFaMFoxNjA0BgNVBAMMLUFwcGxpY2F0aW9uIEF0dGVzdGF0aW9uIEZyYXVkIFJlY2VpcHQgU2lnbmluZzETMBEGA1UECgwKQXBwbGUgSW5jLjELMAkGA1UEBhMCVVMwWTATBgcqhkjOPQIBBggqhkjOPQMBBwNCAAR_6RU0bMOKe5g8k9HQQ1_Yq9pWcATTLFiGZVGVerR498sq-LpF9_p46sYsSeT5zcCEtQMU8QIz2pt2-kQqK7hyo4IB2DCCAdQwDAYDVR0TAQH_BAIwADAfBgNVHSMEGDAWgBTZF_5LZ5A4S5L0287VV4AUC489yTBDBggrBgEFBQcBAQQ3MDUwMwYIKwYBBQUHMAGGJ2h0dHA6Ly9vY3NwLmFwcGxlLmNvbS9vY3NwMDMtYWFpY2E1ZzEwMTCCARwGA1UdIASCARMwggEPMIIBCwYJKoZIhvdjZAUBMIH9MIHDBggrBgEFBQcCAjCBtgyBs1JlbGlhbmNlIG9uIHRoaXMgY2VydGlmaWNhdGUgYnkgYW55IHBhcnR5IGFzc3VtZXMgYWNjZXB0YW5jZSBvZiB0aGUgdGhlbiBhcHBsaWNhYmxlIHN0YW5kYXJkIHRlcm1zIGFuZCBjb25kaXRpb25zIG9mIHVzZSwgY2VydGlmaWNhdGUgcG9saWN5IGFuZCBjZXJ0aWZpY2F0aW9uIHByYWN0aWNlIHN0YXRlbWVudHMuMDUGCCsGAQUFBwIBFilodHRwOi8vd3d3LmFwcGxlLmNvbS9jZXJ0aWZpY2F0ZWF1dGhvcml0eTAdBgNVHQ4EFgQUaR7HD0fs443ddTdE8-nhWmwQViUwDgYDVR0PAQH_BAQDAgeAMA8GCSqGSIb3Y2QMDwQCBQAwCgYIKoZIzj0EAwIDRwAwRAIgJRgWXF4pnFn2hTmtXduZ9jc-9g7NCEWp_Xca1iQtLCICIF0qmypfq6NjgWWNGED3r0gL12uhlNg0IIf01pNbtRuuMIIC-TCCAn-gAwIBAgIQVvuD1Cv_jcM3mSO1Wq5uvTAKBggqhkjOPQQDAzBnMRswGQYDVQQDDBJBcHBsZSBSb290IENBIC0gRzMxJjAkBgNVBAsMHUFwcGxlIENlcnRpZmljYXRpb24gQXV0aG9yaXR5MRMwEQYDVQQKDApBcHBsZSBJbmMuMQswCQYDVQQGEwJVUzAeFw0xOTAzMjIxNzUzMzNaFw0zNDAzMjIwMDAwMDBaMHwxMDAuBgNVBAMMJ0FwcGxlIEFwcGxpY2F0aW9uIEludGVncmF0aW9uIENBIDUgLSBHMTEmMCQGA1UECwwdQXBwbGUgQ2VydGlmaWNhdGlvbiBBdXRob3JpdHkxEzARBgNVBAoMCkFwcGxlIEluYy4xCzAJBgNVBAYTAlVTMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEks5jvX2GsasoCjsc4a_7BJSAkaz2Md-myyg1b0RL4SHlV90SjY26gnyVvkn6vjPKrs0EGfEvQyX69L6zy4N-uqOB9zCB9DAPBgNVHRMBAf8EBTADAQH_MB8GA1UdIwQYMBaAFLuw3qFYM4iapIqZ3r6966_ayySrMEYGCCsGAQUFBwEBBDowODA2BggrBgEFBQcwAYYqaHR0cDovL29jc3AuYXBwbGUuY29tL29jc3AwMy1hcHBsZXJvb3RjYWczMDcGA1UdHwQwMC4wLKAqoCiGJmh0dHA6Ly9jcmwuYXBwbGUuY29tL2FwcGxlcm9vdGNhZzMuY3JsMB0GA1UdDgQWBBTZF_5LZ5A4S5L0287VV4AUC489yTAOBgNVHQ8BAf8EBAMCAQYwEAYKKoZIhvdjZAYCAwQCBQAwCgYIKoZIzj0EAwMDaAAwZQIxAI1vpp-h4OTsW05zipJ_PXhTmI_02h9YHsN1Sv44qEwqgxoaqg2mZG3huZPo0VVM7QIwZzsstOHoNwd3y9XsdqgaOlU7PzVqyMXmkrDhYb6ASWnkXyupbOERAqrMYdk4t3NKMIICQzCCAcmgAwIBAgIILcX8iNLFS5UwCgYIKoZIzj0EAwMwZzEbMBkGA1UEAwwSQXBwbGUgUm9vdCBDQSAtIEczMSYwJAYDVQQLDB1BcHBsZSBDZXJ0aWZpY2F0aW9uIEF1dGhvcml0eTETMBEGA1UECgwKQXBwbGUgSW5jLjELMAkGA1UEBhMCVVMwHhcNMTQwNDMwMTgxOTA2WhcNMzkwNDMwMTgxOTA2WjBnMRswGQYDVQQDDBJBcHBsZSBSb290IENBIC0gRzMxJjAkBgNVBAsMHUFwcGxlIENlcnRpZmljYXRpb24gQXV0aG9yaXR5MRMwEQYDVQQKDApBcHBsZSBJbmMuMQswCQYDVQQGEwJVUzB2MBAGByqGSM49AgEGBSuBBAAiA2IABJjpLz1AcqTtkyJygRMc3RCV8cWjTnHcFBbZDuWmBSp3ZHtfTjjTuxxEtX_1H7YyYl3J6YRbTzBPEVoA_VhYDKX1DyxNB0cTddqXl5dvMVztK517IDvYuVTZXpmkOlEKMaNCMEAwHQYDVR0OBBYEFLuw3qFYM4iapIqZ3r6966_ayySrMA8GA1UdEwEB_wQFMAMBAf8wDgYDVR0PAQH_BAQDAgEGMAoGCCqGSM49BAMDA2gAMGUCMQCD6cHEFl4aXTQY2e3v9GwOAEZLuN-yRhHFD_3meoyhpmvOwgPUnPWTxnS4at-qIxUCMG1mihDK1A3UT82NQz60imOlM27jbdoXt2QfyFMm-YhidDkLF1vLUagM6BgD56KyKAAAMYH9MIH6AgEBMIGQMHwxMDAuBgNVBAMMJ0FwcGxlIEFwcGxpY2F0aW9uIEludGVncmF0aW9uIENBIDUgLSBHMTEmMCQGA1UECwwdQXBwbGUgQ2VydGlmaWNhdGlvbiBBdXRob3JpdHkxEzARBgNVBAoMCkFwcGxlIEluYy4xCzAJBgNVBAYTAlVTAhBZM1at5VmCz0RCN6zfRRtTMA0GCWCGSAFlAwQCAQUAMAoGCCqGSM49BAMCBEcwRQIgUEClatNpJhJevokCcdbzCvmLPTGKgCpqTcAqo75reeACIQD6mKXj7_E__f78hraVFpg1Bgu44k8zimIrwFpp_5YogAAAAAAAAGhhdXRoRGF0YVikfO8rVdpyQQi6kSeW9nX_AL5x1S2uJo-miNNMptJ_cHRAAAAAAGFwcGF0dGVzdGRldmVsb3AAIAHD_6Z6aDVTyCWGTr5iL1sw75sZBaEAhOFLuzZOlogApQECAyYgASFYIDfEBPorv4-89O5wgFc9X6gMT2zDoi99tDr5LDlOfNHIIlgggMlatCKXJiXo5nOvG9orCWZU6bYCiVYB-SW7WUHFMII";
        let challenge: &str = "attestation-test";
        let raw_key_id: &str = "AcP/pnpoNVPIJYZOvmIvWzDvmxkFoQCE4Uu7Nk6WiAA=";
        let app_id: &str = "35MFYY2JY5.co.chiff.attestation-test";

        // Production is set to false.
        let result =
            validate_raw_attestation(raw_attestation, raw_key_id, challenge, app_id, false, false);

        assert_eq!(result, true);
    }

    // Example of validating a raw assertion object.
    #[test]
    fn test_validate_assertion() {
        // Valid assertion object.
        let encoded_assertion: &str = "omlzaWduYXR1cmVYRzBFAiEAyC5S3pcvtSpmTfNSd8aJRJCQ6PbN7Dnv_oPkZNMLeIwCIBmxCHXKYyGswzp_LwOxoL18puHooxudXWqDgtTvRomdcWF1dGhlbnRpY2F0b3JEYXRhWCV87ytV2nJBCLqRJ5b2df8AvnHVLa4mj6aI00ym0n9wdEAAAAAD";
        let client_data_encoded: &str = "eyJjaGFsbGVuZ2UiOiJhc3NlcnRpb24tdGVzdCJ9";

        let stored_challenge: &str = "assertion-test";
        let client_id: &str = "35MFYY2JY5.co.chiff.attestation-test";
        let prev_counter: u32 = 0;
        let public_key_uncompressed_hex: &str = "0437c404fa2bbf8fbcf4ee7080573d5fa80c4f6cc3a22f7db43af92c394e7cd1c880c95ab422972625e8e673af1bda2b096654e9b602895601f925bb5941c53082";

        let result = validate_raw_assertion(
            encoded_assertion,
            client_data_encoded,
            public_key_uncompressed_hex,
            client_id,
            stored_challenge,
            prev_counter,
        );

        assert_eq!(result, true);
    }
}
