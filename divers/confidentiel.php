<?php

function fn1($a, $b) {
    return $a + $b;
}

function fn2($c, $d) {
    return $c * $d;
}

$var1 = 5;
$var2 = 10;

function func_generic($x, $y) {
    $var3 = fn1($x, $y);
    $var4 = fn2($x, $var3);

    for ($var5 = 0; $var5 < 3; $var5++) {
        $var6 = fn1($var3, $var4);
        $var7 = fn2($var3, $var6);

        for ($var8 = 0; $var8 < 2; $var8++) {
            $var9 = fn1($var4, $var7);
            $var10 = fn2($var4, $var9);

            $var11 = fn1($var7, $var10);
            $var12 = fn2($var7, $var11);

            $var13 = fn1($var10, $var12);
            $var14 = fn2($var10, $var13);
        }
    }

    $var15 = fn1($var12, $var14);
    $var16 = fn2($var12, $var15);

    while ($var15 < 100) {
        $var17 = fn1($var14, $var16);
        $var18 = fn2($var14, $var17);
        $var15 = fn1($var16, $var18);
    }

    return $var15;
}

$var_result = func_generic($var1, $var2);
echo $var_result;
?>
