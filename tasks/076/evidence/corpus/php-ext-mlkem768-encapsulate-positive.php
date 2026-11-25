<?php
[$k0,$k1] = \PQCrypto\MLKem768::generateKeypair(); $k1->encapsulate($data);
