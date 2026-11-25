<?php
[$k0,$k1] = \ParagonIE\PQCrypto\MLKem512::generateKeypair(); $k0->decapsulate($data);
