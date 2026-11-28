<?php
[$k0,$k1] = \PQCrypto\MLDSA87::generateKeypair(); $k0->sign($data);
