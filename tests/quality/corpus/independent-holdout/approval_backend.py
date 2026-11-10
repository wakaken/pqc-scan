# SPDX-License-Identifier: MIT
# Static fixture data: offline approval backend; never import or execute.
from cryptography.hazmat.primitives.asymmetric.mldsa import (
    MLDSA87PrivateKey,
)

class ApprovalBackend:
    algorithm_identifier = "artifact-approval-v3"
    artifact_kind = "detached-approval"
