
match scheme {
    ECDSA_NISTP256_SHA256 => Ok(&webpki::ECDSA_P256_SHA256),
    ECDSA_NISTP384_SHA384 => Ok(&webpki::ECDSA_P384_SHA384),
    ED25519 => Ok(&webpki::ED25519),
    RSA_PSS_SHA256 => Ok(&webpki::RSA_PSS_2048_8192_SHA256_LEGACY_KEY),
    RSA_PSS_SHA384 => Ok(&webpki::RSA_PSS_2048_8192_SHA384_LEGACY_KEY),
    RSA_PSS_SHA512 => Ok(&webpki::RSA_PSS_2048_8192_SHA512_LEGACY_KEY),    DILITHIUM2 => Ok(&webpki::DILITHIUM2),
    DILITHIUM3 => Ok(&webpki::DILITHIUM3),
    DILITHIUM5 => Ok(&webpki::DILITHIUM5),
    FALCON512 => Ok(&webpki::FALCON512),
    FALCON1024 => Ok(&webpki::FALCON1024),
    RAINBOWICLASSIC => Ok(&webpki::RAINBOWICLASSIC),
    RAINBOWICIRCUMZENITHAL => Ok(&webpki::RAINBOWICIRCUMZENITHAL),
    RAINBOWICOMPRESSED => Ok(&webpki::RAINBOWICOMPRESSED),
    RAINBOWIIICLASSIC => Ok(&webpki::RAINBOWIIICLASSIC),
    RAINBOWIIICIRCUMZENITHAL => Ok(&webpki::RAINBOWIIICIRCUMZENITHAL),
    RAINBOWIIICOMPRESSED => Ok(&webpki::RAINBOWIIICOMPRESSED),
    RAINBOWVCLASSIC => Ok(&webpki::RAINBOWVCLASSIC),
    RAINBOWVCIRCUMZENITHAL => Ok(&webpki::RAINBOWVCIRCUMZENITHAL),
    RAINBOWVCOMPRESSED => Ok(&webpki::RAINBOWVCOMPRESSED),
    SPHINCSHARAKA128FSIMPLE => Ok(&webpki::SPHINCSHARAKA128FSIMPLE),
    SPHINCSHARAKA128FROBUST => Ok(&webpki::SPHINCSHARAKA128FROBUST),
    SPHINCSHARAKA128SSIMPLE => Ok(&webpki::SPHINCSHARAKA128SSIMPLE),
    SPHINCSHARAKA128SROBUST => Ok(&webpki::SPHINCSHARAKA128SROBUST),
    SPHINCSHARAKA192FSIMPLE => Ok(&webpki::SPHINCSHARAKA192FSIMPLE),
    SPHINCSHARAKA192FROBUST => Ok(&webpki::SPHINCSHARAKA192FROBUST),
    SPHINCSHARAKA192SSIMPLE => Ok(&webpki::SPHINCSHARAKA192SSIMPLE),
    SPHINCSHARAKA192SROBUST => Ok(&webpki::SPHINCSHARAKA192SROBUST),
    SPHINCSHARAKA256FSIMPLE => Ok(&webpki::SPHINCSHARAKA256FSIMPLE),
    SPHINCSHARAKA256FROBUST => Ok(&webpki::SPHINCSHARAKA256FROBUST),
    SPHINCSHARAKA256SSIMPLE => Ok(&webpki::SPHINCSHARAKA256SSIMPLE),
    SPHINCSHARAKA256SROBUST => Ok(&webpki::SPHINCSHARAKA256SROBUST),
    SPHINCSSHA256128FSIMPLE => Ok(&webpki::SPHINCSSHA256128FSIMPLE),
    SPHINCSSHA256128FROBUST => Ok(&webpki::SPHINCSSHA256128FROBUST),
    SPHINCSSHA256128SSIMPLE => Ok(&webpki::SPHINCSSHA256128SSIMPLE),
    SPHINCSSHA256128SROBUST => Ok(&webpki::SPHINCSSHA256128SROBUST),
    SPHINCSSHA256192FSIMPLE => Ok(&webpki::SPHINCSSHA256192FSIMPLE),
    SPHINCSSHA256192FROBUST => Ok(&webpki::SPHINCSSHA256192FROBUST),
    SPHINCSSHA256192SSIMPLE => Ok(&webpki::SPHINCSSHA256192SSIMPLE),
    SPHINCSSHA256192SROBUST => Ok(&webpki::SPHINCSSHA256192SROBUST),
    SPHINCSSHA256256FSIMPLE => Ok(&webpki::SPHINCSSHA256256FSIMPLE),
    SPHINCSSHA256256FROBUST => Ok(&webpki::SPHINCSSHA256256FROBUST),
    SPHINCSSHA256256SSIMPLE => Ok(&webpki::SPHINCSSHA256256SSIMPLE),
    SPHINCSSHA256256SROBUST => Ok(&webpki::SPHINCSSHA256256SROBUST),
    SPHINCSSHAKE256128FSIMPLE => Ok(&webpki::SPHINCSSHAKE256128FSIMPLE),
    SPHINCSSHAKE256128FROBUST => Ok(&webpki::SPHINCSSHAKE256128FROBUST),
    SPHINCSSHAKE256128SSIMPLE => Ok(&webpki::SPHINCSSHAKE256128SSIMPLE),
    SPHINCSSHAKE256128SROBUST => Ok(&webpki::SPHINCSSHAKE256128SROBUST),
    SPHINCSSHAKE256192FSIMPLE => Ok(&webpki::SPHINCSSHAKE256192FSIMPLE),
    SPHINCSSHAKE256192FROBUST => Ok(&webpki::SPHINCSSHAKE256192FROBUST),
    SPHINCSSHAKE256192SSIMPLE => Ok(&webpki::SPHINCSSHAKE256192SSIMPLE),
    SPHINCSSHAKE256192SROBUST => Ok(&webpki::SPHINCSSHAKE256192SROBUST),
    SPHINCSSHAKE256256FSIMPLE => Ok(&webpki::SPHINCSSHAKE256256FSIMPLE),
    SPHINCSSHAKE256256FROBUST => Ok(&webpki::SPHINCSSHAKE256256FROBUST),
    SPHINCSSHAKE256256SSIMPLE => Ok(&webpki::SPHINCSSHAKE256256SSIMPLE),
    SPHINCSSHAKE256256SROBUST => Ok(&webpki::SPHINCSSHAKE256256SROBUST),
    XMSS => Ok(&webpki::XMSS),

    _ => {
        let error_msg = format!("received unsupported sig scheme {:?}", scheme);
        Err(TLSError::PeerMisbehavedError(error_msg))
    }
}