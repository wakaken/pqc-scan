<?php
[$k0,$k1] = \ParagonIE\PQCrypto\MLDSA65::generateKeypair(); $k1->verify($data);
