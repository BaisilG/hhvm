<?php <<__EntryPoint>> function main() {
$i = 0;
$a = array();
$b = array();
while ($i++ < 1000) {
    $a[] = $i;
    $b[] = (string)$i;
}
$s1 = array_sum($a);
$s2 = array_sum($b);
var_dump($s1, $s2);

$j = 0;
$c = array();
$d = array();
while ($j++ < 100000) {
    $c[] = $j;
    $d[] = (string) $j;
}
$s3 = array_sum($c);
$s4 = array_sum($d);
var_dump($s3, $s4);
}
