# main.py -- put your code here!
from machine import Pin
from time import sleep

red = Pin(20, Pin.OUT)
amber = Pin(19, Pin.OUT)
green = Pin(18, Pin.OUT)

pir  = Pin(26, Pin.IN, Pin.PULL_DOWN)


red.value(1)
amber.value(0)
green.value(0)

sleep(10)

red.value(0)
amber.value(1)

while True:
  sleep(0.01)
  if pir.value() == 1:
    amber.value(0)
    green.value(1)
    sleep(5)
  else:
    green.value(0)
    amber.value(1)