<?php
[$k0,$k1] = \ParagonIE\PQCrypto\MLDSA44::generateKeypair(); $k1->verify($data);
