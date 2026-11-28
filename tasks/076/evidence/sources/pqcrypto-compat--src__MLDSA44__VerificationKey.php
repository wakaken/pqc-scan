<?php
declare(strict_types=1);
namespace ParagonIE\PQCrypto\MLDSA44;

use ParagonIE\PQCrypto\Exception\MLDSAInternalException;
use ParagonIE\PQCrypto\Internal\MLDSA\InternalVerificationKey;
use ParagonIE\PQCrypto\Internal\MLDSA\Operations;
use ParagonIE\PQCrypto\Internal\MLDSA\Params;
use ParagonIE\PQCrypto\SignatureInterface;
use ParagonIE\PQCrypto\VerificationKeyInterface;

class VerificationKey implements VerificationKeyInterface
{
    const PARAMS = Params::MLDSA44;
    public function __construct(private InternalVerificationKey $vk)
    {}

    public function bytes(): string
    {
        return $this->vk->bytes();
    }

    /**
     * @throws MLDSAInternalException
     */
    public static function fromBytes(string $bytes): self
    {
        return new self(
            InternalVerificationKey::fromBytes(self::PARAMS, $bytes)
        );
    }

    /**
     * @throws MLDSAInternalException
     */
    public function verify(SignatureInterface $signature, string $message, string $ctx = ''): bool
    {
        if (!($signature instanceof Signature)) {
            throw new MLDSAInternalException('invalid signature object');
        }
        $mPrime = Operations::prepareMessage($message, $ctx);
        return $this->vk->verifyInternal($signature->sig, $mPrime);
    }
}
