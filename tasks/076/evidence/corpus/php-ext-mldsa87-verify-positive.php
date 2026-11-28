<?php
[$k0,$k1] = \PQCrypto\MLDSA87::generateKeypair(); $k1->verify($data);
