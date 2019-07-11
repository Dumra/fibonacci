<?php
function Fibonacci($number)
{
    if ($number < 2) {
        return $number;
    } else {
        return (Fibonacci($number - 1) +
            Fibonacci($number - 2));
    }
}

$number = 46;
for ($counter = 0; $counter < $number; $counter++) {
    echo Fibonacci($counter), ' ';
}
