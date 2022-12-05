# main.py -- put your code here!
from machine import ADC, Pin
import time

green = Pin(18, Pin.OUT)
amber = Pin(19, Pin.OUT)
red = Pin(20, Pin.OUT)

potentiometer = ADC(Pin(27))

mydelay = 0

while True:
  mydelay = potentiometer.read_u16() / 65000

  red.value(1)
  time.sleep(mydelay)
  red.value(0)
  time.sleep(mydelay)