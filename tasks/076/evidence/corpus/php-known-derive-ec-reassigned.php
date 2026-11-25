<?php
$private = openssl_pkey_new(["private_key_type"=>OPENSSL_KEYTYPE_EC]); $peer = openssl_pkey_new(["private_key_type"=>OPENSSL_KEYTYPE_EC]); $private=$other; openssl_pkey_derive($peer,$private);
