# pqcrypto_compat

[![Build Status](https://github.com/paragonie/pqcrypto_compat/actions/workflows/ci.yml/badge.svg)](https://github.com/paragonie/pqcrypto_compat/actions)
[![Latest Stable Version](https://poser.pugx.org/paragonie/pqcrypto_compat/v/stable)](https://packagist.org/packages/paragonie/pqcrypto_compat)
[![License](https://poser.pugx.org/paragonie/pqcrypto_compat/license)](https://packagist.org/packages/paragonie/pqcrypto_compat)
[![Downloads](https://img.shields.io/packagist/dt/paragonie/pqcrypto_compat.svg)](https://packagist.org/packages/paragonie/pqcrypto_compat)

pqcrypto_compat defers to [the pqcrypto extension](https://github.com/paragonie/ext-pqcrypto) if it's available, and
provides a polyfill for environments where it is not available, ensuring the PHP ecosystem can effectively migrate to
use post-quantum secure cryptographic algorithms.

> [!WARNING]
> 
> This code has never been independently audited. Use at your own risk.

## Installing

```shell
composer require paragonie/pqcrypto_compat
```

Optional, but recommended: Install [the pqcrypto extension](https://github.com/paragonie/ext-pqcrypto).

## Usage

The recommended way to use this polyfill library is the Compat class.

> [!TIP]
> If you aren't already using opcache and JIT, we **highly** recommend enabling those PHP features
> when using pqcrypto_compat without ext-pqcrypto. It makes a huge difference.

### X-Wing Example

X-Wing is a hybrid KEM combining X25519 and ML-KEM-768. The X25519 implementation is provided by 
[sodium_compat](https://github.com/paragonie/sodium_compat).

```php
<?php
declare(strict_types=1);
use ParagonIE\PQCrypto\Compat;

// Key generation
[$decapsKey, $encapsKey] = Compat::xwing_keygen();

// Encapsulation
['sharedKey' => $ss, 'ciphertext' => $ct] = Compat::xwing_encaps($encapsKey);

// Decapsulation
$sharedKey = Compat::xwing_decaps($decapsKey, $ct);
var_dump(hash_equals($ss, $sharedKey)); // bool(true)
```

### ML-KEM-768 Example

```php
<?php
declare(strict_types=1);
use ParagonIE\PQCrypto\Compat;

// Key generation
[$decapsKey, $encapsKey] = Compat::mlkem768_keygen();
$decapsKeyBytes = $decapsKey->bytes();
$encapsKeyBytes = $encapsKey->bytes();

// Encapsulation
['sharedKey' => $ss, 'ciphertext' => $ct] = Compat::mlkem768_encaps($encapsKey);
// Send $ct to recipient that possesses $decapsKey

$sharedKey = Compat::mlkem768_decaps($decapsKey, $ct);
var_dump(hash_equals($ss, $sharedKey)); // bool(true)
```

### ML-KEM-1024 Example

```php
<?php
declare(strict_types=1);
use ParagonIE\PQCrypto\Compat;

// Key generation
[$decapsKey, $encapsKey] = Compat::mlkem1024_keygen();
$decapsKeyBytes = $decapsKey->bytes();
$encapsKeyBytes = $encapsKey->bytes();

// Encapsulation
['sharedKey' => $ss, 'ciphertext' => $ct] = Compat::mlkem1024_encaps($encapsKey);
// Send $ct to recipient that possesses $decapsKey

$sharedKey = Compat::mlkem768_decaps($decapsKey, $ct);
var_dump(hash_equals($ss, $sharedKey)); // bool(true)
```

### ML-DSA-44 Example

```php
<?php
declare(strict_types=1);
use ParagonIE\PQCrypto\Compat;

// Key generation
['signingKey' => $sk, 'verificationKey' => $vk] = Compat::mldsa44_keygen();

// Signing
$message = 'message';
$signature = Compat::mldsa44_sign($sk, $message);
$valid = Compat::mldsa44_verify($vk, $signature, $message);
var_dump($valid); // bool(true)
```

### ML-DSA-65 Example

```php
<?php
declare(strict_types=1);
use ParagonIE\PQCrypto\Compat;

// Key generation
['signingKey' => $sk, 'verificationKey' => $vk] = Compat::mldsa65_keygen();

// Signing
$message = 'message';
$signature = Compat::mldsa65_sign($sk, $message);
$valid = Compat::mldsa65_verify($vk, $signature, $message);
var_dump($valid); // bool(true)
```

### ML-DSA-87 Example

```php
<?php
declare(strict_types=1);
use ParagonIE\PQCrypto\Compat;

// Key generation
['signingKey' => $sk, 'verificationKey' => $vk] = Compat::mldsa87_keygen();

// Signing
$message = 'message';
$signature = Compat::mldsa87_sign($sk, $message);
$valid = Compat::mldsa87_verify($vk, $signature, $message);
var_dump($valid); // bool(true)
```

### Other Algorithms

We also include ML-KEM-512 for completeness, but do not recommend its usage.
