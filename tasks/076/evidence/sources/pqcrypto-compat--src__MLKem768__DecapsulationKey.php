<?php
declare(strict_types=1);
namespace ParagonIE\PQCrypto\MLKem768;

use ParagonIE\PQCrypto\DecapsKeyInterface;
use ParagonIE\PQCrypto\Exception\MLKemInternalException;
use ParagonIE\PQCrypto\MLKem768;
use ParagonIE\PQCrypto\Traits\SensitiveDebugInfoTrait;
use SensitiveParameter;

class DecapsulationKey implements DecapsKeyInterface
{
    use SensitiveDebugInfoTrait;

    public function __construct(
        #[SensitiveParameter]
        private string $key
    ) {}

    public function bytes(): string
    {
        return $this->key;
    }

    /**
     * @throws MLKemInternalException
     */
    public function decapsulate(string $ciphertext): string
    {
        return MLKem768::decapsulate($this, $ciphertext);
    }
}
