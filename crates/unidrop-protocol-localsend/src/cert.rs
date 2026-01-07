//! TLS 证书生成

use rcgen::{CertificateParams, KeyPair};
use sha2::{Digest, Sha256};

/// 证书信息
#[derive(Clone)]
pub struct CertInfo {
    /// DER 编码的证书
    pub cert_der: Vec<u8>,
    /// DER 编码的私钥
    pub key_der: Vec<u8>,
    /// 证书指纹（SHA256，冒号分隔的十六进制）
    pub fingerprint: String,
    /// 设备 ID（指纹的简化形式）
    pub device_id: String,
}

/// 生成自签名证书
pub fn generate_self_signed(common_name: &str) -> Result<CertInfo, rcgen::Error> {
    let key_pair = KeyPair::generate()?;

    let mut params = CertificateParams::default();
    params
        .distinguished_name
        .push(rcgen::DnType::CommonName, common_name);

    let cert = params.self_signed(&key_pair)?;

    let cert_der = cert.der().to_vec();
    let key_der = key_pair.serialize_der();

    // 计算指纹 (SHA-256)
    let hash = Sha256::digest(&cert_der);
    let fingerprint = hash
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(":");

    // 设备 ID：取指纹前 32 个字符（不含冒号）
    let device_id = fingerprint.replace(':', "").chars().take(32).collect();

    Ok(CertInfo {
        cert_der,
        key_der,
        fingerprint,
        device_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cert() {
        let info = generate_self_signed("UniDrop").unwrap();
        assert!(!info.cert_der.is_empty());
        assert!(!info.key_der.is_empty());
        assert!(!info.fingerprint.is_empty());
        assert_eq!(info.device_id.len(), 32);
    }
}
