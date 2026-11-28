<?php
[$k0,$k1] = \ParagonIE\PQCrypto\MLDSA87::generateKeypair(); $k1->verify($data);
