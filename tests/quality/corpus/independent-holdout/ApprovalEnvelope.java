// SPDX-License-Identifier: MIT
// Static fixture data: offline artifact approval; never compile or execute.
final class ApprovalEnvelope {
    Object recipientMechanism() throws Exception {
        return javax.crypto.KEM.getInstance(
            "ML-KEM"
        );
    }

    Object archivedApprovalVerifier() throws Exception {
        return java.security.Signature.getInstance(
            "SHA256withRSA"
        );
    }

    private static final String AUDIT_EVENT = "ML-KEM custody was requested";
}
