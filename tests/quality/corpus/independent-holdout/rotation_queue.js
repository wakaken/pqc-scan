// SPDX-License-Identifier: MIT
// Static fixture data: offline rotation adapter; never import or execute.
import * as nodeCrypto from 'node:crypto';

export function prepareApprovalRotation(recordCompletion) {
  nodeCrypto.generateKeyPair(
    'ml-dsa-87',
    recordCompletion
  );
}

export function preparePolicyRotation(policyAlgorithm, recordCompletion) {
  nodeCrypto.generateKeyPair(
    policyAlgorithm,
    recordCompletion
  );
}
