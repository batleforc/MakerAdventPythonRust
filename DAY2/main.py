# main.py -- put your code here!
from machine import Pin
import time

green = Pin(18, Pin.OUT)
amber = Pin(19, Pin.OUT)
red = Pin(20, Pin.OUT)

green.value(1)
amber.value(0)
red.value(1)
truc = 0
while True:
    green.value(1 if truc == 0 else 0)
    amber.value(1 if truc == 1 else 0)
    red.value(1 if truc == 2 else 0)
    truc = (truc + 1) % 3
    time.sleep(1)
