set print pretty on
target remote :3333
layout src
set print pretty on
load
monitor tpiu config internal itm.txt uart off 8000000
break main
continue
