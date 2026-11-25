<?php
[$k0,$k1] = \PQCrypto\MLDSA65::generateKeypair(); $k0->sign($data);
