<?php
[$k0,$k1] = \PQCrypto\MLDSA65::generateKeypair(); $k1->verify($data);
