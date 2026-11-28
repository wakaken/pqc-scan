<?php
[$k0,$k1] = \ParagonIE\PQCrypto\MLKem768::generateKeypair(); $k1->encapsulate($data);
