<?php
declare(strict_types=1);
namespace ParagonIE\PQCrypto;

use ParagonIE\PQCrypto\Exception\MLKemInternalException;
use ParagonIE\PQCrypto\Exception\PQCryptoCompatException;
use ParagonIE\PQCrypto\Internal\Keccak;
use ParagonIE\PQCrypto\Internal\MLKem\Operations;
use ParagonIE\PQCrypto\XWing\DecapsulationKey;
use ParagonIE\PQCrypto\XWing\EncapsulationKey;
use Random\RandomException;
use SensitiveParameter;
use SodiumException;

abstract class XWing
{
    public const SEED_SIZE = 32;
    public const ENCAPSULATION_KEY_SIZE = 1216;
    public const CIPHERTEXT_SIZE = 1120;
    private const MLKEM768_EK_SIZE = 1184;
    private const MLKEM768_CT_SIZE = 1088;
    private const MLKEM768_K = 3;
    private const MLKEM768_ETA1 = 2;
    private const MLKEM768_DU = 10;
    private const MLKEM768_DV = 4;
    private const XWING_LABEL = "\x5c\x2e\x2f\x2f\x5e\x5c";

    /**
     * @return array{0: DecapsulationKey, 1: EncapsulationKey}
     *
     * @throws MLKemInternalException
     * @throws RandomException
     * @throws SodiumException
     */
    public static function generateKeypair(): array
    {
        $seed = random_bytes(32);
        return self::generateKeypairFromSeed($seed);
    }

    /**
     * @return array{0: DecapsulationKey, 1: EncapsulationKey}
     *
     * @throws MLKemInternalException
     * @throws PQCryptoCompatException
     * @throws SodiumException
     */
    public static function generateKeypairFromSeed(
        #[SensitiveParameter]
        string $seed
    ): array {
        if (\strlen($seed) !== self::SEED_SIZE) {
            throw new PQCryptoCompatException(
                'Seed must be ' . self::SEED_SIZE . ' bytes'
            );
        }
        [, , $pkM, $pkX] = self::expandDecapsulationKey($seed);
        return [
            new DecapsulationKey($seed),
            new EncapsulationKey($pkM . $pkX),
        ];
    }

    /**
     * @return array{sharedKey: string, ciphertext: string}
     *
     * @throws MLKemInternalException
     * @throws PQCryptoCompatException
     * @throws RandomException
     * @throws SodiumException
     */
    public static function encapsulate(EncapsKeyInterface $ek): array
    {
        $ekBytes = $ek->bytes();
        if (strlen($ekBytes) !== self::ENCAPSULATION_KEY_SIZE) {
            throw new PQCryptoCompatException('Invalid X-Wing encapsulation key length');
        }
        return self::encapsulateDerandomized($ekBytes, random_bytes(64));
    }

    /**
     * SECURITY WARNING: Do not use this outside of unit testing.
     *
     * @return array{sharedKey: string, ciphertext: string}
     *
     * @throws MLKemInternalException
     * @throws SodiumException
     */
    public static function encapsulateDerandomized(string $pk, #[SensitiveParameter] string $eSeed): array
    {
        $pkM = substr($pk, 0, self::MLKEM768_EK_SIZE);
        $pkX = substr($pk, self::MLKEM768_EK_SIZE, 32);

        $ekX = substr($eSeed, 32, 32);
        $ctX = sodium_crypto_scalarmult_base($ekX);
        $ssX = sodium_crypto_scalarmult($ekX, $pkX);

        $m = substr($eSeed, 0, 32);
        $ekParsed = Operations::parseEncapsulationKey(self::MLKEM768_K, $pkM);
        $kemResult = Operations::kemEncaps(
            self::MLKEM768_K,
            self::MLKEM768_ETA1,
            self::MLKEM768_DU,
            self::MLKEM768_DV,
            $ekParsed,
            $m
        );
        $ssM = $kemResult['sharedKey'];
        $ctM = $kemResult['ciphertext'];

        $ss = self::combiner($ssM, $ssX, $ctX, $pkX);
        return [
            'sharedKey' => $ss,
            'ciphertext' => $ctM . $ctX,
        ];
    }

    /**
     * @throws MLKemInternalException
     * @throws PQCryptoCompatException
     * @throws SodiumException
     */
    public static function decapsulate(
        #[SensitiveParameter]
        DecapsKeyInterface $dk,
        string $ciphertext
    ): string {
        $seed = $dk->bytes();
        if (strlen($seed) !== self::SEED_SIZE) {
            throw new PQCryptoCompatException(
                'Invalid X-Wing decapsulation key length'
            );
        }
        if (strlen($ciphertext) !== self::CIPHERTEXT_SIZE) {
            throw new PQCryptoCompatException(
                'Invalid X-Wing ciphertext length'
            );
        }

        $ctM = substr($ciphertext, 0, self::MLKEM768_CT_SIZE);
        $ctX = substr($ciphertext, self::MLKEM768_CT_SIZE, 32);

        [$skM, $skX, , $pkX] = self::expandDecapsulationKey($seed);

        // ML-KEM-768 decapsulate
        $d = substr($skM, 0, 32);
        $z = substr($skM, 32, 32);
        $key = Operations::kemKeyGen(
            self::MLKEM768_K,
            self::MLKEM768_ETA1,
            $d,
            $z
        );
        $ssM = Operations::kemDecaps(
            self::MLKEM768_K,
            self::MLKEM768_ETA1,
            self::MLKEM768_DU,
            self::MLKEM768_DV,
            $z,
            $key['h'],
            ['t' => $key['t'], 'a' => $key['a']],
            $key['s'],
            $ctM
        );

        $ssX = sodium_crypto_scalarmult($skX, $ctX);
        return self::combiner($ssM, $ssX, $ctX, $pkX);
    }

    /**
     * @return string[]
     * @throws MLKemInternalException
     * @throws SodiumException
     */
    private static function expandDecapsulationKey(#[SensitiveParameter] string $seed): array
    {
        $expanded = Keccak::shake256()
            ->absorb($seed)
            ->squeeze(96);

        $d = substr($expanded, 0, 32);
        $z = substr($expanded, 32, 32);
        $skX = substr($expanded, 64, 32);

        // ML-KEM-768 keygen from d, z
        $key = Operations::kemKeyGen(
            self::MLKEM768_K,
            self::MLKEM768_ETA1,
            $d,
            $z
        );
        $pkM = $key['encapsulationKeyBytes'];
        $skM = $d . $z;

        $pkX = sodium_crypto_scalarmult_base($skX);

        return [$skM, $skX, $pkM, $pkX];
    }

    private static function combiner(
        string $ssM,
        string $ssX,
        string $ctX,
        string $pkX
    ): string {
        return hash(
            'sha3-256',
            $ssM . $ssX . $ctX . $pkX . self::XWING_LABEL,
            true
        );
    }
}
