<?php
$private = openssl_pkey_new(["private_key_type"=>OPENSSL_KEYTYPE_DH]); $peer = openssl_pkey_new(["private_key_type"=>OPENSSL_KEYTYPE_DH]); openssl_pkey_derive($peer,$private);
