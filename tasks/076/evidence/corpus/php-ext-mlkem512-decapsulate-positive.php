<?php
[$k0,$k1] = \PQCrypto\MLKem512::generateKeypair(); $k0->decapsulate($data);
