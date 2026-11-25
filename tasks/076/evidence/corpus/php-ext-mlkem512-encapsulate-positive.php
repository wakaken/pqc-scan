<?php
[$k0,$k1] = \PQCrypto\MLKem512::generateKeypair(); $k1->encapsulate($data);
