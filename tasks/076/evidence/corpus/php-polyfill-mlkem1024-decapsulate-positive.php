<?php
[$k0,$k1] = \ParagonIE\PQCrypto\MLKem1024::generateKeypair(); $k0->decapsulate($data);
