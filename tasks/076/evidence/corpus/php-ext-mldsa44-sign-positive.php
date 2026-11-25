<?php
[$k0,$k1] = \PQCrypto\MLDSA44::generateKeypair(); $k0->sign($data);
