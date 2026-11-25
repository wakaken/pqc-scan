<?php
declare(strict_types=1);
namespace ParagonIE\PQCrypto\MLDSA44;

use ParagonIE\PQCrypto\Exception\MLDSAInternalException;
use ParagonIE\PQCrypto\Internal\MLDSA\InternalSigningKey;
use ParagonIE\PQCrypto\Internal\MLDSA\Operations;
use ParagonIE\PQCrypto\Internal\MLDSA\Params;
use ParagonIE\PQCrypto\SignatureInterface;
use ParagonIE\PQCrypto\SigningKeyInterface;
use ParagonIE\PQCrypto\Traits\SensitiveDebugInfoTrait;
use ParagonIE\PQCrypto\VerificationKeyInterface;
use Random\RandomException;

class SigningKey implements SigningKeyInterface
{
    use SensitiveDebugInfoTrait;

    const PARAMS = Params::MLDSA44;
    protected function __construct(private InternalSigningKey $sk)
    {}

    public function bytes(): string
    {
        return $this->sk->bytes();
    }

    /**
     * @throws MLDSAInternalException
     */
    public static function fromBytes(#[\SensitiveParameter] string $bytes): self
    {
        return new self(
            InternalSigningKey::keyGenInternal(self::PARAMS, $bytes)
        );
    }

    /**
     * @param string $message
     * @param string $ctx
     * @return SignatureInterface
     * @throws MLDSAInternalException
     * @throws RandomException
     */
    public function sign(string $message, string $ctx = ''): SignatureInterface
    {
        $mPrime = Operations::prepareMessage($message, $ctx);
        $rnd = random_bytes(32);
        return new Signature($this->sk->signInternal($mPrime, $rnd));
    }

    public function getVerificationKey(): VerificationKeyInterface
    {
        return new VerificationKey($this->sk->vk);
    }
}
