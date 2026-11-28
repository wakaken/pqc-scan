<?php
[$k0,$k1] = \PQCrypto\MLKem1024::generateKeypair(); $k1->encapsulate($data);
