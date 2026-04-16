# 📜 Blockchain Certificate Verification System

## 📖 Overview

This project is a **smart contract-based certificate verification system** built using the Soroban SDK. It allows institutions to issue digital certificates securely on the blockchain and enables anyone to verify their authenticity.

Instead of storing the entire certificate file, the system stores a **cryptographic hash** of the certificate, ensuring:

* Data integrity
* Tamper-proof verification
* Decentralized trust

---

## 🎯 Purpose

Traditional certificate systems are vulnerable to:

* Forgery
* Unauthorized duplication
* Difficult verification processes

This application solves these issues by leveraging blockchain technology to create a **trustless and immutable verification system**.

---

## ⚙️ How It Works

1. A certificate file is generated off-chain (PDF, image, etc.)
2. The file is hashed (e.g., SHA-256)
3. The hash is stored on the blockchain via the smart contract
4. Anyone can verify the certificate by comparing hashes

---

## 🧱 Data Structure

```rust
pub struct Certificate {
    id: u64,
    owner: Address,
    issuer: Address,
    hash: String,
    timestamp: u64,
}
```

---

## 🚀 Features

### ✅ 1. Issue Certificate

* Authorized users can create certificates
* Each certificate is uniquely identified
* Includes owner and issuer information

### 🔍 2. Verify Certificate

* Anyone can verify a certificate using its hash
* Returns validation result (VALID / INVALID)

### 🗂️ 3. Retrieve Certificates

* Fetch all stored certificates from the blockchain

### 🔐 4. Secure Ownership

* Each certificate is linked to:

  * **Owner (recipient)**
  * **Issuer (creator)**

### ⏱️ 5. Timestamping

* Records the exact time of issuance
* Ensures chronological integrity

---

## 🔒 Security Design

This system implements several security principles:

* **Immutability**: Data cannot be altered once stored
* **Hash-based verification**: Prevents tampering
* **Ownership tracking**: Ensures accountability
* **Decentralization**: No single point of failure

---

## 🧪 Example Use Cases

* 🎓 Academic Certificates (Diplomas, Transcripts)
* 📜 Professional Certifications
* 🏆 Training Completion Certificates
* 📄 Legal Documents Verification

---

## 📦 Future Improvements

* 🔐 Role-based access control (admin/issuer)
* ❌ Certificate revocation feature
* 📡 Event logging for audit trails
* 🌐 Frontend integration (web dashboard)
* 📁 File upload + automatic hashing

---

## 🛠️ Tech Stack

* Rust (no_std)
* Soroban SDK
* Stellar Smart Contract Platform

---

## 📌 Conclusion

This project demonstrates how blockchain can be used to build a **secure, transparent, and reliable certificate verification system**, eliminating the need for centralized validation authorities.

---


## 👨‍💻 Author

Developed as a smart contract security-focused project.

---

<img width="1916" height="857" alt="image" src="https://github.com/user-attachments/assets/fb3b72eb-b6b7-46fc-89e5-b1c91add15b7" />
<img width="1487" height="840" alt="image" src="https://github.com/user-attachments/assets/0b9382c3-f271-4590-92f1-c0fab0ce03b5" />


CDS7ARZPYPT3ALF7JSDXRAPWFVRENK4RL2MJVZU3SB54IKZYNQYWUI54
