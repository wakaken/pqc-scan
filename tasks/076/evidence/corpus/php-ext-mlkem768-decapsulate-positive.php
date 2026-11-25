<?php
[$k0,$k1] = \PQCrypto\MLKem768::generateKeypair(); $k0->decapsulate($data);
