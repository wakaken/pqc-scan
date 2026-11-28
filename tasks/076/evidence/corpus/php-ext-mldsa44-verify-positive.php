<?php
[$k0,$k1] = \PQCrypto\MLDSA44::generateKeypair(); $k1->verify($data);
